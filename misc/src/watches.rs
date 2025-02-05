#![allow(unused_imports, unused_variables, dead_code)]

use std::path::{Path, PathBuf};

use notify::Config;
extern crate notify;

fn t_watch() {
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    fn watch() -> notify::Result<()> {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let config = Config::default().with_poll_interval(Duration::from_secs(2));
        let mut watcher: RecommendedWatcher = Watcher::new(tx, config)?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        use std::env;
        let home_dir: PathBuf = env::var_os("HOME").unwrap().into();
        watcher.watch(home_dir.as_path(), RecursiveMode::Recursive)?;

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

fn main() {
    t_watch();
}
