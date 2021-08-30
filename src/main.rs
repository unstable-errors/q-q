use terminal_menu::{submenu, back_button, menu, label, button, run, mut_menu};

mod games;

fn main()
{

    let menu = menu(vec![

        // label:
        //  not selectable, usefule as a title, separator, etc...
        label("----------------------"),
        label("quantity>quality"),
        label("use wasd or arrow keys"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------------"),

        submenu("games", vec![

            // button:
            //  buttons exit all the menus
            button("guessing game"),
            back_button("back")

        ]),
        submenu("applications", vec![

            // button:
            //  buttons exit all the menus
            label("Oops! Nothing here!"),
            back_button("back"),

        ]),
        submenu("other", vec![

            // button:
            //  buttons exit all the menus
            label("Nothing to see here.."),
            back_button("back"),

        ]),

        button("exit")

    ]);
    run(&menu);

    // name of the menu active before exiting [DEBUG]
    // println!("{:?}", mut_menu(&menu).get_latest_menu_name());

    // pull values [DEBUG]
    // println!("{}", mut_menu(&menu).get_submenu("sub").selection_value("scr"));
    
    if mut_menu(&menu).selected_item_name() == "exit" {
        println!("Goodbye!");
    }
    if mut_menu(&menu).get_submenu("games").selected_item_name() == "guessing game" {
        games::guessing_game::guessing_game();
    }
}