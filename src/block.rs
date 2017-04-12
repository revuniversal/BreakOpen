use update::*;
use draw::*;
use piston::input::*;
use opengl_graphics::GlGraphics;

type Color = [f32; 4];
type Point = [f64; 2];

pub struct Block {
    pub color: Color,
    pub rotVel: f64,
    pub rot: f64,
    pub pos: Point,
    pub vel: Point,
    pub acc: Point
}
impl Block {
    pub fn new(color: Color, rotVel: f64, pos: Point, vel: Point, acc: Point) -> Block {
        Block {
            color: color, 
            rotVel: rotVel,
            rot: 0.0,
            pos: pos,
            vel: vel,
            acc: acc
        }
    }
}
impl Update for Block {
    fn update(&mut self, args: &UpdateArgs) {
        self.vel[0] += self.acc[0] * args.dt;
        self.vel[1] += self.acc[1] * args.dt;
        self.pos[0] += self.vel[0] * args.dt;
        self.pos[1] += self.vel[1] * args.dt;
        self.rot += self.rotVel * args.dt;
    } 
}
impl Draw for Block {
    fn draw(&mut self, g: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        
        g.draw(args.viewport(), |c, gl| {
            let transform = c.transform
                .trans(x, y)
                .trans(self.pos[0], self.pos[1])
                .rot_rad(self.rot)
                .trans(-25.0, -25.0);
            rectangle(self.color, square, transform, gl);
        });
    }
}