//! Variable declarations and assignments inherited from a parent test fn may show up as unused in a
//! nested subtest, if that subtest doesn't make use of them.
//! This is confusing when they are used in the parent, especially since the unused-variable-warning
//! will point to the parent's code.
//! Therefore, we mask declarations and assignments in inherited code with
//! `#[allow(unused_variables)]` and `#[allow(unused_assignments)]`.

use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{Attribute, BinOp, Expr, FnArg, Item, Stmt, Token, parse_quote};

/// Go through the given statements, and mark all (top-level) declared variables in them as used, so they don't show up in unused-variable-warnings
pub fn mask_unused_variables(statements: Vec<Stmt>) -> Vec<Stmt> {
    statements.into_iter().map(mask_statement).collect()
}

fn mask_statement(mut statement: Stmt) -> Stmt {
    if let Stmt::Local(declaration) = &mut statement {
        declaration.attrs.push(allow_unused_variables());
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
    fn mask_assignment() {
        let statements: Vec<Stmt> = parse_quote! {
            value = 5;
        };

        let expected: Vec<Stmt> = parse_quote! {
            #[allow(unused_assignments)]
            {
                value = 5;
            }
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_assignment_nested_in_a_block() {
        let statements: Vec<Stmt> = parse_quote! {
            {
                value = 5;
            }
        };

        let expected: Vec<Stmt> = parse_quote! {
            #[allow(unused_assignments)]
            {
                {
                    value = 5;
                };
            }
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_assignment_nested_in_a_branch() {
        let statements: Vec<Stmt> = parse_quote! {
            if condition {
                value = 5;
            }
        };

        let expected: Vec<Stmt> = parse_quote! {
            #[allow(unused_assignments)]
            {
                if condition {
                    value = 5;
                };
            }
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_compound_assignment() {
        let statements: Vec<Stmt> = parse_quote! {
            value += 1;
        };

        let expected: Vec<Stmt> = parse_quote! {
            #[allow(unused_assignments)]
            {
                value += 1;
            }
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_assignment_inside_a_declaration() {
        let statements: Vec<Stmt> = parse_quote! {
            let other = {
                value = 5;
                6
            };
        };

        let expected: Vec<Stmt> = parse_quote! {
            #[allow(unused_variables)]
            #[allow(unused_assignments)]
            let other = {
                value = 5;
                6
            };
        };

        assert_eq!(mask_unused_variables(statements), expected);
    }

    #[test]
    fn mask_leaves_assignments_in_a_nested_item_alone() {
        let statements: Vec<Stmt> = parse_quote! {
            fn helper() {
                let mut value = 1;
                value = 2;
            }
        };

        let expected: Vec<Stmt> = parse_quote! {
            fn helper() {
                let mut value = 1;
                value = 2;
            }
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
