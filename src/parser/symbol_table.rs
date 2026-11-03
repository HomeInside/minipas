use crate::parser::ast::Value;
use crate::parser::ast::VarType;
use bincode::{Decode, Encode};
use std::collections::HashMap;

#[derive(Encode, Decode, Clone, Debug)]
pub struct SymbolTable {
    variables: HashMap<String, VarType>,
    constants: HashMap<String, (VarType, Value)>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            constants: HashMap::new(),
        }
    }
    pub fn global_vars(&self) -> Vec<(String, VarType)> {
        self.variables
            .iter()
            .map(|(name, ty)| (name.clone(), ty.clone()))
            .collect()
    }

    pub fn with_builtins() -> Self {
        let mut table = SymbolTable::new();

        // Constante nula
        table.insert("nil".to_string(), VarType::Nil);

        table
    }
    pub fn declare(&mut self, name: &str, typ: VarType) -> Result<(), String> {
        if self.variables.contains_key(name) {
            return Err(format!("Variable '{}' ya declarada anteriormente", name));
        }
        // se valida las constantes
        if self.constants.contains_key(name) {
            return Err(format!("'{}' ya ha sido declarada como constante", name));
        }
        self.variables.insert(name.to_string(), typ);
        Ok(())
    }

    // TO FIX
    #[allow(dead_code)]
    pub fn get_type(&self, name: &str) -> Option<&VarType> {
        self.variables.get(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn insert(&mut self, name: String, ty: VarType) {
        self.variables.insert(name, ty);
    }

    /// valida y agrega una constante
    pub fn declare_const(&mut self, name: &str, ty: VarType, val: Value) -> Result<(), String> {
        if self.exists(name) {
            return Err(format!(
                "El identificador '{}' ya ha sido declarado antes como variable",
                name
            ));
        }
        if self.is_constant(name) {
            return Err(format!("La constante '{}' ya ha sido declarada", name));
        }
        self.constants.insert(name.to_string(), (ty, val));
        Ok(())
    }

    pub fn is_constant(&self, name: &str) -> bool {
        self.constants.contains_key(name)
    }

    /// obtiene la lista de constantes
    pub fn global_constants(&self) -> Vec<(String, (VarType, Value))> {
        self.constants.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}
