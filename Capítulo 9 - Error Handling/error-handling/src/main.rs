use std::{
    fs::File,
    io::{self, Read},
};
fn main() {
    // Recoverable Errors with Result
    let greeting_file_result = File::open("hello.txt");

    //Shortcuts for Panic on Error: unwrap and expect:
    // let greeting_file=File::open("hello.txt").unwrap();

    // let greeting_file =
    //File::open("hello.txt").expect("The file 'hello.txt' should be inclued in this project");

    //Propagating Errors:
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
//A Shortcut for Propagating Errors: The '?' Operator:
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    Ok(username)
}
