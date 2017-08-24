extern crate pancurses;

use pancurses::*;

#[derive(Debug)]
struct WindowData {
    width: i32,
    height: i32,
    data: Vec<chtype>
}

impl WindowData {
    pub fn set_ch(&mut self, x: i32, y: i32, ch: chtype) {
        self.data[(y * self.width + x) as usize] = ch
    }

    pub fn get_ch(&self, x: i32, y: i32) -> chtype {
        return self.data[(y * self.width + x) as usize];
    }

    pub fn draw(&self, win: &Window) {
        win.clear();
        win.mv(0, 0);
        for y in 0..self.height {
            for x in 0..self.width {
                win.addch(self.get_ch(x, y));
            }
        }
        win.refresh();
    }
}

fn main() {
    let window = initscr();
    let mut data = WindowData { width: 80, height: 24, data: vec![' ' as chtype; 80 * 24]};
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

    window.mv(12, 40);
    window.addch('#');
    window.getch();
    endwin();
}