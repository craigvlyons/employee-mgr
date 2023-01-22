use std::collections::HashMap;
use crate::user_input::get_input;

pub struct Employee {
    employee: HashMap<String, String>,
}

impl Employee {
    pub fn new() -> Employee {
        Employee { 
            employee: HashMap::new(),
        }
    }

    pub fn add_employee(mut self: Employee) -> Employee {
        println!("add employees by name, department.");
        let name = get_input("Name");
        let department = get_input("Department");
        self.employee.insert(name, department);
        self
    }

    pub fn print_emp_by_dpt(self: &Employee) {
        // read user input for department.
        let depart = get_input("Department");

        // return Vec of names by department.
        let mut namelst = Employee::return_names(&self, &depart);

        // sort vec of names with closure.
        namelst.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        println!("Employees in {}:", depart);

        // print out vector of names.
        for name in namelst {
            println!("{}", name);
        }
    }

    pub fn return_names(&self, deprt: &String) -> Vec<String> {
        let mut names_by_dpt: Vec<String> = vec![];
        // closure for getting a vector of names out of HashMap.
        let mut printer = |emplyee: &HashMap<String, String>| {
            for (k, v) in emplyee.iter() {
                if v == deprt {
                    names_by_dpt.push(k.into());
                }
            }
        };
        // call closure function.
        printer(&self.employee);
        return names_by_dpt;
    }

    // Read employee HashMap useing closure.
    pub fn read_hash(&self) {
        let printer = |hmap| {
            for (k, v) in hmap {
                println!("Employee: {}, Deparment: {}", k, v)
            }
        };
        printer(&self.employee);
    }
}
