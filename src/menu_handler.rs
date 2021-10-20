// Copyright (c) 2021 unstable-errors
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::apps;
use crate::credits; // credits
use crate::games; // games
use crate::other; // other // apps

pub fn launch_app(app: &str) {
    if app == "Exit" {
        println!("Bye!");
    } else if app == "guessing_game" {
        games::guessing_game::guessing_game();
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
