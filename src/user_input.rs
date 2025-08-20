use std::io;

pub fn get_user_input() -> String {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    input_string
}
