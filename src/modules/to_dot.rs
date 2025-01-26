use super::parser::parser::{Expression, IfStatement, Statement};

pub struct Dot<'a>(pub &'a Vec<Statement>);

impl<'a> Dot<'a> {
    /// Converts the entire program into a DOT representation.
    pub fn to_dot(&self) -> String {
        let mut output = String::from("digraph G {\n");
        for (i, stmt) in self.0.iter().enumerate() {
            Self::dotify_statement(stmt, &mut output, &format!("root_{}", i));
        }
        output.push_str("}\n");
        output
    }

    fn dotify_statement(stmt: &Statement, output: &mut String, parent: &str) {
        match stmt {
            Statement::MAKE(var) => {
                let node_id = format!("{}_make", parent);
                output.push_str(&format!("{} [label=\"MAKE: {}\"];\n", node_id, var.ident));
                Self::dotify_expression(&var.expr, output, &node_id);
                output.push_str(&format!("{} -> {};\n", parent, node_id));
            }
            Statement::ASSIGN(var) => {
                let node_id = format!("{}_assign", parent);
                output.push_str(&format!("{} [label=\"ASSIGN: {}\"];\n", node_id, var.ident));
                Self::dotify_expression(&var.expr, output, &node_id);
                output.push_str(&format!("{} -> {};\n", parent, node_id));
            }
            Statement::IF(if_stmt) => {
                Self::dotify_if_stmt(if_stmt, output, parent);
            }
            Statement::SCOPE(scope) => {
                if let Some(statements) = scope {
                    for (i, stmt) in statements.iter().enumerate() {
                        Self::dotify_statement(stmt, output, &format!("{}_scope_{}", parent, i));
                    }
                }
            }
        }
    }
    fn dotify_if_stmt(if_stmt: &IfStatement, output: &mut String, parent: &str){
        let node_id = format!("{}_if", parent);
        output.push_str(&format!("{} [label=\"IF\"];\n", node_id));
        output.push_str(&format!("{} -> {};\n", parent, node_id));

        // Expression
        let expr_id = format!("{}_expr", node_id);
        Self::dotify_expression(&if_stmt.expression, output, &expr_id);
        output.push_str(&format!("{} -> {};\n", node_id, expr_id));

        // If scope
        if let Some(scope) = &if_stmt.if_scope {
            for (i, stmt) in scope.iter().enumerate() {
                Self::dotify_statement(stmt, output, &format!("{}_if_scope_{}", node_id, i));
            }
        }

        // Else-if scope (recursive call as it's another IF statement)
        if let Some(else_if) = &if_stmt.else_if_scope {
            Self::dotify_if_stmt(else_if.as_ref(), output, &format!("{}_else_if", node_id));
        }

        // Else scope
        if let Some(scope) = &if_stmt.else_scope {
            for (i, stmt) in scope.iter().enumerate() {
                Self::dotify_statement(stmt, output, &format!("{}_else_scope_{}", node_id, i));
            }
        }
    }

    fn dotify_expression(expr: &Expression, output: &mut String, parent: &str) {
        match expr {
            Expression::BINEXPR(bin_expr) => {
                let node_id = format!("{}_bin_expr", parent);
                output.push_str(&format!(
                    "{} [label=\"BINEXPR: {:?}\"];\n",
                    node_id, bin_expr.operator
                ));
                output.push_str(&format!("{} -> {};\n", parent, node_id));

                // Left-hand side
                let lhs_id = format!("{}_lhs", node_id);
                Self::dotify_expression(&bin_expr.lhs, output, &lhs_id);

                // Right-hand side
                let rhs_id = format!("{}_rhs", node_id);
                Self::dotify_expression(&bin_expr.rhs, output, &rhs_id);
            }
            Expression::TERM(term) => {
                let node_id = format!("{}_term", parent);
                output.push_str(&format!("{} [label=\"TERM: {:?}\"];\n", node_id, term));
                output.push_str(&format!("{} -> {};\n", parent, node_id));
            }
            Expression::STRING(string) => {
                let node_id = format!("{}_string", parent);
                output.push_str(&format!("{} [label=\"STRING: {}\"];\n", node_id, string));
                output.push_str(&format!("{} -> {};\n", parent, node_id));
            }
        }
    }
}
