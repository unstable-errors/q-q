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

use dialoguer::{theme::ColorfulTheme, Select}; // menu

pub mod apps; // get the apps
pub mod credits; // get the credits
pub mod games; // get the games
pub mod menu_handler; // get the menu handler
pub mod other; // get the other apps // credits

fn main() {
    let selections = &["Apps", "Games", "Other", "About", "Exit"]; // types of apps
    let appslist = &["clock", "Exit"]; // apps
    let gameslist = &["guessing_game", "Exit"]; // games
    let otherlist = &["progress_bar_test", "Exit"]; // other
    let aboutlist = &["credits", "Exit"]; // about

    let selection = Select::with_theme(&ColorfulTheme::default()) // get category
        .with_prompt("What category?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    menu_handler::just_exit(selections[selection]);
    if selections[selection] == "Apps" {
        let selection = Select::with_theme(&ColorfulTheme::default()) // choose app
            .with_prompt("What app?")
            .default(0)
            .items(&appslist[..])
            .interact()
            .unwrap();
        menu_handler::launch_app(appslist[selection]); // launch app
    } else if selections[selection] == "Games" {
        let selection = Select::with_theme(&ColorfulTheme::default()) // choose game
            .with_prompt("What app?")
            .default(0)
            .items(&gameslist[..])
            .interact()
            .unwrap();
        menu_handler::launch_app(gameslist[selection]); // launch game
    } else if selections[selection] == "Other" {
        let selection = Select::with_theme(&ColorfulTheme::default()) // choose other
            .with_prompt("What app?")
            .default(0)
            .items(&otherlist[..])
            .interact()
            .unwrap();
        menu_handler::launch_app(otherlist[selection]); // launch other
    } else if selections[selection] == "About" {
        let selection = Select::with_theme(&ColorfulTheme::default()) // choose about
            .with_prompt("Choose an option")
            .default(0)
            .items(&aboutlist[..])
            .interact()
            .unwrap();
        menu_handler::about(aboutlist[selection]); // launch about
    }
}
