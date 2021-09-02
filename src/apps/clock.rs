// labels: APPS, TIME
use chrono::{DateTime, Local};
use colored::*;
use std::{thread, time};

pub fn clock() {
    let ten_millis = time::Duration::from_millis(10);
    loop {
        let now: DateTime<Local> = Local::now();
        println!("{}", format!("{}", now.format("%b %d %I:%M:%S %p")).blue());
        thread::sleep(ten_millis);
        println!("\x1B[2J\x1B[1;1H"); // needs windows testing
    }
}
