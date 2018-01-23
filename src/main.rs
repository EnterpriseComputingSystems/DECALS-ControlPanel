
extern crate DECALS_base;
use DECALS_base::{Network};

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


fn main() {

    let mut interests: Vec<String> = Vec::new();
    interests.push("SysStatus".to_string());

    let net: Network = Network::new(interests);

    println!("{}", net.get_num_devices());
}
