use super::assignment::parse_assignment;
use super::ast::{ForDir, Stmt};
use super::conditionals::parse_stmt_if;
use super::expressions::parse_expr;
use super::program::parse_block;
use super::program::parse_return_stmt;
use super::symbol_table::SymbolTable;
use crate::Rule;
use pest::iterators::Pair;

pub fn parse_stmt(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    //println!("parse_stmt entro");
    match pair.as_rule() {
        Rule::var_decl => {
            panic!("Declaración de variables no permitida dentro de begin…end");
        }
        Rule::stmt => {
            //println!("parse_stmt brazo Rule::stmt entro");
            let inner = pair.clone().into_inner().next().unwrap();
            //println!("inner: {}", inner);
            match inner.as_rule() {
                Rule::assignment => parse_assignment(inner, sym_table),
                Rule::if_stmt => parse_stmt_if(inner, sym_table),
                // permite statements de expresiones (como writeln(...);)
                Rule::block => parse_block(inner, sym_table),
                Rule::expr_stmt => {
                    let expr_pair = inner.into_inner().next().unwrap();
                    let expr = parse_expr(expr_pair, sym_table);
                    Stmt::Expr(expr)
                }
                Rule::for_stmt => {
                    // 👈 NUEVO
                    //println!("parse_stmt brazo Rule::stmt -> Rule::for_stmt entro");
                    let mut for_inner = inner.into_inner();

                    // Saltar keyword_for
                    let _for_kw = for_inner.next().expect("Se esperaba 'for'");

                    // variables
                    let var_pair = for_inner.next().expect("Se esperaba la variable del for");
                    assert_eq!(var_pair.as_rule(), Rule::ident);
                    let var = var_pair.as_str().to_string();

                    // Token :=
                    let assign_pair = for_inner.next().expect("Se esperaba ':=' en for");
                    assert_eq!(assign_pair.as_rule(), Rule::assign_op);

                    // Expresión inicial
                    let start_expr_pair = for_inner.next().expect("Se esperaba la expresión inicial del for");
                    let start_expr = parse_expr(start_expr_pair, sym_table);

                    // Dirección (to / downto)
                    let dir_pair = for_inner.next().expect("Se esperaba 'to' o 'downto' en for");
                    let direction = match dir_pair.as_str().to_lowercase().as_str() {
                        "to" => ForDir::To,
                        "downto" => ForDir::DownTo,
                        _ => panic!("Dirección inválida en for: {}", dir_pair.as_str()),
                    };

                    // Expresión final
                    let end_expr_pair = for_inner.next().expect("Se esperaba la expresión final del for");
                    let end_expr = parse_expr(end_expr_pair, sym_table);

                    // Saltar la keyword 'do'
                    let do_kw = for_inner.next().expect("Se esperaba 'do' en for");
                    assert_eq!(do_kw.as_rule(), Rule::keyword_do);

                    // Cuerpo del for
                    let body_pair = for_inner.next().expect("Se esperaba el cuerpo del for");
                    let body = parse_stmt(body_pair, sym_table);

                    Stmt::For {
                        var,
                        start: start_expr,
                        end: end_expr,
                        direction,
                        body: Box::new(body),
                    }
                }

                Rule::return_stmt => parse_return_stmt(inner, sym_table),
                other => panic!("Regla inesperada en stmt: {:?}", other),
            }
        }
        Rule::EOI => {
            // Ignorar fin/inicio de input
            // no hay un Stmt válido
            panic!("parse_stmt no debería recibir SOI/EOI directamente");
        }
        other => panic!("Regla inesperada en stmt: {:?}", other),
    }
}
