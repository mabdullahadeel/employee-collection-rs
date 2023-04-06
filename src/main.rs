fn main() {
    use crate::comp_employee_record::*;
}


mod comp_employee_record {
    use std::collections::HashMap;

    #[allow(dead_code)]
    enum Function {
        Add,
        Delete,
        Reset,
    }
    
    struct ParsedRes {
        function: Function,
        name: String,
        department: String
    }
    
    fn parse_function(f: &String) -> Result<Function, String> {
        let t = f.to_uppercase();
        if t == "ADD" {
            Ok(Function::Add)
        } else if t == "REMOVE" || t == "DELETE" {
            Ok(Function::Delete)
        } else if t == "RESET" {
            Ok(Function::Reset)
        } else {
            Err(String::from("Invlid function"))
        }
    }
    
    fn parse_statement(stmt: &String) -> Result<ParsedRes, String> {
        let parts: Vec<&str> = stmt.split_whitespace().collect();
        if parts.len() != 4 {
            panic!("Invalid statement");
        } else {
            match parse_function(&parts.get(0).unwrap().to_string()) {
                Ok(v) => {
                    Ok(ParsedRes { 
                        function: v,
                        name: parts.get(1).unwrap().to_string(),
                        department: parts.get(3).unwrap().to_string(),
                    })
                },
                Err(v) => Err(v)
            }
        }
    }

    fn has_department(department: &String, db: &DB) -> bool {
        db.contains_key(department)
    }

    fn encode_name(name: &String) -> String {
        name.to_lowercase()
    }

    fn add_employee(res: &ParsedRes, db: &mut DB) {
        if has_department(&res.department, db) {
            db.get_mut(&res.department).unwrap().push(encode_name(&res.name))
        } else {
            db.insert(res.department.clone(), vec![encode_name(&res.name)]);
        }
    }

    fn remove_employee(res: &ParsedRes, db: &mut DB) {
        if has_department(&res.department, db) {
            db.get_mut(&res.department).unwrap().retain(|x| x.clone() != encode_name(&res.name))
        }
    }

    fn reset_department(res: &ParsedRes, db: &mut DB) {
        if has_department(&res.department, db) {
            db.get_mut(&res.department).unwrap().clear()
        }
    }
    
    #[allow(dead_code)]
    type DB = HashMap<String, Vec<String>>;
    
    #[allow(dead_code)]
    pub fn update_employee_record(stmt: &String, db: &mut DB) {
        let p = parse_statement(&stmt);
        match p {
            Ok(value) => {
                match value.function {
                    Function::Add => {
                        add_employee(&value, db)
                    } 
                    Function::Delete => {
                        remove_employee(&value, db)
                    }
                    Function::Reset => {
                        reset_department(&value, db)
                    }
                }
            },
            Err(msg) => panic!("{}", msg)
        }
    }

    pub struct EmployeeDBMap {
        map: DB
    }

    #[allow(dead_code)]
    impl EmployeeDBMap {
        pub fn new() -> Self {
            Self {
                map: HashMap::new()
            }
        }
        pub fn perform_query(&mut self, stmt: String) {
            self::update_employee_record(&stmt, &mut self.map)
        }
        pub fn is_employee(&self, name: &String) -> bool {
            for (_, value) in &self.map {
                let name = name.to_lowercase();
                if value.contains(&name) {
                    return true
                }
            }
            false
        }
        pub fn total_employees(&self) -> usize {
            self.map.values().fold(0, |acc, x| acc + x.len())
        }
        pub fn print_employees(&self) {
            println!("{:?}", self.map);
            for (_, value) in &self.map {
                for name in value {
                    println!("{}", name)
                }
            }
        }
    }
}

#[cfg(test)]
mod test_employee_record {
    use crate::comp_employee_record::{EmployeeDBMap};

    #[test]
    fn test_add() {
        let mut db = EmployeeDBMap::new();
        db.perform_query(String::from("Add Jason to Accounts"));
        assert_eq!(db.total_employees(), 1);
        assert_eq!(db.is_employee(&String::from("Jason")), true)
    }
    #[test]
    fn test_remove() {
        let mut db = EmployeeDBMap::new();
        db.perform_query(String::from("Add Jason to Accounts"));
        db.perform_query(String::from("Delete Marry from Finance"));
        assert_eq!(db.total_employees(), 1);       
        db.perform_query(String::from("Delete Jason from Accounts"));
    }
    #[test]
    fn test_reset() {
        let mut db = EmployeeDBMap::new();
        db.perform_query(String::from("Add Jason to Accounts"));
        db.perform_query(String::from("Reset department of Accounts"));
        assert_eq!(db.total_employees(), 0);

    }
}