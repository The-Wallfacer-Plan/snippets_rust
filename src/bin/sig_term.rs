#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate nix;
extern crate libc;

use chan_signal::Signal;

fn chan() {
    // Signal gets a value when the OS sent a INT or TERM signal.
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    // When our work is complete, send a sentinel value on `sdone`.
    let (sdone, rdone) = chan::sync(0);
    // Run work.
    thread::spawn(move || run(sdone));

    // Wait for a signal or for work to be done.
    chan_select! {
        signal.recv() -> signal => {
            println!("received signal: {:?}", signal)
        },
        rdone.recv() => {
            println!("Program completed normally.");
        }
    }
}

fn run(_sdone: chan::Sender<()>) {
    println!("Running work for 5 seconds.");
    println!("Can you send a signal quickly enough?");
    // Do some work.
    thread::sleep_ms(5000);

    // _sdone gets dropped which closes the channel and causes `rdone`
    // to unblock.
}

use nix::sys::signal;
use nix::sys::signal::*;
use std::{thread, time};

static mut _GUARD: bool = false;

fn setup_handlers() {
    extern "C" fn test_sigaction_handler(_: libc::c_int) {
        unsafe { _GUARD = true };
    };

    let handler_sig = SigHandler::Handler(test_sigaction_handler);
    let flags = SA_RESTART;
    let mask = SigSet::empty();
    let sa = SigAction::new(handler_sig, flags, mask);
    unsafe { sigaction(SIGINT, &sa) }.unwrap();
}

fn main() {
    setup_handlers();
    loop {
        if unsafe { _GUARD } {
            println!("stopping...");
            break;
        }
        println!("sleepy...");
        let duration = time::Duration::from_millis(1000);
        thread::sleep(duration);
    }
    println!("exit...");
}
