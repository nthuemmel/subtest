use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{
    Attribute, Block, Expr, ExprLit, FnArg, Item, ItemFn, Lit, Meta, MetaNameValue, ReturnType,
    Signature, Stmt, Token,
};

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

struct MacroConfig {
    allow_missing_test_attr: bool,
}

impl MacroConfig {
    fn parse(args: &TokenStream) -> Result<Self, syn::Error> {
        let allow_missing_test_attr_string = "allow_missing_test_attribute";

        if syn::parse2::<Ident>(args.clone())
            .ok()
            .map(|ident| ident.to_string())
            .as_deref()
            == Some(allow_missing_test_attr_string)
        {
            Ok(Self {
                allow_missing_test_attr: true,
            })
        } else if args.is_empty() {
            Ok(Self {
                allow_missing_test_attr: false,
            })
        } else {
            Err(syn::Error::new_spanned(
                args,
                format!("expected either {allow_missing_test_attr_string} or no arguments"),
            ))
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
struct SubtestConfig {
    inherit_attributes: bool,
}

impl Default for SubtestConfig {
    fn default() -> Self {
        Self {
            inherit_attributes: true,
        }
    }
}

impl SubtestConfig {
    fn parse(args: &TokenStream) -> Result<Self, syn::Error> {
        if args.is_empty() {
            return Ok(Self::default());
        }

        let inherit_attributes_ident = "inherit_attributes";

        let key_value_pair: MetaNameValue = syn::parse2(args.clone())
            .map_err(|_| syn::Error::new_spanned(args, "expected '<key> = <value>' pair"))?;

        if !key_value_pair.path.is_ident(inherit_attributes_ident) {
            return Err(syn::Error::new_spanned(
                key_value_pair.path,
                format!("expected `{inherit_attributes_ident}`"),
            ));
        }

        let inherit_attributes = match key_value_pair.value {
            Expr::Lit(ExprLit {
                lit: Lit::Bool(lit_bool),
                ..
            }) => lit_bool.value,
            other => {
                return Err(syn::Error::new_spanned(other, "expected a bool literal"));
            }
        };

        Ok(Self { inherit_attributes })
    }
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
                "function is missing a test attribute, such as #[test], #[tokio::test] or #[rstest] - add one below #[subtest]",
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

/// Whether the function has a `#[test]`, `#[tokio::test]`, `#[rstest]` attribute etc.
fn has_test_attr(item_fn: &ItemFn) -> bool {
    item_fn.attrs.iter().any(|attr| {
        !is_subtest_attr(attr)
            && attr
                .meta
                .path()
                .segments
                .last()
                .is_some_and(|segment| segment.ident.to_string().ends_with("test"))
    })
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

/// Strip the `#[subtest]` attribute off a subtest fn, parsing its arguments
fn remove_subtest_attrs(mut from_fn: ItemFn) -> Result<RemovedSubtestAttrs, syn::Error> {
    let mut parsed_config = None;
    let mut validation_error = None;

    from_fn.attrs.retain(|attr| {
        if !is_subtest_attr(attr) {
            return true;
        }

        if parsed_config.is_some() {
            validation_error = Some(syn::Error::new_spanned(
                attr,
                "duplicate #[subtest] attribute, remove one",
            ));
            return false;
        }

        parsed_config = Some(match &attr.meta {
            Meta::Path(_) => SubtestConfig::default(),
            Meta::List(list) => match SubtestConfig::parse(&list.tokens) {
                Ok(config) => config,
                Err(e) => {
                    validation_error = Some(e);
                    return false;
                }
            },
            Meta::NameValue(_) => {
                validation_error = Some(syn::Error::new_spanned(
                    attr,
                    "expected #[subtest] or #[subtest(<args>)]",
                ));
                return false;
            }
        });

        false
    });

    if let Some(validation_error) = validation_error {
        return Err(validation_error);
    }

    if let Some(subtest_config) = parsed_config {
        Ok(RemovedSubtestAttrs::RemovedSubtest {
            subtest_config,
            cleaned_function: from_fn,
        })
    } else {
        Ok(RemovedSubtestAttrs::NoSubtest(from_fn))
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
enum RemovedSubtestAttrs {
    NoSubtest(ItemFn),
    RemovedSubtest {
        cleaned_function: ItemFn,
        subtest_config: SubtestConfig,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn remove_subtest_attrs_none() {
        let input: ItemFn = parse_quote! { fn bare() {} };
        let result = remove_subtest_attrs(input.clone());
        assert_eq!(result.unwrap(), RemovedSubtestAttrs::NoSubtest(input));
    }

    #[test]
    fn remove_subtest_attrs_other() {
        let input: ItemFn = parse_quote! {
            #[inline]
            fn bare() {}
        };
        let result = remove_subtest_attrs(input.clone());
        assert_eq!(result.unwrap(), RemovedSubtestAttrs::NoSubtest(input));
    }

    #[test]
    fn remove_subtest_attrs_one_without_args() {
        let input: ItemFn = parse_quote! {
            #[subtest]
            #[inline]
            fn bare() {}
        };

        let result = remove_subtest_attrs(input.clone());

        assert_eq!(
            result.unwrap(),
            RemovedSubtestAttrs::RemovedSubtest {
                cleaned_function: parse_quote! {
                    #[inline]
                    fn bare() {}
                },
                subtest_config: SubtestConfig {
                    inherit_attributes: true,
                }
            }
        );
    }

    #[test]
    #[should_panic(expected = "expected '<key> = <value>' pair")]
    fn remove_subtest_attrs_one_with_wrong_arg_value() {
        let input: ItemFn = parse_quote! {
            #[subtest(foo)]
            fn bare() {}
        };
        let result = remove_subtest_attrs(input.clone());
        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "expected #[subtest] or #[subtest(<args>)]")]
    fn remove_subtest_attrs_one_with_wrong_arg_type() {
        let input: ItemFn = parse_quote! {
            #[subtest = foo]
            fn bare() {}
        };
        let result = remove_subtest_attrs(input.clone());
        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "duplicate #[subtest] attribute, remove one")]
    fn remove_subtest_attrs_two() {
        let input: ItemFn = parse_quote! {
            #[subtest]
            #[subtest]
            fn bare() {}
        };
        let result = remove_subtest_attrs(input.clone());
        result.unwrap();
    }

    #[test]
    fn remove_subtest_attrs_with_inherit_attributes_true() {
        let input: ItemFn = parse_quote! {
            #[subtest(inherit_attributes = true)]
            #[inline]
            #[test]
            fn bare() {}
        };

        let result = remove_subtest_attrs(input.clone());

        assert_eq!(
            result.unwrap(),
            RemovedSubtestAttrs::RemovedSubtest {
                cleaned_function: parse_quote! {
                    #[inline]
                    #[test]
                    fn bare() {}
                },
                subtest_config: SubtestConfig {
                    inherit_attributes: true,
                }
            }
        );
    }

    #[test]
    fn remove_subtest_attrs_with_inherit_attributes_false() {
        let input: ItemFn = parse_quote! {
            #[subtest(inherit_attributes = false)]
            #[inline]
            fn bare() {}
        };

        let result = remove_subtest_attrs(input.clone());

        assert_eq!(
            result.unwrap(),
            RemovedSubtestAttrs::RemovedSubtest {
                cleaned_function: parse_quote! {
                    #[inline]
                    fn bare() {}
                },
                subtest_config: SubtestConfig {
                    inherit_attributes: false,
                }
            }
        );
    }
}
