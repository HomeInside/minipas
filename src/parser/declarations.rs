use super::ast::Param;
use super::symbol_table::SymbolTable;
use super::types::parse_type;
use crate::Rule;
use pest::iterators::Pair;

pub fn parse_var_section(pair: Option<Pair<Rule>>, sym_table: &mut SymbolTable) -> Vec<Param> {
    println!("parse_var_section: entro");
    let mut locals = Vec::new();

    if let Some(var_section_pair) = pair {
        for var_decl_pair in var_section_pair.into_inner() {
            if var_decl_pair.as_rule() == Rule::var_decl {
                let mut inner = var_decl_pair.into_inner();
                let ids_pair = inner.next().unwrap(); // ident list
                let ty_pair = inner.next().unwrap(); // type

                let ty = parse_type(ty_pair);
                for id_pair in ids_pair.into_inner() {
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

    locals
}
