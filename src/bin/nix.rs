#![allow(dead_code, unused_variables, unused_imports)]
extern crate tempdir;
extern crate tempfile;
extern crate nix;
extern crate libc;

use nix::fcntl::{OFlag, O_RDONLY, open, openat, readlink, readlinkat};
use nix::sys::stat::Mode;
use nix::unistd::{close, read};
use nix::errno::errno;
use nix::Errno;
use tempdir::TempDir;
use tempfile::NamedTempFile;
use std::io::prelude::*;
use std::os::unix::fs;

// 1. wait for a special time
// 2. execvp
//

use std::ffi::CString;
use libc::*;

fn rperror(s: &str) {
    let cstr = CString::new(s).unwrap();
    let pc = cstr.into_raw();
    unsafe {
        perror(pc);
    }
}

fn fork_child() -> pid_t {
    unsafe {
        let p = fork();
        if p == -1 {
            rperror("fork");
            exit(1);
        }
        if p == 0 {
            println!("child working...");
            sleep(3);
            println!("child exit");
            _exit(0);
        }
        return p;
    }
}

fn main() {
    unsafe {
        let mut mask: sigset_t = std::mem::uninitialized();
        let mut orig_mask: sigset_t = std::mem::uninitialized();
        let timeout: timespec = timespec {
            tv_sec: 4,
            tv_nsec: 0,
        };

        let mask_ptr: *mut sigset_t = &mut mask;
        let orig_mask_ptr: *mut sigset_t = &mut orig_mask;

        sigemptyset(mask_ptr);
        sigaddset(mask_ptr, SIGCHLD);

        if sigprocmask(SIG_BLOCK, mask_ptr, orig_mask_ptr) < 0 {
            rperror("sigprocmask");
            exit(1);
        }

        let pid = fork_child();

        let timeout_ptr: *const timespec = &timeout;

        use std::ptr;

        loop {
            if sigtimedwait(mask_ptr, ptr::null_mut(), timeout_ptr) < 0 {
                if Errno::from_i32(errno()) == Errno::EINTR {
                    continue;
                } else if Errno::from_i32(errno()) == Errno::EAGAIN {
                    println!("timeout, killing child");
                    kill(pid, SIGKILL);
                } else {
                    rperror("sigtimedwait");
                    exit(1);
                }
            }
            break;
        }

        if waitpid(pid, ptr::null_mut(), 0) < 0 {
            rperror("waitpid");
            exit(1);
        }
    }
}
