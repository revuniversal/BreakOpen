extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod draw;
mod update;
mod paddle;
mod block;
mod screen;

use draw::*;
use update::*;
use block::*;
use screen::*;
use paddle::*;

type Color = [f32; 4];
type Point = (f64, f64);

fn main() {
    const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
    const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
    const RED:   Color = [1.0, 0.0, 0.0, 1.0];
    const BLUE:  Color = [0.0, 0.0, 1.0, 1.0];
    
    let opengl = OpenGL::V3_2;
    let mut events = Events::new(EventSettings::new());
    let mut window: Window = WindowSettings::new(
            "Spinning Square",
            [800, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut g = GlGraphics::new(opengl);
    let mut screen = Screen {
        color: BLACK, 
        paddle: Paddle::new(BLUE, (80., 16.), (0.0, -50.0)),
        blocks: vec![
            Block::new(RED, 10.0, [-50.0, 0.0], [0.0, 0.0], [0.0, 0.0]),
            Block::new(GREEN, 2.0, [0.0, 0.0], [25.0, -13.0], [-10.0, 1.0]),
            Block::new(BLUE, -2.0, [50.0, 0.0], [10.0, 20.0], [-3.0, -6.0])
        ]
    };

    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            screen.update(&u);
        }
        
        if let Some(r) = e.render_args() {
            screen.draw(&mut g, &r);
        }
    }
}
