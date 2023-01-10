pub mod error;


pub fn error_processor(res : Result<i32, bool>) -> Result<i32, bool> {
    let x = res?;
    Ok(x)
}