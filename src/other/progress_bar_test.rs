// Copyright (c) 2021 unstable-errors
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// import libs:
// std libs
use std::cmp::min;
use std::thread;
use std::time::Duration;

// random libs
use rand::Rng;

// progress bar libs
use indicatif::{ProgressBar, ProgressStyle};

// main function [progress_bar_test]
pub fn progress_bar_test() {
    let mut downloaded = 0;
    let total_size = 231231231;
    let mut rng = rand::thread_rng();

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(rng.gen_range(2..50)));
    }

    pb.finish_with_message("downloaded");
}
