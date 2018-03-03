
extern crate DECALS_base;
use DECALS_base::{Network};

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod colorscheme;
mod colors;

use colors::{Color, Pallette};
use colorscheme::ColorScheme;


use std::sync::{Arc};




pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64   // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Colors


        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(colors::SPACEBLUE, gl);

            //let transform = c.transform.trans(x, y)
            //                           .rot_rad(rotation)
            //                           .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);

            // Draw a console box at the bottom of the screen.
            rectangle(colors::BLACK, rectangle::square(0.0, 0.0, 50.0), c.transform.trans((args.width / 2) as f64, (args.height - 50) as f64), gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
	let opengl = OpenGL::V3_2;

	let mut window: Window = WindowSettings::new(
            "Starfleet Medical",
            [800, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut interests: Vec<String> = Vec::new();
    interests.push("SysStatus".to_string());

    let net: Arc<Network> = Network::new(interests);

    println!("attached {}", net.get_num_devices());

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
