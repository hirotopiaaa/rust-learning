use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // create the file if it doesn't exist
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // The above code can be written more concisely using closures

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// Error Propagation
// Returning errors back to the caller instead of handling it within the function
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?;

    // The ? operator can only be used in functions that have a return type of Result

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // read_to_string() returns a Result<String, std::io::Error>
    // it reads the contents of the file into a string
    // if the read operation is successful, it returns the string
    // if it encounters an error, it returns an error
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    f.read_to_string(&mut s)?;
    // The ? operator can be used in functions that return Result
    // Ok(s) is returned if the read operation is successful

    // equivalent to:
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
