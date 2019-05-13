#![allow(unused)]
#![warn(unused_must_use)]

const SWITCH : &[u8] = b"/sys/class/leds/led:switch_0/brightness";
const TOR1 : &[u8] = b"/sys/class/leds/led:torch_0/brightness";
const TOR2 : &[u8] = b"/sys/class/leds/led:torch_1/brightness";

static MODES : &'static [&'static dyn TorchMode] = &[
    &JustWriteValsToSysfs(Off),
    &JustWriteValsToSysfs(Dim),
    &JustWriteValsToSysfs(Bright),
];

use nix::Result;
use crate::util::stderr;

/// init called when entering a mode, term when exiting it
trait TorchMode : Send + Sync {
    fn init(&self) -> Result<()>;
    fn term(&self) -> Result<()>;
}


/// Values to use for WriteVal
trait BrightnessSettings : Default + Send + Sync {
    const BR1: &'static [u8];
    const BR2: &'static [u8];
    const SW: &'static [u8];
}

macro_rules! declare_brightness_settings {
    (name=$name:ident, BR1=$br1:expr, BR2=$br2:expr, SW=$sw:expr,) => {
        #[derive(Default)] struct $name;
        impl BrightnessSettings for $name {
            const BR1 : &'static [u8] = $br1;
            const BR2 : &'static [u8] = $br2;
            const SW : &'static [u8] = $sw;
        }
    }
}

declare_brightness_settings! {
    name=Off,
    BR1=b"0",
    BR2=b"0",
    SW=b"0",
}
declare_brightness_settings! {
    name=Dim,
    BR1=b"1",
    BR2=b"0",
    SW=b"1",
}

declare_brightness_settings! {
    name=Bright,
    BR1=b"100",
    BR2=b"100",
    SW=b"1",
}

/// Torch configuration that just writes some values to sysfs
#[derive(Default)]
struct JustWriteValsToSysfs<S:BrightnessSettings>(S);


#[path="impls.rs"]
mod impls;



pub struct Torch {
    state: usize,
}

pub enum Adjust {
    Up,
    Down,
}

impl Torch {
    pub fn new() -> Torch {
        Torch {
            state: 0,
        }
    }
    pub fn adjust(&mut self, d: Adjust) -> Result<()> {
        let newstate = match d {
            Adjust::Up => {
               (self.state + 1).min(MODES.len() - 1)
            },
            Adjust::Down => {
                self.state.saturating_sub(1)
            },
        };
        if self.state == newstate {
            stderr("NO CHANGE\n");
            Ok(())
        } else {
            unsafe{MODES.get_unchecked(self.state)}.term()?;
            self.state = newstate;
            unsafe{MODES.get_unchecked(self.state)}.init()?;
            Ok(())
        }
    }
}
