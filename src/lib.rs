use std::{fs, collections::BTreeMap};

pub fn load(input: &str) -> Option<VariableMap> {
    match fs::read_to_string(input) {
        Ok(contents) => Some(get_variable_map(contents)),
        Err(_) => None,
    }
}

fn get_variable_map(contents: String) -> VariableMap {
    let lines = contents
        .split('\n')
        .collect::<Vec<&str>>();

    let mut varmap = BTreeMap::<String, String>::new();

    // Search all lines for a pair, and skip line if it does not have one.
    for line in lines {
        let (var, value) = match line.split_once('=') {
            Some(pair) => pair,
            None => continue,
        };

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

#[cfg(test)]
mod test {
    use crate::get_variable_map;


    #[test]
    fn complete_variable_map() {
        let contents = "abc = 123\nijk = lmnop";

        let varmap = get_variable_map(contents.to_string());
        let first = varmap.get_val("abc").unwrap();
        let second = varmap.get_val("ijk").unwrap();
        
        assert_eq!(first, "123");
        assert_eq!(second, "lmnop");
    }

    #[test]
    fn half_variable_map() {
        let contents = "abc 123\nijk = lm=nop";
        
        let varmap = get_variable_map(contents.to_string());
        let map_size = varmap.0.len();
        let first = varmap.get_val("ijk").unwrap();

        assert_eq!(map_size, 1);
        assert_eq!(first, "lm=nop");
    }

    #[test]
    fn empty_variable_map() {
        let contents = "\n\n\n\n\n\n";

        let varmap = get_variable_map(contents.to_string());
        let size = varmap.0.len();

        assert_eq!(size, 0);
    }

    #[test]
    #[should_panic]
    fn missing_variables() {
        let contents = "a = 1";
        
        let varmap = get_variable_map(contents.to_string());
        let _ = varmap.get_val("b").unwrap();
    }
}