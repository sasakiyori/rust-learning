use std::io::{Error, Read, Seek, SeekFrom, Write};
use tempfile::tempfile;

fn write_tmp_file() -> Result<bool, Error> {
    let mut file = tempfile()?;
    let mut size = file.write(b"abcs")?;
    println!("write size: {}", size);
    let pos = file.seek(SeekFrom::Start(0))?;
    println!("seek pos: {}", pos);

    let mut buf = String::new();
    size = file.read_to_string(&mut buf)?;
    println!("read size: {}", size);
    println!("read buf: {}", buf);
    Ok(true)
}

fn main() {
    let res = write_tmp_file();
    match res {
        Ok(tf) => {
            println!("true or false: {}", tf);
        }
        Err(e) => {
            println!("error {}", e)
        }
    }
}
