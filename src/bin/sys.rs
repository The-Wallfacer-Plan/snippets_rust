#![allow(unused_imports, unused_variables, dead_code)]
extern crate sysinfo;
extern crate notify;
#[macro_use]
extern crate log;

use sysinfo::SystemExt;

fn notify() {
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    fn watch() -> notify::Result<()> {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        use std::env;
        let home_dir: String = env::var("HOME").unwrap();
        try!(watcher.watch(home_dir, RecursiveMode::Recursive));

        // This is a simple loop, but you may want to use more complex logic here,
        // for example to handle I/O.
        loop {
            match rx.recv() {
                Ok(event) => println!("{:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }

    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}


fn sysinfo() {
    let mut system = sysinfo::System::new();

    // First we update all information of our system struct.
    system.refresh_all();

    // Now let's print every process' id and name:
    //    for (pid, proc_) in system.get_process_list() {
    //        println!("{}:{} => status: {:?}", pid, proc_.name, proc_.status);
    //    }

    for p in system.get_processor_list() {
        println!("{:?}", p);
    }

    // Then let's print the temperature of the different components:
    //    for component in system.get_components_list() {
    //        println!("{:?}", component);
    //    }

    // And then all disks' information:
    for disk in system.get_disks() {
        println!("{:?}", disk);
    }

    // And finally the RAM and SWAP information:
    println!("total memory: {} kB", system.get_total_memory());
    println!("used memory : {} kB", system.get_used_memory());
    println!("total swap  : {} kB", system.get_total_swap());
    println!("used swap   : {} kB", system.get_used_swap());
}

fn main() {
    sysinfo();
}
