use crate::parser::ast::VarType;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SymbolTable {
    variables: HashMap<String, VarType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
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
            return Err(format!("Variable '{}' ya declarada", name));
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
}
