use crate::Rule;
use pest::error::Error;

/// Traduce los nombres internos de las reglas de Pest
/// para los mensajes de error de sintaxis.
pub fn rename_msg_rule(rule: &Rule) -> String {
    match rule {
        // palabras clave
        Rule::keyword_const => "'const'".to_string(),
        Rule::keyword_var => "'var'".to_string(),
        Rule::keyword_program => "'program'".to_string(),
        Rule::keyword_function => "'function'".to_string(),
        Rule::keyword_procedure => "'procedure'".to_string(),
        Rule::keyword_integer => "'integer'".to_string(),
        Rule::keyword_real => "'real'".to_string(),
        Rule::keyword_string => "'string'".to_string(),
        Rule::keyword_boolean => "'boolean'".to_string(),
        Rule::keyword_byte => "'byte'".to_string(),
        Rule::keyword_nil => "'nil'".to_string(),
        // expresiones
        Rule::expr | Rule::sum | Rule::product | Rule::method_chain | Rule::expr_item => "una expresión".to_string(),
        Rule::factor => "una expresión".to_string(),
        Rule::expr_list => "una lista de expresiones".to_string(),
        Rule::block => "'begin'...'end'".to_string(),
        Rule::stmt => "una sentencia".to_string(),
        // TODO se necesitan mas casos aqui
        // fallback/otros casos no cubiertos
        other => format!("{:?}", other),
    }
}

/// Pest usa `renamed_rules` para cambiar el nombre
/// de todas las reglas si se trata de un «ParsingError».
/// pero no realiza ninguna acción cuando se invoca
/// sobre un «CustomError». útil para cambiar el nombre
/// de reglas con descripciones detalladas o para aplicar
/// un formato específico a cada regla.
pub fn rename_error(err: Error<Rule>) -> Error<Rule> {
    err.renamed_rules(rename_msg_rule)
}
