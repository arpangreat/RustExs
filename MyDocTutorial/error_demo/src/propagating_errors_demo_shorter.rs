use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello1.txt")?;
    f.read_to_string(&mut s)?;
    Ok(s)
}
