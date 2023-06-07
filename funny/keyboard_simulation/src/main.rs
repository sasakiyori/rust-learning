use enigo::{Enigo, KeyboardControllable, MouseButton, MouseControllable};
use opencv::highgui::*;
use opencv::prelude::*;
use opencv::viz::get_window_by_name;

fn main() {
    let rect = get_window_image_rect("iTerm");
    println!("rect is : {:?}", rect);
    // let res = move_window("QQ", 100, 100);
    // match res {
    //     Ok(_) => println!("ok"),
    //     Err(e) => println!("{}", e),
    // }

    let res = get_window_by_name("Docker Desktop");
    match res {
        Ok(v3d) => {
            println!("{:?}", v3d.get_window_size());
            println!("{:?}", v3d.get_screenshot());
            println!("{:?}", v3d.get_window_name());
        }
        Err(e) => println!("{}", e),
    }
}

fn enigo() {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(800, 800);
    enigo.mouse_click(MouseButton::Left);
    //enigo.mouse_scroll_x(100);
    enigo.mouse_scroll_y(100);
    //enigo.mouse_click(MouseButton::Right);
    // enigo.key_click();
    //enigo.key_sequence_parse("{+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}");
}
