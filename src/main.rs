// labels: CORE, MENU
use dialoguer::{theme::ColorfulTheme, Select}; // menu

pub mod games; // get the games
pub mod menu_handler; // get the menu handler
pub mod other; // get the other apps

fn main() {
    let selections = &["Apps", "Games", "Other", "Exit"]; // types of apps
    let appslist = &["Exit"]; // apps
    let gameslist = &["guessing_game", "Exit"]; // games
    let otherlist = &["progress_bar_test", "python", "Exit"]; // other

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
    }
}
