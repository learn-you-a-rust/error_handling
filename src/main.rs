use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;

// custom type for error handling where user should input an integer 1 to 100;
// essentially outsources error handling so it doesn't clutter the code
pub struct Guess {
    // private field because this way outside code has to go through the check
    // in the constructor below
    value: i32,
}

impl Guess {
    // validation occurs in the constructor for this type
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    // "getter": returns value as i32
    pub fn value(&self) -> i32 {
        self.value
    }
}


// this function propagates any errors back to the calling code
fn read_username_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        // store file handle in f
        Ok(file) => file,
        // passes an error value from File::open back to calling code
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        // return username
        Ok(_) => Ok(s),
        // or return error (implicit return because is last expression)
        Err(e) => Err(e),
    }
}

// this is the same but shorter
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// now even shorter!!!! 
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// REALLY FUCKING SHORT!!!!
fn read_username_from_file4() -> Result<String, io::Error> {
    // fs just does all the error handling for us
    fs::read_to_string("hello.txt")
}

// you can return this result type to mean `either nothing, or 
// any kind of error`
fn some_other_function() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

fn main() {
    //panic!("crash and burn");
    
    //let v = vec![1, 2, 3];
    //v[99];

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error)
        },
    };

    // alternative, same as above with closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    // unwrap in action
    let f = File::open("hello.txt").unwrap();

    // use expect to choose a panic! message
    let f = File::open("hello3.txt").expect("Failed to open hello3.txt");
}
