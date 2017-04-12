extern crate graphics;

use piston::input::*;
use graphics::clear;
use opengl_graphics::GlGraphics;

use update::*;
use draw::*;
use block::*;
use paddle::*;

type Color = [f32;4];

// pub struct Paddle {
//     scale: (f64, f64),
//     pos: (f64, f64)
// }

// pub struct Ball {
//     speed: f64,
//     radius: f64,
//     pos: (f64, f64)
// }

// pub struct Block {
//     scale: (f64, f64),
//     pos: (f64, f64)
// }

// pub struct Zone {
//     pos: (f64, f64),
//     scale: (f64, f64),
//     color: Color
// }

// pub struct border {
//     pos: (f64, f64),
//     scale: (f64, f64),
//     color: Color
// }

pub struct Screen {
    pub color: Color,
    pub blocks: Vec<Block>,
    pub paddle: Paddle,
    // pub ball: Ball,
    // pub topBorder: Border,
    // pub leftBorder: Border,
    // pub bottomBorder: Border,
    // pub outOfBounds: Zone
}

impl Update for Screen {
    fn update(&mut self, args: &UpdateArgs) {
        self.paddle.update(args);
        
        for ref mut block in &mut self.blocks {
            block.update(&args);
        }
    }
}

impl Draw for Screen {
    fn draw(&mut self, g: &mut GlGraphics, args: &RenderArgs) {
        g.draw(args.viewport(), |_, g| clear(self.color, g));
        self.paddle.draw(g, &args);

        for block in &mut self.blocks.iter_mut() {
           block.draw(g, &args);
        }     
    }
}