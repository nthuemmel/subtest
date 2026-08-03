use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{Attribute, Block, FnArg, Item, ItemFn, ReturnType, Signature, Stmt, Token};

pub fn expand_subtest_main_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    expand_subtest_main_fn_fallible(args, input).unwrap_or_else(|err| err.to_compile_error())
}

fn expand_subtest_main_fn_fallible(
    args: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, syn::Error> {
    if !args.is_empty() {
        return Err(syn::Error::new_spanned(args, "expected no arguments"));
    }

    let input_fn: ItemFn = syn::parse2(input)?;

    let main_subtest = Subtest::new(
        input_fn,
        vec![],
        &[],
        &Punctuated::new(),
        &ReturnType::Default,
    )?;

    Ok(main_subtest.render())
}

struct Subtest {
    function: ItemFn,
    subtests: Vec<Subtest>,
}

impl Subtest {
    fn new(
        input_fn: ItemFn,
        parent_fn_statements: Vec<Stmt>,
        parent_fn_attrs: &[Attribute],
        parent_fn_params: &Punctuated<FnArg, Token![,]>,
        parent_fn_return_type: &ReturnType,
    ) -> Result<Self, syn::Error> {
        // If the subtest fn does not specify any overriding attributes (#[subtest] itself,
        // doc comments and lint & configuration attributes excluded),
        // inherit attributes from the parent test fn
        let attrs = if !input_fn.attrs.iter().any(is_overriding_attr) {
            let mut attrs = input_fn.attrs;
            attrs.extend_from_slice(parent_fn_attrs);
            attrs
        } else {
            input_fn.attrs
        };

        // Inherit function parameters if the subtest fn does not specify any
        let fn_params = if input_fn.sig.inputs.is_empty() {
            parent_fn_params.clone()
        } else {
            input_fn.sig.inputs
        };

        // Inherit function return type if the subtest fn does not specify any
        let fn_return_type = if matches!(input_fn.sig.output, ReturnType::Default) {
            parent_fn_return_type.clone()
        } else {
            input_fn.sig.output
        };

        let mut function = ItemFn {
            attrs,
            vis: input_fn.vis,
            sig: Signature {
                inputs: fn_params,
                output: fn_return_type,
                ..input_fn.sig
            },
            block: Box::new(Block {
                brace_token: input_fn.block.brace_token,
                // inherit all preceding statements from the parent
                stmts: parent_fn_statements,
            }),
        };

        let mut subtests = Vec::new();

        // Doc comments describe the function they are written on, so they are not passed down
        let inheritable_attrs: Vec<Attribute> = function
            .attrs
            .iter()
            .filter(|attr| !is_doc_attr(attr))
            .cloned()
            .collect();

        for statement in input_fn.block.stmts {
            match statement {
                Stmt::Item(Item::Fn(nested_fn)) => {
                    subtests.push(Subtest::new(
                        check_and_remove_subtest_attr(nested_fn)?,
                        function.block.stmts.clone(),
                        &inheritable_attrs,
                        &function.sig.inputs,
                        &function.sig.output,
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

/// Whether an attribute of a subtest fn overrides the attributes inherited from the parent test fn.
///
/// Doc comments as well as lint & configuration attributes don't override, they are additive to the
/// inherited attributes
fn is_overriding_attr(attr: &Attribute) -> bool {
    const NON_OVERRIDING_ATTRS: &[&str] = &[
        "doc", "allow", "expect", "warn", "deny", "forbid", "cfg", "cfg_attr",
    ];

    let path = attr.meta.path();

    let is_non_overriding = NON_OVERRIDING_ATTRS.iter().any(|name| path.is_ident(name))
        // tool attributes such as #[rustfmt::skip]
        || path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "rustfmt");

    !is_non_overriding
}

/// Whether an attribute is a doc comment (or an equivalent `#[doc = "..."]` attribute)
fn is_doc_attr(attr: &Attribute) -> bool {
    attr.meta.path().is_ident("doc")
}

fn check_and_remove_subtest_attr(mut from_fn: ItemFn) -> Result<ItemFn, syn::Error> {
    let mut subtest_attr_found = false;
    let mut validation_error = None;

    from_fn.attrs.retain(|attr| {
        if attr.meta.path().is_ident("subtest") {
            subtest_attr_found = true;
            if validation_error.is_none() {
                validation_error = attr.meta.require_path_only().err().map(|_| {
                    syn::Error::new_spanned(attr, "expected #[subtest] with no arguments")
                });
            }
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

    if let Some(validation_error) = validation_error {
        return Err(validation_error);
    }

    Ok(from_fn)
}
