use crate::Rule;
use pest::error::Error;

/// Pest usa `renamed_rules` para cambiar el nombre
/// de todas las reglas si se trata de un `ParsingError`.
/// pero no realiza ninguna acción cuando se invoca
/// sobre un `CustomError`. útil para cambiar el nombre
/// de reglas con descripciones detalladas o para aplicar
/// un formato específico a cada regla.
///
pub fn rename_error(err: Error<Rule>) -> Error<Rule> {
    err.renamed_rules(rename_msg_rule)
}

/// Traduce los nombres internos de las reglas de Pest
/// para los mensajes de error de sintaxis.
///
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
        //
        Rule::keyword_if => "'if'".to_string(),
        Rule::keyword_then => "'then'".to_string(),
        Rule::keyword_else => "'else'".to_string(),
        //
        Rule::keyword_for => "'for'".to_string(),
        Rule::keyword_to => "'to'".to_string(),
        Rule::keyword_downto => "'downto'".to_string(),
        Rule::keyword_do => "'do'".to_string(),
        Rule::keyword_while => "'while'".to_string(),
        Rule::keyword_repeat => "'repeat'".to_string(),
        Rule::keyword_until => "'until'".to_string(),
        Rule::keyword_break => "'break'".to_string(),
        Rule::keyword_continue => "'continue'".to_string(),
        // operadores
        Rule::add_op => "'+'".to_string(),
        Rule::sub_op => "'-'".to_string(),
        Rule::mul_op => "'*'".to_string(),
        Rule::div_op => "'/'".to_string(),
        Rule::assign_op => "':='".to_string(),
        Rule::assign_add => "'+='".to_string(),
        Rule::assign_sub => "'-='".to_string(),
        Rule::assign_mul => "'*='".to_string(),
        Rule::assign_div => "'/='".to_string(),
        Rule::assign_mod => "'%='".to_string(),
        Rule::idiv_op => "'div'".to_string(),
        Rule::mod_op => "('%' ó 'mod')".to_string(),
        Rule::cmp_op => "'='".to_string(),
        //
        Rule::semicolon => "';'".to_string(),
        Rule::comma => "','".to_string(),
        Rule::colon => "':'".to_string(),
        Rule::lparen => "'('".to_string(),
        Rule::rparen => "')'".to_string(),
        // expresiones
        Rule::factor | Rule::expr | Rule::sum | Rule::product | Rule::method_chain | Rule::expr_item => {
            "una expresión".to_string()
        }
        //Rule::factor => "una expresión".to_string(),
        Rule::expr_list => "una lista de expresiones".to_string(),
        // literales/identificadores
        Rule::ident => "un identificador".to_string(),
        Rule::number => "un número".to_string(),
        //
        Rule::string_literal => "una cadena de caracteres".to_string(),
        Rule::boolean_literal => "'True' o 'False'".to_string(),
        // declaraciones
        Rule::var_decl => "una declaración de variable".to_string(),
        Rule::const_decl => "una declaración de constante".to_string(),
        Rule::var_section => "una sección 'var'".to_string(),
        Rule::const_section => "una sección 'const'".to_string(),
        Rule::func_decl => "una declaración de función".to_string(),
        Rule::proc_decl => "una declaración de procedimiento".to_string(),
        Rule::param => "un parámetro".to_string(),
        Rule::param_list => "una lista de parámetros".to_string(),
        Rule::type_keyword => "un tipo de dato".to_string(),
        // sentencias/bloques
        Rule::program => "un bloque de programa".to_string(),
        Rule::block => "un bloque 'begin'...'end'".to_string(),
        Rule::stmt => "una sentencia".to_string(),
        Rule::assignment => "una asignación".to_string(),
        Rule::if_stmt => "una sentencia 'if'".to_string(),
        //
        Rule::for_stmt => "un bucle 'for'".to_string(),
        Rule::while_stmt => "un bucle 'while'".to_string(),
        Rule::repeat_stmt => "un bucle 'repeat'".to_string(),
        Rule::return_stmt => "una sentencia 'return'".to_string(),
        Rule::break_stmt => "una sentencia 'break'".to_string(),
        Rule::continue_stmt => "una sentencia 'continue'".to_string(),
        // otros casos no cubiertos
        other => format!("{:?}", other),
    }
}
