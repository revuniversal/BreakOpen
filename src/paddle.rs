use update::*;
use draw::*;
use piston::input::*;
use opengl_graphics::GlGraphics;
use graphics::*;

type Color = [f32; 4];
type Point = (f64, f64);

pub struct Paddle {
    pub color: Color,
    pub scale: Point,
    pub pos: Point,
}

impl Paddle {
    pub fn new(color: Color, scale: Point, pos: Point) -> Paddle {
        Paddle {
            color: color, 
            scale: scale,
            pos: pos
        }
    }
}

impl Update for Paddle {
    fn update(&mut self, args: &UpdateArgs) {
        // TODO: handle keyboard input
    } 
}

impl Draw for Paddle {
    fn draw(&mut self, g: &mut GlGraphics, args: &RenderArgs) {
        let (centerX, centerY): Point = get_screen_center(args);
        let square = rectangle::Rectangle::new()
        
        g.draw(args.viewport(), |c, gl| {
        let transform = c.transform
            .trans(centerX, centerY)
            .trans(self.pos.0, self.pos.1)
            .trans((-0.5 * self.scale.0), (-0.5 * self.scale.1));
        rectangle(self.color, square, transform, gl);
        });
    }
}



/// Get the center of the screen
fn get_screen_center(args: &RenderArgs) -> Point {
    ((args.width/2) as f64, (args.height/2) as f64)
}

fn screen_coords(args: &RenderArgs, point: Point) -> Point{
    ((point.0 + args.width / 2.0), (point.1 - args.height / 2.0));
}


/// Get the coordinates for the upper left corner of a rectangle
fn upper_left_coords(position: Point, scale: Point) -> Point {
    ((scale.0 * -0.5) as f64, (scale.1 * -0.5) as f64)
}

/// Get the coordinates for the lower right corner of a rectangle
fn lower_right_coords(position: Point, scale: Point) -> Point {
    ((scale.0 * 0.5) as f64, (scale.1 * 0.5) as f64)
}

fn to_screen_corners(
    args: &RenderArgs, 
    position: Point, 
    scale: Point
) -> (Point, Point) {
    let center = get_screen_center(args);
    let unshifted = (
        (center.0 + position.0) as f64, 
        (center.1 + (position.1 * -1.0) as f64)
    );
    let (x0, y0) = upper_left(unshifted);
    let (x1, y1) = upper_left(unshifted);
    ((x0, y0), (x1, y1))
}

