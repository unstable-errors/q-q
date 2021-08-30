use dialoguer::{theme::ColorfulTheme, Select};

pub mod menu_handler;
pub mod other;
pub mod games;

fn main() {
    let selections = &[
        "Apps",
        "Games",
        "Other",
        "Exit",
    ];
    let appslist = &[
        "Exit",
    ];
    let gameslist = &[
        "guessing_game",
        "Exit",
    ];
    let otherlist = &[
        "progress_bar_test",
        "python",
        "Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What category?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
        menu_handler::launch_app(selections[selection]);
    if selections[selection] == "Apps"
    {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What app?")
            .default(0)
            .items(&appslist[..])
            .interact()
            .unwrap();
        menu_handler::launch_app(appslist[selection]);
    }
    else if selections[selection] == "Games"
    {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What app?")
            .default(0)
            .items(&gameslist[..])
            .interact()
            .unwrap();
        menu_handler::launch_app(gameslist[selection]);
    }
    else if selections[selection] == "Other"
    {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What app?")
            .default(0)
            .items(&otherlist[..])
            .interact()
            .unwrap();
        menu_handler::launch_app(otherlist[selection]);
    }
}