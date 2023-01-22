use std::io;

// get user input useing io::stdin.
pub fn get_input(mes: &str) -> String {
    let mut user_input = String::new();

    // dynamic message passed in from funtion call.
    println!("please enter a {}.", mes);

    // read user input as string.
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line.");
    user_input = user_input.trim().to_string();
    // return user input.
    user_input
}