use super::ast::Param;
use super::types::parse_type;
use crate::Rule;
use pest::iterators::Pair;

// Parsea un parámetro individual: ident : tipo
pub fn parse_param(pair: Pair<Rule>) -> Param {
    println!("parse_param: entro");
    let mut inner = pair.into_inner();

    let name_pair = inner.next().expect("parse_param: se esperaba un identificador");
    let name = name_pair.as_str().to_string();

    let colon_pair = inner.next().expect("parse_param: se esperaba ':'");
    assert_eq!(colon_pair.as_rule(), Rule::colon);

    let type_pair = inner.next().expect("parse_param: se esperaba tipo");
    let ty = parse_type(type_pair);

    Param { name, ty }
}

// Parsea la lista de parámetros: param (; param)*
pub fn parse_param_list(pair: Pair<Rule>) -> Vec<Param> {
    println!("parse_param_list: entro");
    assert_eq!(pair.as_rule(), Rule::param_list);
    let mut params = Vec::new();

    for p in pair.into_inner() {
        if p.as_rule() == Rule::param {
            params.push(parse_param(p));
        }
        // ignoramos otros tokens como semicolon
    }

    params
}
