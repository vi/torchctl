const WAKE_LOCK: &[u8] = b"/sys/power/wake_lock";
const WAKE_UNLOCK: &[u8] = b"/sys/power/wake_unlock";
const WAKE_LOCK_NAME: &[u8] = b"torchctl";

use nix::fcntl::{open, OFlag};
use nix::sys::signal::{kill, Signal};
use nix::sys::stat::Mode;
use nix::unistd::{close, fork, write, ForkResult, Pid};
//use nix::sys::wait::waitpid;

use super::*;

fn writefile(file: &[u8], content: &[u8]) -> Result<()> {
    let f = open(file, OFlag::O_WRONLY, Mode::empty())?;
    write(f, content)?;
    close(f)?;
    Ok(())
}

impl<S: BrightnessSettings> TorchMode for JustWriteValsToSysfs<S> {
    fn init(&self, _: &mut GlobalState) -> Result<()> {
        writefile(SWITCH, b"0")?;
        writefile(TOR1, S::BR1)?;
        writefile(TOR2, S::BR2)?;
        writefile(SWITCH, S::SW)?;
        Ok(())
    }
    fn term(&self, _: &mut GlobalState) -> Result<()> {
        Ok(())
    }
}

#[derive(Default)]
pub struct GlobalState {
    pid: Option<Pid>,
}

impl TorchMode for UltraDimPWM {
    fn init(&self, gs: &mut GlobalState) -> Result<()> {
        match fork()? {
            ForkResult::Parent { child } => {
                gs.pid = Some(child);
            }
            ForkResult::Child => {
                use nix::libc::sched_param;
                use nix::libc::sched_setscheduler;
                //use nix::libc::SCHED_FIFO;
                const SCHED_FIFO: nix::libc::c_int = 1;

                let param = sched_param { sched_priority: 10 };
                unsafe {
                    sched_setscheduler(0, SCHED_FIFO, &param);
                }

                writefile(WAKE_LOCK, WAKE_LOCK_NAME)?;

                //let sleeper = spin_sleep::SpinSleeper::default();

                loop {
                    writefile(SWITCH, b"0")?;
                    let mut sl = nix::libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 5_000_000,
                    };
                    unsafe {
                        nix::libc::nanosleep(&sl, &mut sl);
                    }
                    //sleeper.sleep_ns(5_000_000);
                    writefile(TOR1, b"0")?;
                    writefile(TOR2, b"1")?;
                    writefile(SWITCH, b"1")?;
                    let mut sl = nix::libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 500_000,
                    };
                    unsafe {
                        nix::libc::nanosleep(&sl, &mut sl);
                    }
                    //sleeper.sleep_ns(500_000);
                }

                #[allow(unreachable_code)]
                {
                    unsafe { nix::libc::_exit(0) };
                }
            }
        }

        Ok(())
    }
    fn term(&self, gs: &mut GlobalState) -> Result<()> {
        if let Some(pid) = gs.pid {
            kill(pid, Signal::SIGTERM)?;
            //waitpid(pid, None)?;  //causes inclusion of panic/formatting code
            let mut status = 0;
            unsafe {
                nix::libc::waitpid(pid.as_raw(), &mut status, 0);
            }
            writefile(WAKE_UNLOCK, WAKE_LOCK_NAME)?;
            gs.pid = None;
        }
        Ok(())
    }
}
