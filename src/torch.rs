#![allow(unused)]
#![warn(unused_must_use)]

use nix::Result;

pub struct Torch {}

pub enum Adjust {
    Up,
    Down,
}

impl Torch {
    pub fn new() -> Torch {
        Torch {}
    }
    pub fn adjust(&mut self, d: Adjust) -> Result<()> {
        Ok(())
    }
}
