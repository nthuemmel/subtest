use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
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
                Stmt::Item(Item::Fn(nested_fn)) if has_subtest_attr(&nested_fn) => {
                    subtests.push(Subtest::new(
                        remove_subtest_attrs(nested_fn)?,
                        function.block.stmts.clone(),
                        &inheritable_attrs,
                        &function.sig.inputs,
                        &function.sig.output,
                    )?);
                }

                // A function carrying attributes which would override the inherited ones - such as
                // #[test] - is most likely a subtest whose #[subtest] attribute was forgotten
                Stmt::Item(Item::Fn(nested_fn))
                    if nested_fn.attrs.iter().any(is_overriding_attr) =>
                {
                    return Err(syn::Error::new_spanned(
                        nested_fn.sig.ident,
                        "function is missing the #[subtest] attribute",
                    ));
                }

                // Any remaining function is a helper function, which is treated like any other
                // statement: kept in the test fn and copied into the subtests following it
                other => {
                    check_for_misplaced_subtests(&other)?;
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

/// Reject `#[subtest]` functions which are not declared directly in the body of their parent test
/// function.
///
/// Only the statements of a test function's body are searched for subtests, so a `#[subtest]`
/// function nested inside an `if`, a loop, a block, a closure or another item, would
/// keep its `#[subtest]` attribute and be expanded by the compiler as if it were a top-level test
/// function, which is definitely not what we want.
fn check_for_misplaced_subtests(statement: &Stmt) -> Result<(), syn::Error> {
    #[derive(Default)]
    struct MisplacedSubtestVisitor {
        error: Option<syn::Error>,
    }

    impl<'ast> Visit<'ast> for MisplacedSubtestVisitor {
        fn visit_item_fn(&mut self, item_fn: &'ast ItemFn) {
            if let Some(attr) = item_fn.attrs.iter().find(|attr| is_subtest_attr(attr)) {
                if self.error.is_none() {
                    self.error = Some(syn::Error::new_spanned(
                        attr,
                        "#[subtest] functions must be declared directly in the body of their \
                         parent test function, not nested inside a block, an expression or \
                         another item",
                    ));
                }
            }

            visit::visit_item_fn(self, item_fn);
        }
    }

    let mut visitor = MisplacedSubtestVisitor::default();
    visitor.visit_stmt(statement);

    if let Some(error) = visitor.error {
        Err(error)
    } else {
        Ok(())
    }
}

/// Whether an attribute is the `#[subtest]` attribute
fn is_subtest_attr(attr: &Attribute) -> bool {
    attr.meta.path().is_ident("subtest")
}

/// Whether a function is marked as a subtest
fn has_subtest_attr(item_fn: &ItemFn) -> bool {
    item_fn.attrs.iter().any(is_subtest_attr)
}

/// Strip the `#[subtest]` attribute off a subtest fn, validating that it carries no arguments
fn remove_subtest_attrs(mut from_fn: ItemFn) -> Result<ItemFn, syn::Error> {
    let mut validation_error = None;

    from_fn.attrs.retain(|attr| {
        if is_subtest_attr(attr) {
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

    match validation_error {
        Some(validation_error) => Err(validation_error),
        None => Ok(from_fn),
    }
}
