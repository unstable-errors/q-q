// Copyright (C) 2021 electron271
// 
// This file is part of q>q.
// 
// q>q is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// q>q is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with q>q.  If not, see <http://www.gnu.org/licenses/>.

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
