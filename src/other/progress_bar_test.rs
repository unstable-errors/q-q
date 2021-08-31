use std::thread;
use std::time::Duration;

use indicatif::ProgressBar;

pub fn progress_bar_test() {
    let pb = ProgressBar::new(1024);
    for _ in 0..1024 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(1));
    }
    pb.finish_with_message("done");
}
