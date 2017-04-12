use opengl_graphics::GlGraphics;
use piston::input::*;

pub trait Draw {
    fn draw(&mut self, g: &mut GlGraphics, args: &RenderArgs) -> ();
}
