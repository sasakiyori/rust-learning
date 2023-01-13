mod compare;
mod display;
mod structure;

fn main() {
    let struct_without_lifecycle = structure::StringWithoutLifecycle {
        content: String::from("abcd"),
    };
    println!(
        "struct_without_lifecycle.content: {}",
        struct_without_lifecycle.content
    );

    let struct_with_lifecycle = structure::StringWithLifecycle { content: "abcd" };
    println!(
        "struct_with_lifecycle.content: {}",
        struct_with_lifecycle.content
    );
    println!(
        "struct_with_lifecycle.get_content_with_lifecycle(): {}",
        struct_with_lifecycle.get_content_with_lifecycle()
    );
    println!(
        "struct_with_lifecycle.get_content_without_lifecycle(): {}",
        struct_with_lifecycle.get_content_without_lifecycle()
    );

    println!("logger string is {}", compare::longer("a", "abc"));

    display::longest_with_an_announcement("a", "abc", display::Announcer {});
}
