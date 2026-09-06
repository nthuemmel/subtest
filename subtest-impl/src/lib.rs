mod attribute_parser;
mod config;
mod inheritance;
mod unused_variables;

use crate::attribute_parser::{check_for_misplaced_subtests, has_test_attr, remove_subtest_attrs};
use crate::inheritance::InheritableFunctionAspects;
use attribute_parser::RemovedSubtestAttrs;
use config::{MacroConfig, SubtestConfig};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Item, ItemFn, Stmt, parse_quote};

pub fn expand_subtest_main_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    expand_subtest_main_fn_fallible(args, input).unwrap_or_else(|err| err.to_compile_error())
}

fn expand_subtest_main_fn_fallible(
    args: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, syn::Error> {
    let macro_config = MacroConfig::parse(&args)?;
    let input_fn: ItemFn = syn::parse2(input)?;

    let main_subtest = Subtest::new(
        &macro_config,
        &SubtestConfig::default(),
        input_fn,
        &InheritableFunctionAspects::none(),
    )?;

    Ok(main_subtest.render())
}

struct Subtest {
    function: ItemFn,
    subtests: Vec<Subtest>,
}

impl Subtest {
    fn new(
        macro_config: &MacroConfig,
        subtest_config: &SubtestConfig,
        mut function: ItemFn,
        inheritable_from_parent: &InheritableFunctionAspects,
    ) -> Result<Self, syn::Error> {
        let function_statements = std::mem::take(&mut function.block.stmts);

        let mut inheritable_from_function =
            inheritable_from_parent.apply(&mut function, subtest_config);

        if !has_test_attr(&function) && !macro_config.allow_missing_test_attr {
            return Err(syn::Error::new_spanned(
                function.sig.ident,
                "function is missing a test attribute, such as #[test], #[tokio::test] or #[rstest]\n\
                 add one below #[subtest] - attributes written above it are not visible to this macro\n\
                 if this function is meant to be a nested subtest, add #[subtest] to the enclosing test function instead",
            ));
        }

        let mut subtests = Vec::new();
        let mut leading_items_after_statements = !function.block.stmts.is_empty();

        for statement in function_statements {
            let statement = match statement {
                Stmt::Item(Item::Fn(nested_fn)) => {
                    match remove_subtest_attrs(nested_fn)? {
                        RemovedSubtestAttrs::RemovedSubtest {
                            subtest_config,
                            cleaned_function,
                        } => {
                            subtests.push(Subtest::new(
                                macro_config,
                                &subtest_config,
                                cleaned_function,
                                &inheritable_from_function,
                            )?);
                            continue;
                        }

                        RemovedSubtestAttrs::NoSubtest(nested_fn) => {
                            // A function carrying a test attribute - such as #[test] -
                            // is most likely a subtest whose #[subtest] attribute was forgotten
                            if has_test_attr(&nested_fn) {
                                return Err(syn::Error::new_spanned(
                                    nested_fn.sig.ident,
                                    "function is missing the #[subtest] attribute",
                                ));
                            }

                            // Any remaining function is a helper function, which is treated like
                            // any other statement: kept in the test fn and copied into the subtests
                            // following it
                            Stmt::Item(Item::Fn(nested_fn))
                        }
                    }
                }

                // non-function statements
                other => other,
            };

            check_for_misplaced_subtests(&statement)?;

            let statement = match statement {
                Stmt::Item(item) if leading_items_after_statements => {
                    // Items that are at the top of a nested subtests, but after statements
                    // inherited from the parent, would trigger a 'clippy::items_after_statements'
                    // false positive. Mask them.
                    // Note that this does not work in a crate which sets `#![forbid(clippy::items_after_statements)]`,
                    // because `forbid` rejects any `allow` below it. We'll think about how to solve this once a user
                    // complains.
                    parse_quote!(#[allow(clippy::items_after_statements)] #item)
                }

                other => {
                    leading_items_after_statements = false;
                    other
                }
            };

            inheritable_from_function.add_statement(statement.clone());
            function.block.stmts.push(statement);
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
