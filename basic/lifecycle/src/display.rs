use std::fmt::write;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct Announcer {}
impl Display for Announcer {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let mut output = String::new();
        write(&mut output, format_args!("Hello {}!", "world"))
    }
}
