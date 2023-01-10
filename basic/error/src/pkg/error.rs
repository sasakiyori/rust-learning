pub fn trouble_maker(i : i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}

pub fn error_processor(res : Result<i32, bool>) -> Result<i32, bool> {
    let x = res?;
    Ok(x)
}