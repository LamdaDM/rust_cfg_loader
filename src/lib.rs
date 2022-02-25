use std::{fs, collections::BTreeMap};

/// Convenience function for `get_variable_map`. This reads the contents of the file given and parses into the variable map.
/// If contents of file could not be read, this returns `None`.
pub fn load(file_path: &str) -> Option<VariableMap> {
    match fs::read_to_string(file_path) {
        Ok(contents) => Some(get_variable_map(contents)),
        Err(_) => None,
    }
}


/// Parses the contents into a variable map.
/// 
/// The contents are separated into lines, and each line is checked for a pair. 
/// The key and the value of a pair are separated by the first appearance of '='.
/// Lines without any appearance of '=' are ignored.
/// ## Example
/// ``` rust
/// let contents = "abc = 123\nijk = lmnop";
/// let varmap = cfg_loader::get_variable_map(contents.to_string());
/// let first = varmap.get_val("abc").unwrap();
/// let second = varmap.get_val("ijk").unwrap();
/// assert_eq!(first, "123");
/// assert_eq!(second, "lmnop");
/// ```
pub fn get_variable_map(contents: String) -> VariableMap {
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
        match self.0.get(name) {
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