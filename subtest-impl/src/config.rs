use proc_macro2::{Ident, TokenStream, TokenTree};
use syn::{Expr, ExprLit, Lit, MetaNameValue};

/// Argument accepted by the `#[subtest]` attribute of a top-level test function
const ALLOW_MISSING_TEST_ATTR_ARG: &str = "allow_missing_test_attribute";

/// Argument accepted by the `#[subtest]` attribute of a nested subtest function
const INHERIT_ATTRIBUTES_ARG: &str = "inherit_attributes";

/// Whether the argument list starts with the given argument name, no matter whether it is written
/// as a bare name or as a `<key> = <value>` pair
fn args_start_with(args: &TokenStream, arg_name: &str) -> bool {
    matches!(args.clone().into_iter().next(), Some(TokenTree::Ident(ident)) if ident == arg_name)
}

#[cfg_attr(test, derive(Debug))]
pub struct MacroConfig {
    pub allow_missing_test_attr: bool,
}

impl MacroConfig {
    pub fn parse(args: &TokenStream) -> Result<Self, syn::Error> {
        if args.is_empty() {
            return Ok(Self {
                allow_missing_test_attr: false,
            });
        }

        if args_start_with(args, INHERIT_ATTRIBUTES_ARG) {
            return Err(syn::Error::new_spanned(
                args,
                format!(
                    "`{INHERIT_ATTRIBUTES_ARG}` is only available on nested #[subtest] functions\n\
                     a top-level test function has no parent to inherit attributes from"
                ),
            ));
        }

        if syn::parse2::<Ident>(args.clone())
            .ok()
            .map(|ident| ident.to_string())
            .as_deref()
            == Some(ALLOW_MISSING_TEST_ATTR_ARG)
        {
            return Ok(Self {
                allow_missing_test_attr: true,
            });
        }

        Err(syn::Error::new_spanned(
            args,
            format!("expected either {ALLOW_MISSING_TEST_ATTR_ARG} or no arguments"),
        ))
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct SubtestConfig {
    pub inherit_attributes: bool,
}

impl Default for SubtestConfig {
    fn default() -> Self {
        Self {
            inherit_attributes: true,
        }
    }
}

impl SubtestConfig {
    pub fn parse(args: &TokenStream) -> Result<Self, syn::Error> {
        if args.is_empty() {
            return Ok(Self::default());
        }

        if args_start_with(args, ALLOW_MISSING_TEST_ATTR_ARG) {
            return Err(syn::Error::new_spanned(
                args,
                format!(
                    "`{ALLOW_MISSING_TEST_ATTR_ARG}` is only available on the top-level #[subtest] function\n\
                     it applies to the whole subtest tree, so specify it on the enclosing test function instead"
                ),
            ));
        }

        let key_value_pair: MetaNameValue = syn::parse2(args.clone())
            .map_err(|_| syn::Error::new_spanned(args, "expected '<key> = <value>' pair"))?;

        if !key_value_pair.path.is_ident(INHERIT_ATTRIBUTES_ARG) {
            return Err(syn::Error::new_spanned(
                key_value_pair.path,
                format!("expected `{INHERIT_ATTRIBUTES_ARG}`"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    #[should_panic(
        expected = "`inherit_attributes` is only available on nested #[subtest] functions"
    )]
    fn macro_config_with_nested_only_arg() {
        let result = MacroConfig::parse(&quote! { inherit_attributes = false });
        result.unwrap();
    }
}
