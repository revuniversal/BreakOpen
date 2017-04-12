use piston::input::*;

pub trait Update {
    fn update(&mut self, args: &UpdateArgs) -> ();
}
