#![no_main]

const SOCK: &[u8] = b"torchctl";

use strum_macros::EnumString;

use nix::libc::{c_char, c_int, strlen};
use nix::sys::socket::{
    accept, bind, connect, listen, recv, send, shutdown, socket, AddressFamily, MsgFlags, Shutdown,
    SockAddr, SockFlag, SockType, UnixAddr, setsockopt
};
use nix::sys::socket::sockopt::ReceiveTimeout;
use nix::sys::time::{TimeValLike};
use nix::Result;

mod torch;
mod util;

use std::slice::from_raw_parts;
use util::{printerr, stderr};

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
enum Cmd {
    Serve,
    Up,
    Down,
    Quit,
}

fn getcmd(argc: c_int, argv: *mut *mut c_char) -> Option<Cmd> {
    if argc != 2 {
        return None;
    }
    if argv.is_null() {
        return None;
    }
    unsafe {
        let argv = from_raw_parts(argv, 2);
        if argv[0].is_null() || argv[1].is_null() {
            return None;
        }
        let l = strlen(argv[1]);
        let cmd = argv[1] as *const u8;
        let cmd = from_raw_parts(cmd, l);
        let cmd = std::str::from_utf8_unchecked(cmd);
        cmd.parse().ok()
    }
}

fn serve() -> Result<()> {
    let s = socket(
        AddressFamily::Unix,
        SockType::SeqPacket,
        SockFlag::SOCK_CLOEXEC,
        None,
    )?;
    bind(s, &SockAddr::Unix(UnixAddr::new_abstract(SOCK)?))?;
    listen(s, 1)?;

    let mut m = torch::Torch::new();

    loop {
        let ret = accept(s);
        if ret == Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) {
            stderr("TIMEOUT\n");
            m.time_passed()?;
            continue;
        }
        let c = ret?;
        shutdown(s, Shutdown::Write)?;

        let mut buf = [0u8; 4];
        let l = recv(c, &mut buf[..], MsgFlags::empty())?;
        let buf = unsafe { buf[..].get_unchecked(..l) };

        
        let ret = match buf {
            b"up" => {
                stderr("UP\n");
                m.adjust(torch::Adjust::Up)
            },
            b"down" => {
                stderr("DOWN\n");
                m.adjust(torch::Adjust::Down)
            },
            b"quit" => {
                stderr("QUIT\n");
                break;
            },
            _ => {
                stderr("Invalid control packet\n");
                Ok(torch::NeedTimeout::No)
            }
        };
        match ret {
            Err(e) => printerr(e),
            Ok(t) => match t {
                torch::NeedTimeout::No => {
                    setsockopt(s, ReceiveTimeout, &TimeValLike::seconds(
                        0,
                    ))?;
                },
                torch::NeedTimeout::Yes => {
                    setsockopt(s, ReceiveTimeout, &TimeValLike::seconds(
                        torch::FALLBACK_FROM_VERY_BRIGHT_SECONDS,
                    ))?;
                },
            }
        }
    }
    return Ok(())
}

fn notify(cmd: Cmd) -> Result<()> {
    let s = socket(
        AddressFamily::Unix,
        SockType::SeqPacket,
        SockFlag::SOCK_CLOEXEC,
        None,
    )?;
    connect(s, &SockAddr::Unix(UnixAddr::new_abstract(SOCK)?))?;

    let buf: &[u8] = match cmd {
        Cmd::Up => b"up",
        Cmd::Down => b"down",
        Cmd::Quit => b"quit",
        _ => unsafe { std::hint::unreachable_unchecked() },
    };

    send(s, buf, MsgFlags::empty())?;

    Ok(())
}

fn run(cmd: Cmd) -> Result<()> {
    match cmd {
        Cmd::Serve => serve(),
        Cmd::Up | Cmd::Down | Cmd::Quit => notify(cmd),
    }
}

#[no_mangle]
pub extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let cmd = if let Some(x) = getcmd(argc, argv) {
        x
    } else {
        stderr("Usage: torchctl {serve|up|down}\n");
        return 1;
    };

    if let Err(e) = run(cmd) {
        printerr(e);
        1
    } else {
        0
    }
}
