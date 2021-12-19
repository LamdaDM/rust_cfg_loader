use std::{fs, collections::BTreeMap};

pub fn load(input: &str) -> VariableMap {
    let contents = fs::read_to_string(input)
        .unwrap();

    let lines = contents
        .split('\n')
        .collect::<Vec<&str>>();

    let mut varmap = BTreeMap::<String, String>::new();

    for line in lines {
        let (var, value) = line.split_once('=').unwrap();

        varmap.insert(String::from(var.trim()), String::from(value.trim()));
    }

    VariableMap(varmap)
}

pub struct VariableMap(BTreeMap<String, String>);

impl VariableMap {
    pub fn get_val(&self, name: &str) -> Option<&str> {
        let op = self.0.get(name);

        match op {
            Some(val) => Some(val),
            None => None,
        }
    }
}