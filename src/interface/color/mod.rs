
pub mod colors;

use rand;

use std::boxed::Box;

use DECALS_base::support::alert::Alert;

use self::colors::{Color, Pallette};


pub struct ColorScheme {
    idx: usize,
    start_idx: usize,
    pallette: Box<Vec<Color>>
}

impl ColorScheme {
    pub fn new(pallette: Pallette)->ColorScheme {
        ColorScheme{idx: 0, start_idx: 0, pallette: Box::new(pallette)}
    }

    pub fn randomize(pallette: Pallette)->ColorScheme {
        let scheme = Self::new(pallette);
        scheme.get_num_color(rand::random::<usize>() % scheme.pallette.len());

        return scheme;
    }

    pub fn rotate_start(&mut self) {
        self.rotate_start_by(1);
    }

    pub fn rotate_start_by(&mut self, count: usize) {
        self.start_idx = (self.start_idx + 1) % self.pallette.len();
    }

    pub fn reset_to_start(&mut self) {
        self.idx = self.start_idx;
    }

    pub fn set_start_default(&mut self) {
        self.start_idx = 0;
    }

    pub fn get_next_color(&mut self)->Color {
        let next = self.get_num_color(self.idx);
        self.idx = (self.idx + 1) % self.pallette.len();
        return next;
    }

    pub fn get_num_color(&self, num: usize)->Color {
        self.pallette[num]
    }
}

pub fn get_suggested_colorscheme(al: Alert) ->ColorScheme {
    match al {
        Alert::Normal=> ColorScheme::new(colors::NO_ALERT.to_vec()),
        Alert::Yellow=> ColorScheme::new(colors::YELLOW_ALERT.to_vec()),
        Alert::Blue=> ColorScheme::new(colors::BLUE_ALERT.to_vec()),
        Alert::Black=> ColorScheme::new(colors::BLUE_ALERT.to_vec()),
        Alert::Red=> ColorScheme::new(colors::RED_ALERT.to_vec())
    }

}
