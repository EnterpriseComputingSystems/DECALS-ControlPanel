

use colors::{Color, Pallette};

use rand;


pub trait ColorScheme {
    fn get_next_color(&mut self)->Color;
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
    fn get_next_color(&mut self)->Color {
        return self.get_num_color(rand::random::<usize>() % self.pallette.len())
    }

    fn get_num_color(&self, num: usize)->Color {
        return self.pallette[num];
    }
}

//~~~~~~~~~~~~~~Weighted Random~~~~~~~~~~~~~~~~~~~~~~


pub struct WeightedRandScheme {
    pallette: Pallette
}

//~~~~~~~~~~~~~~Iterated~~~~~~~~~~~~~~~~~~~~~~

pub struct IteratedScheme {
    idx: usize,
    pallette: Pallette
}

impl IteratedScheme {
    pub fn new(pallette: Pallette)->IteratedScheme {
        return IteratedScheme{idx: pallette.len() + 1, pallette};
    }
}

impl ColorScheme for IteratedScheme {
    fn get_next_color(&mut self)->Color {
        self.idx = (self.idx + 1) % self.pallette.len();
        return self.get_num_color(self.idx);
    }

    fn get_num_color(&self, num: usize)->Color {
        return self.pallette[num];
    }
}

pub struct SimilarityScheme {

}
