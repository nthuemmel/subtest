use crate::config::SubtestConfig;
use syn::visit::Visit;
use syn::{Attribute, ItemFn, Meta, Path, Stmt, visit};

/// Whether the function has a `#[test]`, `#[tokio::test]`, `#[rstest]` attribute etc.
pub fn has_test_attr(item_fn: &ItemFn) -> bool {
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

/// Reject `#[subtest]` functions which are not declared directly in the body of their parent test
/// function.
///
/// Only the statements of a test function's body are searched for subtests, so a `#[subtest]`
/// function nested inside an `if`, a loop, a block, a closure or another item, would
/// keep its `#[subtest]` attribute and be expanded by the compiler as if it were a top-level test
/// function, which is definitely not what we want.
pub fn check_for_misplaced_subtests(statement: &Stmt) -> Result<(), syn::Error> {
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

/// Strip the `#[subtest]` attribute off a subtest fn, parsing its arguments
pub fn remove_subtest_attrs(mut from_fn: ItemFn) -> Result<RemovedSubtestAttrs, syn::Error> {
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
pub enum RemovedSubtestAttrs {
    NoSubtest(ItemFn),
    RemovedSubtest {
        cleaned_function: ItemFn,
        subtest_config: SubtestConfig,
    },
}

/// Whether an attribute is the `#[subtest]` attribute (or `#[subtest::subtest]`)
fn is_subtest_attr(attr: &Attribute) -> bool {
    let path = attr.meta.path();
    path.is_ident("subtest") || path_matches(path, &["subtest", "subtest"])
}

fn path_matches(path: &Path, segments: &[&str]) -> bool {
    // ignores leading `::`
    path.segments.len() == segments.len()
        && path
            .segments
            .iter()
            .zip(segments)
            .all(|(s, e)| s.ident == e)
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
    #[should_panic(
        expected = "`allow_missing_test_attribute` is only available on the top-level #[subtest] function"
    )]
    fn remove_subtest_attrs_one_with_top_level_only_arg() {
        let input: ItemFn = parse_quote! {
            #[subtest(allow_missing_test_attribute)]
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
