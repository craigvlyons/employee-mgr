use user_input::get_input;
use employed::Employee;
mod user_input;
mod employed;

fn main() {
    let mut employ = Employee::new();
    loop {
        let message = String::from(
            "please enter an option. 
        q to quit.
        r to read employees.
        a to add employees.
        d to check employees by department.",
        );

        let select = get_input(message.as_str());

        match select.as_str() {
            "q" => break,
            "r" => Employee::read_hash(&employ),
            "d" => Employee::print_emp_by_dpt(&employ),
            "a" => employ = Employee::add_employee(employ),
            _ => println!("command not reconized."),
        }        
    }    
}


