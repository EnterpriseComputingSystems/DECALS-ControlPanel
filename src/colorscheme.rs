

use colors::{Color, Pallette};

use rand;


pub trait ColorScheme {
    fn get_next_color(&self)->Color;
    fn get_num_color(&self, num: usize)->Color;
}



//~~~~~~~~~~~~~~Random~~~~~~~~~~~~~~~~~~~~~~
pub struct RandScheme {
    pallette: Pallette
}

impl RandScheme {
    pub fn new(pallette: Pallette)->RandScheme {
        return RandScheme{pallette};
    }
}

impl ColorScheme for RandScheme {
    fn get_next_color(&self)->Color {
        self.get_num_color(rand::random::<usize>() % self.pallette.len())
    }

    fn get_num_color(&self, num: usize)->Color {
        return self.pallette[num];
    }
}

//~~~~~~~~~~~~~~Weighted Random~~~~~~~~~~~~~~~~~~~~~~


pub struct WeightedRandScheme {
    pallette: Pallette
}

pub struct IteratedScheme {
    idx: u8,
    pallette: Pallette
}

pub struct GraphScheme {

}
