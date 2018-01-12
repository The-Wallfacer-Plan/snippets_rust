extern crate errno;
extern crate libc;
use libc::{c_int, c_void, close, execv, fork, pid_t, pipe, read, size_t, ssize_t, dup2};
use std::env::set_var;
use std::ffi::CString;
use std::thread;

const FORKSRV_FD: c_int = 198;
static PIPE_ENV_VAR: &'static str = "__AFL_PIPE_ID";

fn main() {
    let mut threads = vec![];
    for i in 0..4 {
        // println!("Spawning thread {}", i);
        threads.push(thread::spawn(move || {
            unsafe {
                let mut st_pipe: [c_int; 2] = [0; 2];
                let mut ctl_pipe: [c_int; 2] = [0; 2];
                let status: *mut c_int = &mut -32;
                // let exec_path : [c_char; 15] = "examples/pngfix";
                println!("Thread{}: spinning up the fork server...", i);
                if (!pipe(st_pipe.as_mut_ptr()) == 0) || (!pipe(ctl_pipe.as_mut_ptr()) == 0) {
                    panic!("Thread{}: pipe failed!", i);
                }
                let forksrv_pid: pid_t = fork();
                println!("Thread{}: forksrv_pid is {}", i, forksrv_pid);
                if forksrv_pid < 0 {
                    panic!("Thread{}: fork() failed!", i);
                }

                // code under this if will run in the children
                if forksrv_pid == 0 {
                    let ctl_fd = FORKSRV_FD + 2 * i;
                    set_var(PIPE_ENV_VAR, ctl_fd.to_string());
                    let new_ctl_pipe = dup2(ctl_pipe[0], ctl_fd);
                    let new_st_pipe = dup2(st_pipe[1], ctl_fd + 1);
                    if new_ctl_pipe < 0 {
                        panic!("Thread{}: dup2 failed! ALERT: errno={}", i, errno::errno());
                    }
                    if new_st_pipe < 0 {
                        panic!("Thread{}: dup2 failed! ALERT: errno={}", i, errno::errno());
                    }
                    println!("Thread{}: new_ctl_pipe is {}", i, new_ctl_pipe);
                    println!("Thread{}: new_st_pipe is {}", i, new_st_pipe);
                    close(ctl_pipe[0]);
                    close(ctl_pipe[1]);
                    close(st_pipe[0]);
                    close(st_pipe[1]);
                    let argv = vec![CString::new("examples/main").unwrap().as_ptr()];
                    println!("Thread{}: calling execv", i);
                    execv(
                        CString::new("examples/main").unwrap().as_ptr(),
                        argv.as_ptr(),
                    );
                }

                close(ctl_pipe[0]);
                close(st_pipe[1]);

                let fsrv_st_fd = st_pipe[0];

                let rlen: ssize_t = read(fsrv_st_fd, status as *mut c_void, 4 as size_t);

                if rlen == 4 {
                    println!("Thread{}: forkserver set up correctly!", i);
                // File::create(format!("target/debug/t{}_s", i)).expect("cannot create the file");
                } else {
                    println!(
                        "Thread{}: forserver didn't setup correctly!
                        \nrlen is {}, status is {}, fsrv_st_fd is {}, writer end is {}",
                        i, rlen, *status, fsrv_st_fd, st_pipe[1]
                    );
                    // File::create(format!("target/debug/t{}_f", i)).expect("cannot create the file");
                    panic!("ALERT: errno={}", errno::errno());
                }
            }
        }));
    }
    for thread_ in threads {
        let _ = thread_.join();
    }
}
