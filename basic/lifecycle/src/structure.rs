pub struct StringWithoutLifecycle {
    pub content: String,
}

// this struct is invalid
// error message: missing lifetime specifier, expected named lifetime parameter

// struct StringWithoutLifecycle {
//     content: &str
// }

pub struct StringWithLifecycle<'swlf> {
    pub content: &'swlf str,
}

impl<'swlf> StringWithLifecycle<'swlf> {
    pub fn get_content_with_lifecycle(&self) -> &'swlf str {
        self.content
    }
    // return value can omit lifecycle mark
    pub fn get_content_without_lifecycle(&self) -> &str {
        self.content
    }
}
