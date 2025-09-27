use super::ast::Param;
use super::symbol_table::SymbolTable;
use super::types::parse_type;
use crate::Rule;
use pest::iterators::Pair;

pub fn parse_var_section(pair: Option<Pair<Rule>>, sym_table: &mut SymbolTable) -> Vec<Param> {
    //println!("parse_var_section: entro");
    let mut locals = Vec::new();

    if let Some(var_section_pair) = pair {
        for var_decl_pair in var_section_pair.into_inner() {
            if var_decl_pair.as_rule() == Rule::var_decl {
                let mut inner = var_decl_pair.into_inner();

                let mut ids = Vec::new();
                let mut ty_pair_opt = None;

                while let Some(next) = inner.peek() {
                    match next.as_rule() {
                        Rule::ident => {
                            ids.push(inner.next().unwrap());
                        }
                        Rule::comma => {
                            inner.next(); // ignorar coma
                        }
                        Rule::keyword_integer | Rule::keyword_real | Rule::keyword_string | Rule::keyword_boolean => {
                            ty_pair_opt = Some(inner.next().unwrap());
                            break;
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if let Some(ty_pair) = ty_pair_opt {
                    let ty = parse_type(ty_pair);

                    for id_pair in ids {
                        let id = id_pair.as_str().to_string();

                        locals.push(Param {
                            name: id.clone(),
                            ty: ty.clone(),
                        });
                        sym_table.insert(id, ty.clone());
                    }
                }
            }
        }
    }

    locals
}
