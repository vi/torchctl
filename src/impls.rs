use nix::fcntl::{
    open,
    OFlag,
};
use nix::sys::stat::Mode;
use nix::unistd::{
    close,
    write,
};

use super::*;

fn writefile(file:&[u8], content:&[u8]) -> Result<()> {
    let f = open(file, OFlag::O_WRONLY, Mode::empty() )?;
    write(f, content)?;
    close(f)?;
    Ok(())
}

impl<S:BrightnessSettings> TorchMode for JustWriteValsToSysfs<S> {
    fn init(&self) -> Result<()> {
        writefile(SWITCH, b"0")?;
        writefile(TOR1, S::BR1)?;
        writefile(TOR2, S::BR2)?;
        writefile(SWITCH, S::SW)?;
        Ok(())
    }
    fn term(&self) -> Result<()> {
        Ok(())
    }
}
