use crate::config::SubtestConfig;
use crate::unused_variables::{mask_unused_parameters, mask_unused_variables};
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::{Attribute, FnArg, ItemFn, Meta, ReturnType, Stmt, Token};

pub struct InheritableFunctionAspects {
    attributes: Vec<Attribute>,
    parameters: Punctuated<FnArg, Token![,]>,
    return_type: ReturnType,
    statements: Vec<Stmt>,
}

impl InheritableFunctionAspects {
    pub fn none() -> Self {
        Self {
            attributes: Vec::new(),
            parameters: Punctuated::new(),
            return_type: ReturnType::Default,
            statements: Vec::new(),
        }
    }

    /// Applies inheritable aspects to the given `to_function`, and derives and returns new
    /// inheritable function aspects for the given `to_function`
    pub fn apply(&self, to_function: &mut ItemFn, config: &SubtestConfig) -> Self {
        let new_inheritable_attributes;
        if config.inherit_attributes {
            new_inheritable_attributes = self
                .attributes
                .iter()
                .cloned()
                .chain(inheritable_attributes(to_function.attrs.clone()))
                .collect();

            to_function.attrs = self
                .attributes
                .iter()
                .cloned()
                .chain(std::mem::take(&mut to_function.attrs))
                .collect();
        } else {
            new_inheritable_attributes = inheritable_attributes(to_function.attrs.clone());
        }

        // Inherit function parameters if the subtest fn does not specify any.
        let new_inheritable_parameters = if to_function.sig.inputs.is_empty() {
            to_function.sig.inputs = self.parameters.clone();
            self.parameters.clone()
        } else {
            // mask unused params - as long as the params are used in the parent, they should not show up as unused just because one of the subtests doesn't make use of them!
            mask_unused_parameters(&to_function.sig.inputs)
        };

        // Inherit function return type if the subtest fn does not specify any
        if matches!(to_function.sig.output, ReturnType::Default) {
            to_function.sig.output = self.return_type.clone();
        }
        let new_inheritable_return_type = to_function.sig.output.clone();

        to_function.block.stmts = self.statements.clone();

        Self {
            attributes: new_inheritable_attributes,
            parameters: new_inheritable_parameters,
            return_type: new_inheritable_return_type,
            statements: self.statements.clone(),
        }
    }

    pub fn add_statement(&mut self, stmt: Stmt) {
        self.statements.push(mask_unused_variables(stmt));
    }
}

/// Return only the attributes from the given list of `attributes` that a nested subtest can
/// inherit
fn inheritable_attributes(attributes: Vec<Attribute>) -> Vec<Attribute> {
    attributes
        .into_iter()
        // Doc comments describe the function they are written on, so they are not passed down
        .filter(|attr| !is_doc_attr(attr))
        // Neither is an attribute describing that function's own outcome
        .filter(|attr| !is_test_outcome_attr(attr))
        // Pass an `#[expect(...)]` down to nested subtests as an `#[allow(...)]`.
        // `#[expect]` may misfire on nested subtests if the expected thing only happens in the
        // parent and is not inherited
        .map(downgrade_expect_to_allow)
        .collect()
}

/// Whether an attribute is a doc comment (or an equivalent `#[doc = "..."]` attribute)
fn is_doc_attr(attr: &Attribute) -> bool {
    attr.meta.path().is_ident("doc")
}

/// Whether an attribute states the outcome expected of the test function it is written on, such as
/// `#[ignore]` and `#[should_panic]`
fn is_test_outcome_attr(attr: &Attribute) -> bool {
    let path = attr.meta.path();
    path.is_ident("ignore") || path.is_ident("should_panic")
}

fn downgrade_expect_to_allow(mut attr: Attribute) -> Attribute {
    if let Meta::List(lints) = &mut attr.meta {
        if lints.path.is_ident("expect") {
            let name = &mut lints.path.segments[0].ident;
            *name = Ident::new("allow", name.span());
        }
    }

    attr
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn inheritable_attributes_drop_doc_comments() {
        let attributes: Vec<Attribute> = vec![
            parse_quote!(#[doc = "a doc comment"]),
            parse_quote!(#[test]),
        ];

        let expected: Vec<Attribute> = vec![parse_quote!(#[test])];

        assert_eq!(inheritable_attributes(attributes), expected);
    }

    #[test]
    fn inheritable_attributes_drop_test_outcome_attributes() {
        let attributes: Vec<Attribute> = vec![
            parse_quote!(#[test]),
            parse_quote!(#[ignore]),
            parse_quote!(#[ignore = "a reason"]),
            parse_quote!(#[should_panic]),
            parse_quote!(#[should_panic(expected = "boom")]),
        ];

        let expected: Vec<Attribute> = vec![parse_quote!(#[test])];

        assert_eq!(inheritable_attributes(attributes), expected);
    }

    #[test]
    fn inheritable_attributes_keep_other_attributes() {
        let attributes: Vec<Attribute> = vec![
            parse_quote!(#[test]),
            parse_quote!(#[allow(clippy::too_many_lines)]),
            parse_quote!(#[track_caller]),
            parse_quote!(#[cfg(unix)]),
        ];

        assert_eq!(inheritable_attributes(attributes.clone()), attributes);
    }

    #[test]
    fn inheritable_attributes_downgrade_expects_to_allows() {
        let attributes: Vec<Attribute> = vec![
            parse_quote!(#[expect(clippy::too_many_lines)]),
            parse_quote!(#[expect(unused_variables, unused_assignments, reason = "inherited")]),
        ];

        let expected: Vec<Attribute> = vec![
            parse_quote!(#[allow(clippy::too_many_lines)]),
            parse_quote!(#[allow(unused_variables, unused_assignments, reason = "inherited")]),
        ];

        assert_eq!(inheritable_attributes(attributes), expected);
    }
}
