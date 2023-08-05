use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    let mut original = String::from("original value");
    println!("outer scope original: {}", original);

    {
        print_original(&original);
        change_original(&mut original);
        println!("inner scope original after change_original: {}", original);
    }

    let name = "Duck Airlines";
    let write_message = |slogan: String| -> String {
        println!("This is a closure.");
        format!("{}: {}", name, slogan)
    };

    let msg = write_message(String::from("We land everytime."));
    println!("{}", msg);

    let filename = "~/etc/somefile";
    match File::open(filename) {
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(_) => {
                    println!("File created.");
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            },
            _ => {
                println!("{:#?}", error);
            }
        },
    }

    let file_date = read_file(filename);
    match file_date {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {
            println!("Error occurred");
        }
    }
}

fn print_original(original: &String) {
    println!("fn print_original: {}", original);
}

fn change_original(original: &mut String) {
    let next = original;
    *next = String::from("new value");

    println!("fn change_original next: {}", next)
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    return Ok(file_data);
}
