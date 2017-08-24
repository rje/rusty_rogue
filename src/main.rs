extern crate pancurses;
use pancurses::*;

mod rendering;
use rendering::*;

fn main() {
    let window = initscr();
    let mut data = WindowData::new(80,24);
    cbreak();
    window.keypad(true);
    noecho();
    resize_term(24, 80);

    for y in 0..24 {
        for x in 0..80 {
            data.set_ch(x, y, (x+45) as chtype);
        }
    }

    data.draw(&window);

    window.getch();
    endwin();
}