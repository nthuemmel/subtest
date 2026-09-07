mod span;

use crate::config::SubtestConfig;
use crate::inheritance::span::Spanned;
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use syn::{
    Attribute, Block, FnArg, Generics, ItemFn, Meta, ReturnType, Signature, Stmt, Token, Type,
    Visibility, parse_quote,
};

#[derive(Clone)]
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
        // By marking inherited aspects as macro-generated, linting is later suppressed for them.
        // They should only be linted in the context of the parent function, but not in nested
        // subtests, where we can get many false positives, like:
        // - unused_variables, unused_mut, unused_assignments, all for variables or parameters
        //   only used in the parent later
        // - unfulfilled_lint_expectations for expects which only hold in the parent
        self.clone()
            .mark_as_macro_generated()
            .apply_impl(to_function, config)
    }

    fn apply_impl(&self, to_function: &mut ItemFn, config: &SubtestConfig) -> Self {
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
            to_function.sig.inputs.clone_from(&self.parameters);
            self.parameters.clone()
        } else {
            to_function.sig.inputs.clone()
        };

        // Inherit function return type if the subtest fn does not specify any
        if matches!(to_function.sig.output, ReturnType::Default) {
            to_function.sig.output = self.return_type.clone();

            if !matches!(to_function.sig.output, ReturnType::Default) {
                // When a subtest inherits a `Result` or `Option` return type, but itself always
                // returns `Ok` or `Some`, the `clippy::unnecessary_wraps` lint is triggered.
                // However, the lint points at the parent. The user cannot change the parent if the
                // parent later emits an `Err` or `None`, making this lint a false positive.
                // Suppress it.
                to_function
                    .attrs
                    .push(parse_quote!(#[allow(clippy::unnecessary_wraps)]));
            }
        } else if is_unit(&to_function.sig.output)
            && !matches!(self.return_type, ReturnType::Default)
        {
            // when explicitly resetting the return type, `clippy::unused_unit` is a false positive
            // and must be suppressed
            to_function
                .attrs
                .push(parse_quote!(#[allow(clippy::unused_unit)]));
        }

        let new_inheritable_return_type = if is_unit(&to_function.sig.output) {
            // Inherit `-> ()` as the default return type, to not trigger `clippy::unused_unit`
            ReturnType::Default
        } else {
            to_function.sig.output.clone()
        };

        to_function.block.stmts.clone_from(&self.statements);

        Self {
            attributes: new_inheritable_attributes,
            parameters: new_inheritable_parameters,
            return_type: new_inheritable_return_type,
            statements: self.statements.clone(),
        }
    }

    pub fn add_statement(&mut self, stmt: Stmt) {
        self.statements.push(stmt);
    }
}

impl Spanned for InheritableFunctionAspects {
    fn mark_as_macro_generated(self) -> Self {
        let Self {
            attributes,
            parameters,
            return_type,
            statements,
        } = self;

        let wrapper_function = ItemFn {
            attrs: attributes,
            vis: Visibility::Inherited,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: syn::token::Fn::default(),
                ident: parse_quote!(dummy),
                generics: Generics::default(),
                paren_token: Paren::default(),
                inputs: parameters,
                variadic: None,
                output: return_type,
            },
            block: Box::new(Block {
                brace_token: Brace::default(),
                stmts: statements,
            }),
        };

        let wrapper_function = wrapper_function.mark_as_macro_generated();

        Self {
            attributes: wrapper_function.attrs,
            parameters: wrapper_function.sig.inputs,
            return_type: wrapper_function.sig.output,
            statements: wrapper_function.block.stmts,
        }
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

/// Whether the return type is explicitly `-> ()`
fn is_unit(return_type: &ReturnType) -> bool {
    match return_type {
        ReturnType::Default => false,
        ReturnType::Type(_, ty) => matches!(&**ty, Type::Tuple(tuple) if tuple.elems.is_empty()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    /// Everything a subtest can inherit, in a shape which does not round trip through a token
    /// stream on its own: attributes and parameters have no `Parse` implementation, and the
    /// trailing `Ok(())` would fail when parsed by itself
    fn every_aspect() -> InheritableFunctionAspects {
        let body: Block = parse_quote!({
            let (sender, mut receiver) = channel();
            fn helper() {}
            Ok(())
        });

        InheritableFunctionAspects {
            attributes: vec![
                parse_quote!(#[tokio::test]),
                parse_quote!(#[expect(unused_mut, reason = "a reason")]),
            ],
            parameters: parse_quote!(#[case] mut status: TaskStatus, other: u32),
            return_type: parse_quote!(-> anyhow::Result<()>),
            statements: body.stmts,
        }
    }

    /// Spans take no part in syn's `PartialEq`, so this compares the syntax itself - which marking
    /// has to leave untouched. That the spans *are* marked cannot be observed from a unit test,
    /// see the note on `span::tests`.
    fn assert_aspects_are_preserved(aspects: &InheritableFunctionAspects) {
        let marked = aspects.clone().mark_as_macro_generated();

        assert_eq!(marked.attributes, aspects.attributes, "attributes");
        assert_eq!(marked.parameters, aspects.parameters, "parameters");
        assert_eq!(marked.return_type, aspects.return_type, "return type");
        assert_eq!(marked.statements, aspects.statements, "statements");
    }

    #[test]
    fn mark_every_aspect_at_once() {
        assert_aspects_are_preserved(&every_aspect());
    }

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
