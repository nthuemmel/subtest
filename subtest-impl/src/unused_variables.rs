//! Variable declarations and assignments inherited from a parent test fn may show up as unused in a
//! nested subtest, if that subtest doesn't make use of them.
//! This is confusing when they are used in the parent, especially since the unused-variable-warning
//! will point to the parent's code.
//! Therefore, we mask declarations and assignments in inherited code with
//! `#[allow(unused_variables)]`, `#[allow(unused_assignments)]` and `#[allow(unused_mut)]`.

use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{Attribute, BinOp, Expr, FieldPat, FnArg, Item, Pat, Stmt, Token, parse_quote};

/// Go through the given statement, and mark all (top-level) declared variables in them as used, so they don't show up in unused-variable-warnings
pub fn mask_unused_variables(mut statement: Stmt) -> Stmt {
    if let Stmt::Local(declaration) = &mut statement {
        declaration.attrs.push(allow_unused_variables());

        if pattern_declares_mut_variable(&declaration.pat) {
            declaration.attrs.push(allow_unused_mut());
        }
    }

    if !assigns_a_variable(&statement) {
        return statement;
    }

    let attribute = allow_unused_assignments();

    match statement {
        // The lint is reported on the assignment itself, and an attribute on a plain assignment
        // statement is unstable - but a block accepts one, and wrapping an expression statement in
        // a block changes nothing else.
        Stmt::Expr(expression, _) => parse_quote!(#attribute { #expression; }),

        Stmt::Local(mut declaration) => {
            declaration.attrs.push(attribute);
            Stmt::Local(declaration)
        }

        Stmt::Macro(mut invocation) => {
            invocation.attrs.push(attribute);
            Stmt::Macro(invocation)
        }

        // a nested item cannot assign to the variables of the surrounding test function
        other @ Stmt::Item(_) => other,
    }
}

/// Whether a pattern declares a variable as `mut`
fn pattern_declares_mut_variable(pat: &Pat) -> bool {
    match pat {
        Pat::Ident(ident) => {
            ident.mutability.is_some()
                || ident
                    .subpat
                    .as_ref()
                    .is_some_and(|(_, pat)| pattern_declares_mut_variable(pat))
        }

        Pat::Type(type_pat) => pattern_declares_mut_variable(&type_pat.pat),
        Pat::Paren(paren) => pattern_declares_mut_variable(&paren.pat),
        Pat::Reference(ref_pat) => pattern_declares_mut_variable(&ref_pat.pat),
        Pat::Tuple(tuple) => tuple.elems.iter().any(pattern_declares_mut_variable),
        Pat::TupleStruct(tuple_struct) => {
            tuple_struct.elems.iter().any(pattern_declares_mut_variable)
        }
        Pat::Struct(struct_pat) => struct_pat
            .fields
            .iter()
            .any(|FieldPat { pat, .. }| pattern_declares_mut_variable(pat)),
        Pat::Slice(slice) => slice.elems.iter().any(pattern_declares_mut_variable),
        Pat::Or(or) => or.cases.iter().any(pattern_declares_mut_variable),

        _ => false,
    }
}

/// Whether the statement assigns to a variable, be it directly or nested inside a block, a branch,
/// a loop or a closure
fn assigns_a_variable(statement: &Stmt) -> bool {
    #[derive(Default)]
    struct AssignmentVisitor {
        found_assignment: bool,
    }

    impl<'ast> Visit<'ast> for AssignmentVisitor {
        fn visit_expr(&mut self, expression: &'ast Expr) {
            let is_assignment = match expression {
                Expr::Assign(_) => true,
                // syn represents compound assignments such as `value += 1` as binary expressions
                Expr::Binary(binary) => matches!(
                    binary.op,
                    BinOp::AddAssign(_)
                        | BinOp::SubAssign(_)
                        | BinOp::MulAssign(_)
                        | BinOp::DivAssign(_)
                        | BinOp::RemAssign(_)
                        | BinOp::BitXorAssign(_)
                        | BinOp::BitAndAssign(_)
                        | BinOp::BitOrAssign(_)
                        | BinOp::ShlAssign(_)
                        | BinOp::ShrAssign(_)
                ),
                _ => false,
            };

            self.found_assignment |= is_assignment;

            visit::visit_expr(self, expression);
        }

        // a nested item has its own scope, so its assignments are none of our business
        fn visit_item(&mut self, _item: &'ast Item) {}
    }

    let mut visitor = AssignmentVisitor::default();
    visitor.visit_stmt(statement);
    visitor.found_assignment
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

                if pattern_declares_mut_variable(&param.pat) {
                    param.attrs.push(allow_unused_mut());
                }
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

/// The attribute which exempts a single statement from the unused-assignment lint
fn allow_unused_assignments() -> Attribute {
    parse_quote!(#[allow(unused_assignments)])
}

/// The attribute which exempts a single declaration from the `unused_mut` lint.
///
/// Note that this does not work in a crate which sets `#![forbid(unused_mut)]`, because
/// `forbid` rejects any `allow` below it. We'll think about how to solve this once a user complains.
fn allow_unused_mut() -> Attribute {
    parse_quote!(#[allow(unused_mut)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn mask_variables_leaves_other_statements_alone() {
        let statement: Stmt = parse_quote! {
            do_something();
        };

        let expected: Stmt = parse_quote! {
            do_something();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_variables_of_declaration() {
        let statement: Stmt = parse_quote! {
            let (sender, receiver) = channel();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            let (sender, receiver) = channel();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_variables_of_declaration_without_value() {
        let statement: Stmt = parse_quote! {
            let value;
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            let value;
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_assignment() {
        let statement: Stmt = parse_quote! {
            value = 5;
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_assignments)]
            {
                value = 5;
            }
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_assignment_nested_in_a_block() {
        let statement: Stmt = parse_quote! {
            {
                value = 5;
            }
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_assignments)]
            {
                {
                    value = 5;
                };
            }
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_assignment_nested_in_a_branch() {
        let statement: Stmt = parse_quote! {
            if condition {
                value = 5;
            }
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_assignments)]
            {
                if condition {
                    value = 5;
                };
            }
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_compound_assignment() {
        let statement: Stmt = parse_quote! {
            value += 1;
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_assignments)]
            {
                value += 1;
            }
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_assignment_inside_a_declaration() {
        let statement: Stmt = parse_quote! {
            let other = {
                value = 5;
                6
            };
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_assignments)]
            let other = {
                value = 5;
                6
            };
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_leaves_assignments_in_a_nested_item_alone() {
        let statement: Stmt = parse_quote! {
            fn helper() {
                let mut value = 1;
                value = 2;
            }
        };

        let expected: Stmt = parse_quote! {
            fn helper() {
                let mut value = 1;
                value = 2;
            }
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_single_declaration() {
        let statement: Stmt = parse_quote! {
            let mut value = receive();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut value = receive();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_single_typed_declaration() {
        let statement: Stmt = parse_quote! {
            let mut value: i32 = receive();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut value: i32 = receive();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_tuple_declaration() {
        let statement: Stmt = parse_quote! {
            let (sender, mut receiver) = channel();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let (sender, mut receiver) = channel();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_declaration_without_value() {
        let statement: Stmt = parse_quote! {
            let mut value;
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut value;
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_typed_tuple_declaration() {
        let statement: Stmt = parse_quote! {
            let (sender, mut receiver): (Sender, Receiver) = channel();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let (sender, mut receiver): (Sender, Receiver) = channel();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_tuple_struct_declaration() {
        let statement: Stmt = parse_quote! {
            let Wrapper(mut value) = wrap();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let Wrapper(mut value) = wrap();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_struct_declaration() {
        let statement: Stmt = parse_quote! {
            let Point { x, y: mut down } = origin();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let Point { x, y: mut down } = origin();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_struct_declaration_with_shorthand_field() {
        let statement: Stmt = parse_quote! {
            let Point { x, mut y } = origin();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let Point { x, mut y } = origin();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_slice_declaration() {
        let statement: Stmt = parse_quote! {
            let [first, mut second, ..] = collect();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let [first, mut second, ..] = collect();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_parenthesized_declaration() {
        let statement: Stmt = parse_quote! {
            let (mut value) = receive();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let (mut value) = receive();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_alternative_declarations() {
        let statement: Stmt = parse_quote! {
            let (Ok(value) | Err(mut value)) = fallible();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let (Ok(value) | Err(mut value)) = fallible();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_behind_a_reference() {
        let statement: Stmt = parse_quote! {
            let &(mut value) = reference();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let &(mut value) = reference();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_subpattern_of_a_binding() {
        let statement: Stmt = parse_quote! {
            let all @ [mut first, ..] = collect();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let all @ [mut first, ..] = collect();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_of_deeply_nested_declaration() {
        let statement: Stmt = parse_quote! {
            let (sender, Wrapper { receiver: Some([_, mut last]) }) = channel();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let (sender, Wrapper { receiver: Some([_, mut last]) }) = channel();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_leaves_immutable_declarations_alone() {
        let statement: Stmt = parse_quote! {
            let Wrapper { values: [first, ..], rest: Some((second, _)) } = wrap();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            let Wrapper { values: [first, ..], rest: Some((second, _)) } = wrap();
        };

        assert_eq!(mask_unused_variables(statement), expected);
    }

    #[test]
    fn mask_mut_leaves_a_mutable_reference_alone() {
        let statement: Stmt = parse_quote! {
            let &mut value = reference();
        };

        let expected: Stmt = parse_quote! {
            #[allow(unused_variables)]
            let &mut value = reference();
        };

        assert_eq!(mask_unused_variables(statement), expected);
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

    #[test]
    fn mask_mut_of_parameters() {
        let params: Punctuated<FnArg, Token![,]> = parse_quote!(mut value: u32, flag: bool);

        let expected: Punctuated<FnArg, Token![,]> = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            mut value: u32,
            #[allow(unused_variables)] flag: bool
        };

        assert_eq!(mask_unused_parameters(&params), expected);
    }

    #[test]
    fn mask_mut_of_parameter_keeps_existing_attributes() {
        let params: Punctuated<FnArg, Token![,]> = parse_quote!(#[case] mut status: TaskStatus);

        let expected: Punctuated<FnArg, Token![,]> = parse_quote! {
            #[case]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            mut status: TaskStatus
        };

        assert_eq!(mask_unused_parameters(&params), expected);
    }

    #[test]
    fn mask_mut_of_destructured_parameter() {
        let params: Punctuated<FnArg, Token![,]> =
            parse_quote!((sender, mut receiver): (Sender, Receiver));

        let expected: Punctuated<FnArg, Token![,]> = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            (sender, mut receiver): (Sender, Receiver)
        };

        assert_eq!(mask_unused_parameters(&params), expected);
    }

    #[test]
    fn mask_mut_leaves_immutable_parameters_alone() {
        let params: Punctuated<FnArg, Token![,]> =
            parse_quote!(value: u32, &mut reference: &mut u32);

        let expected: Punctuated<FnArg, Token![,]> = parse_quote! {
            #[allow(unused_variables)] value: u32,
            #[allow(unused_variables)] &mut reference: &mut u32
        };

        assert_eq!(mask_unused_parameters(&params), expected);
    }
}
