use std::fs::File;
use std::io::{self,ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(_file) => println!("File opened successfully"),
        Err(_error) => println!("Problem opening the file"),
    };

    // now testing with trying to create the file if it doesn't exist

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // The same as above but, more concise
    let f = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // now using unwrap helper function
    let f = File::open("hello2.txt").unwrap();

    // expect do the same as unwrap but with a custom error message
    let f = File::open("hello2.txt").expect("Failed to open hello.txt");

    let f = read_username_from_file().expect("Failed to read username");

    let f = read_username_from_file_other_file().expect("process the username");

}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello2.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// same functionality as above, but using the ? operator
fn read_username_from_file_other_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
