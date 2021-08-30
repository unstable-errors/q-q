use crate::games;
use crate::other;

pub fn launch_app(app: &str)
{
    if app == "Exit"
    {
        println!("Bye!")
    }
    else if app == "guessing_game"
    {
        games::guessing_game::guessing_game();
    }
    else if app == "python"
    {
        other::python::python();
    }
    else if app == "progress_bar_test"
    {
        other::progress_bar_test::progress_bar_test();
    }
    else
    {
        // println!("error: {} not found", app);
    }
}