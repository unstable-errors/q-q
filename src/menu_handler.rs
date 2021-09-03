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

use crate::apps;
use crate::credits; // credits
use crate::games; // games
use crate::other; // other // apps

pub fn launch_app(app: &str) {
    if app == "Exit" {
        println!("Bye!");
    } else if app == "guessing_game" {
        games::guessing_game::guessing_game();
    } else if app == "python" {
        other::python::python()
            .map_err(|err| println!("{:?}", err))
            .ok();
    } else if app == "progress_bar_test" {
        other::progress_bar_test::progress_bar_test();
    } else if app == "clock" {
        apps::clock::clock();
    } else {
        // debug code, uncomment if something doesnt seem to work just right DEBUG
        // println!("error: {} not found", app);
    }
}
pub fn just_exit(app: &str) {
    if app == "Exit" {
        println!("Bye!");
    } else {
        // debug code, uncomment if something doesnt seem to work just right DEBUG
        // println!("error: {} not found", app);
    }
}
pub fn about(app: &str) {
    if app == "Exit" {
        println!("Bye!");
    } else if app == "credits" {
        credits::credits().map_err(|err| println!("{:?}", err)).ok();
    } else {
        // debug code, uncomment if something doesnt seem to work just right DEBUG
        // println!("error: {} not found", app);
    }
}
