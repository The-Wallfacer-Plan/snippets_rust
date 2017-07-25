extern crate libc;
extern crate errno;
use libc::{c_int, pid_t, c_void, size_t, ssize_t, pipe, fork, dup2, close, execv, read};
use std::env::{set_var};
use std::ffi::CString;

const FORKSRV_FD: c_int = 198;
static PIPE_ENV_VAR: &'static str= "__AFL_PIPE_ID";

fn main() {
    unsafe {
        let mut st_pipe : [c_int; 2] = [0; 2];
        let mut ctl_pipe : [c_int; 2] = [0; 2];
        let status : *mut c_int = &mut -32;
        // let exec_path : [c_char; 15] = "examples/pngfix";
        println!("spinning up the fork server...");
        if (! pipe(st_pipe.as_mut_ptr()) == 0 ) || (! pipe(ctl_pipe.as_mut_ptr()) == 0) {
            panic!("pipe failed!");
        }
        let forksrv_pid: pid_t = fork();
        println!("forksrv_pid is {}", forksrv_pid);
        if forksrv_pid < 0 {
            panic!("fork() failed!");
        }

        // code under this if will run in the child
        if forksrv_pid == 0 {
            let ctl_fd = FORKSRV_FD + 2;
            set_var(PIPE_ENV_VAR, ctl_fd.to_string());
            let new_ctl_pipe = dup2(ctl_pipe[0], ctl_fd);
            let new_st_pipe = dup2(st_pipe[1], ctl_fd + 1);
            if new_ctl_pipe < 0 {
                panic!("dup2 failed! ALERT: errno={}", errno::errno());
            }
            if new_st_pipe < 0 {
                panic!("dup2 failed! ALERT: errno={}", errno::errno());
            }
            println!("new_ctl_pipe is {}", new_ctl_pipe);
            println!("new_st_pipe is {}", new_st_pipe);
            close(ctl_pipe[0]);
            close(ctl_pipe[1]);
            close(st_pipe[0]);
            close(st_pipe[1]);
            let argv = vec![CString::new("examples/main").unwrap().as_ptr()];
            execv(CString::new("examples/main").unwrap().as_ptr(), argv.as_ptr());
        }

        close(ctl_pipe[0]);
        close(st_pipe[1]);

        let fsrv_st_fd = st_pipe[0];

        let rlen : ssize_t = read(fsrv_st_fd, status as *mut c_void, 4 as size_t);
        
        if rlen == 4 {
            println!("forkserver set up correctly!");
        }
        else {
            println!("forserver didn't setup correctly!
                \nrlen is {}, status is {}, fsrv_st_fd is {}, writer end is {}", rlen, *status, fsrv_st_fd, st_pipe[1]);
            panic!("ALERT: errno={}", errno::errno());
        }

    }
    
}