use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello1.txt");

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
