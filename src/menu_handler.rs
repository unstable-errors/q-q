// labels: CORE, MENU
use crate::games; // games
use crate::other; // other

pub fn launch_app(app: &str) {
    if app == "Exit" {
        println!("Bye!")
    } else if app == "guessing_game" {
        games::guessing_game::guessing_game();
    } else if app == "python" {
        other::python::python(); // so far not being fixed, if anyone wants disable this warning WONTFIX
    } else if app == "progress_bar_test" {
        other::progress_bar_test::progress_bar_test();
    } else {
        // debug code, uncomment if something doesnt seem to work just right DEBUG
        // println!("error: {} not found", app);
    }
}
pub fn just_exit(app: &str) {
    if app == "Exit" {
        println!("Bye!")
    }
}
