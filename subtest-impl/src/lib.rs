mod attribute_parser;
mod config;

use crate::attribute_parser::{
    check_for_misplaced_subtests, has_test_attr, is_doc_attr, remove_subtest_attrs,
};
use attribute_parser::RemovedSubtestAttrs;
use config::{MacroConfig, SubtestConfig};
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
    let macro_config = MacroConfig::parse(&args)?;
    let input_fn: ItemFn = syn::parse2(input)?;

    let main_subtest = Subtest::new(
        &macro_config,
        &SubtestConfig::default(),
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
        macro_config: &MacroConfig,
        subtest_config: &SubtestConfig,
        input_fn: ItemFn,
        parent_fn_statements: Vec<Stmt>,
        parent_fn_attrs: &[Attribute],
        parent_fn_params: &Punctuated<FnArg, Token![,]>,
        parent_fn_return_type: &ReturnType,
    ) -> Result<Self, syn::Error> {
        let attrs = if subtest_config.inherit_attributes {
            parent_fn_attrs
                .iter()
                .cloned()
                .chain(input_fn.attrs)
                .collect()
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

        if !has_test_attr(&function) && !macro_config.allow_missing_test_attr {
            return Err(syn::Error::new_spanned(
                function.sig.ident,
                "function is missing a test attribute, such as #[test], #[tokio::test] or #[rstest]\n\
                 add one below #[subtest] - attributes written above it are not visible to this macro\n\
                 if this function is meant to be a nested subtest, add #[subtest] to the enclosing test function instead",
            ));
        }

        let mut subtests = Vec::new();

        // Doc comments describe the function they are written on, so they are not passed down
        let inheritable_attrs: Vec<Attribute> = function
            .attrs
            .iter()
            .filter(|attr| !is_doc_attr(attr))
            .cloned()
            .collect();

        for statement in input_fn.block.stmts {
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
                                function.block.stmts.clone(),
                                &inheritable_attrs,
                                &function.sig.inputs,
                                &function.sig.output,
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
