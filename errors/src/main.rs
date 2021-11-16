use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => println!("File opened successfully"),
        Err(error) => println!("Problem opening the file"),
    };
}
