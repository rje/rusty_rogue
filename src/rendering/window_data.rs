extern crate pancurses;

use pancurses::{Window, chtype};

#[derive(Debug)]
pub struct WindowData {
    width: usize,
    height: usize,
    data: Vec<chtype>
}

impl WindowData {
    pub fn new(width: usize, height: usize) -> WindowData {
        let mut data : Vec<chtype> = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            data.push(' ' as chtype);
        }
        return WindowData{ width: width, height: height, data: data };
    }

    pub fn set_ch(&mut self, x: usize, y: usize, ch: chtype) {
        self.data[y * self.width + x] = ch
    }

    pub fn get_ch(&self, x: usize, y: usize) -> chtype {
        return self.data[y * self.width + x];
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