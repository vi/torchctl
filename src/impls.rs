use nix::fcntl::{
    open,
    OFlag,
};
use nix::sys::stat::Mode;
use nix::unistd::{
    close,
    write,
    fork,
    Pid,
    ForkResult,
};
use nix::sys::signal::{
    kill,
    Signal,
};
use nix::sys::wait::waitpid;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;

fn writefile(file:&[u8], content:&[u8]) -> Result<()> {
    let f = open(file, OFlag::O_WRONLY, Mode::empty() )?;
    write(f, content)?;
    close(f)?;
    Ok(())
}

impl<S:BrightnessSettings> TorchMode for JustWriteValsToSysfs<S> {
    fn init(&self, _:&mut GlobalState) -> Result<()> {
        writefile(SWITCH, b"0")?;
        writefile(TOR1, S::BR1)?;
        writefile(TOR2, S::BR2)?;
        writefile(SWITCH, S::SW)?;
        Ok(())
    }
    fn term(&self, _:&mut GlobalState) -> Result<()> {
        Ok(())
    }
}

#[derive(Default)]
pub struct GlobalState {
    pid: Option<Pid>,
}

impl TorchMode for UltraDimPWM {
    fn init(&self, gs:&mut GlobalState) -> Result<()> {
        
        match fork()? {
            ForkResult::Parent{child} => {
                gs.pid = Some(child);
            },
            ForkResult::Child => {
                writefile(SWITCH, b"0")?;
                writefile(TOR1, b"0")?;
                writefile(TOR2, b"1")?;
                writefile(SWITCH, b"1")?;
                unsafe { nix::libc::_exit(0) };
            },
        }
        
        Ok(())
    }
    fn term(&self, gs:&mut GlobalState) -> Result<()> {
        if let Some(pid) = gs.pid {
            kill(pid, Signal::SIGTERM)?;
            //waitpid(pid, None)?;  //causes inclusion of panic/formatting code
            let mut status = 0;
            unsafe { nix::libc::waitpid(
                pid.as_raw(),
                &mut status,
                0,
            ); }
            gs.pid = None;
        }
        Ok(())
    }
}
