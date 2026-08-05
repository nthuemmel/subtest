use syn::punctuated::Punctuated;
use syn::{Attribute, FnArg, Stmt, Token, parse_quote};

/// Go through the given statements, and mark all (top-level) declared variables in them as used, so they don't show up in unused-variable-warnings
pub fn mask_unused_variables(statements: Vec<Stmt>) -> Vec<Stmt> {
    statements
        .into_iter()
        .map(|mut statement| {
            if let Stmt::Local(declaration) = &mut statement {
                declaration.attrs.push(allow_unused_variables());
            }

            statement
        })
        .collect()
}

/// Mark all given parameters as used, so they don't show up in unused-variable-warnings
pub fn mask_unused_parameters(
    params: &Punctuated<FnArg, Token![,]>,
) -> Punctuated<FnArg, Token![,]> {
    params
        .iter()
        .cloned()
        .map(|mut param| {
            // a test function has no `self` parameter, so there is nothing to mask on a receiver
            if let FnArg::Typed(param) = &mut param {
                param.attrs.push(allow_unused_variables());
            }

            param
        })
        .collect()
}

/// The attribute which exempts a single declaration from the unused-variable lint.
///
/// Note that this does not work in a crate which sets `#![forbid(unused_variables)]`, because
/// `forbid` rejects any `allow` below it. We'll think about how to solve this once a user complains.
fn allow_unused_variables() -> Attribute {
    parse_quote!(#[allow(unused_variables)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn mask_variables_leaves_other_statements_alone() {
        let statements: Vec<Stmt> = parse_quote! {
            do_something();
        };

        let expected: Vec<Stmt> = parse_quote! {
            do_something();
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_variables_of_declaration() {
        let statements: Vec<Stmt> = parse_quote! {
            let (sender, receiver) = channel();
        };

        let expected: Vec<Stmt> = parse_quote! {
            #[allow(unused_variables)]
            let (sender, receiver) = channel();
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_variables_of_declaration_without_value() {
        let statements: Vec<Stmt> = parse_quote! {
            let value;
        };

        let expected: Vec<Stmt> = parse_quote! {
            #[allow(unused_variables)]
            let value;
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_variables_of_multiple_statements() {
        let statements: Vec<Stmt> = parse_quote! {
            let value = 5;
            do_something(value);
            let other = 6;
        };

        let expected: Vec<Stmt> = parse_quote! {
            #[allow(unused_variables)]
            let value = 5;
            do_something(value);
            #[allow(unused_variables)]
            let other = 6;
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_parameters() {
        let params: Punctuated<FnArg, Token![,]> = parse_quote!(value: u32, flag: bool);

        let expected: Punctuated<FnArg, Token![,]> = parse_quote! {
            #[allow(unused_variables)] value: u32,
            #[allow(unused_variables)] flag: bool
        };

        assert_eq!(mask_unused_parameters(&params), expected);
    }

    #[test]
    fn mask_parameters_keeps_existing_attributes() {
        let params: Punctuated<FnArg, Token![,]> = parse_quote!(#[case] status: TaskStatus);

        let expected: Punctuated<FnArg, Token![,]> = parse_quote! {
            #[case]
            #[allow(unused_variables)]
            status: TaskStatus
        };

        assert_eq!(mask_unused_parameters(&params), expected);
    }
}
