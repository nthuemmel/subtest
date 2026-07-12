use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Block, Item, ItemFn, Stmt, parse_quote};

pub fn expand_subtest_main_fn(input: TokenStream) -> TokenStream {
    expand_subtest_main_fn_fallible(input).unwrap_or_else(|err| err.to_compile_error())
}

fn expand_subtest_main_fn_fallible(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input_fn: ItemFn = syn::parse2(input)?;

    // This is the test attribute that will be added if a user just specifies #[subtest] and nothing
    // else
    let default_test_attr: Attribute = parse_quote!(#[test]);
    let default_test_attributes = vec![default_test_attr];

    Ok(Subtest::new(input_fn, vec![], &default_test_attributes)?.render())
}

struct Subtest {
    function: ItemFn,
    subtests: Vec<Subtest>,
}

impl Subtest {
    fn new(
        input_fn: ItemFn,
        parent_fn_statements: Vec<Stmt>,
        default_test_fn_attrs: &Vec<Attribute>,
    ) -> Result<Self, syn::Error> {
        // If the subtest fn does not specify any attributes (#[subtest] itself excluded),
        // inherit attributes from the parent test fn
        let attrs = if input_fn.attrs.is_empty() {
            default_test_fn_attrs.clone()
        } else {
            input_fn.attrs
        };

        let mut function = ItemFn {
            attrs,
            vis: input_fn.vis,
            sig: input_fn.sig,
            block: Box::new(Block {
                brace_token: input_fn.block.brace_token,
                // inherit all preceding statements from the parent
                stmts: parent_fn_statements,
            }),
        };

        let mut subtests = Vec::new();

        for statement in input_fn.block.stmts {
            match statement {
                Stmt::Item(Item::Fn(nested_fn)) => {
                    subtests.push(Subtest::new(
                        check_and_remove_subtest_attr(nested_fn)?,
                        function.block.stmts.clone(),
                        &function.attrs,
                    )?);
                }
                other => {
                    function.block.stmts.push(other);
                }
            }
        }

        Ok(Self { function, subtests })
    }

    fn render(self) -> TokenStream {
        let Self { function, subtests } = self;

        let subtest_module = if subtests.is_empty() {
            None
        } else {
            let module_name = format_ident!("{}_subtests", function.sig.ident);
            let rendered_subtests = subtests.into_iter().map(Subtest::render);

            Some(quote! {
                mod #module_name {
                    use super::*;
                    #(#rendered_subtests)*
                }
            })
        };

        quote! {
            #function
            #subtest_module
        }
    }
}

fn check_and_remove_subtest_attr(mut from_fn: ItemFn) -> Result<ItemFn, syn::Error> {
    let mut subtest_attr_found = false;

    from_fn.attrs.retain(|attr| {
        if attr.meta.path().is_ident("subtest") {
            subtest_attr_found = true;
            false
        } else {
            true
        }
    });

    if !subtest_attr_found {
        return Err(syn::Error::new_spanned(
            from_fn,
            "function is missing the #[subtest] attribute",
        ));
    }

    Ok(from_fn)
}
