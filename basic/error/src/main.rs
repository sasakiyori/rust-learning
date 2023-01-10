mod pkg;

fn main() {
    let a = pkg::error::trouble_maker(1);
    let b = pkg::error::error_processor(a);
    let c = pkg::error_processor(a);
    match a {
        Ok(v) => {
            println!("get value {}", v);
        }
        Err(e) => {
            println!("get error {}", e);
        }
    }

    if let Ok(vv) = b {
        println!("get value {}", vv);
    } else {
        println!("get error");
    }

    if let Err(err) = c {
        println!("get error {}", err);
    }
}
