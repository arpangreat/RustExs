use std::fs::File;
use std::io::ErrorKind;

pub fn run() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },

            other_error => {
                panic!("Problem openning the file {:?}", other_error);
            }
        },
    };
}

/*
 *
 * pub fn run() (
 * let f = File::open("hello.txt").unwrap_or_else( |error| {
 *      if error.kind() == ErrorKind::NotFound {
 *      File::create("hello.txt").unwrap_or_else(|error| (
 *      panic!("Problem creating the file {:?}", error);
 *      ))
 *      } else {
 *          panic!("Problem openning the file {:?}", error);
 *      }
 * });
 *
 *
 * )
 *
 *
 *
 *
 */
