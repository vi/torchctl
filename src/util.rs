use nix::libc::{c_int, c_void};

pub fn stderr(msg: &str) {
    unsafe {
        let _ = nix::libc::write(2 as c_int, msg as *const str as *const c_void, msg.len());
    };
}

pub fn printerr(e: nix::Error) {
    if let Some(errno) = e.as_errno() {
        stderr(errno.desc());
        stderr("\n");
    } else {
        stderr("ERROR\n");
    }
}
