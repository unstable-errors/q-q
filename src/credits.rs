// Copyright (c) 2021 unstable-errors
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crossterm::{
    cursor::{ Hide, Show},
    event::{
        self,
        Event,
        KeyEvent,
        KeyCode::*,
    },
    queue,
    terminal::{
        self,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    style::Color::*,
};
use std::io::{stdout, Write};
use termimad::*;

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column
    area
}

fn run_app(skin: MadSkin) -> Result<(), Error> {
    let mut w = stdout(); // we could also have used stderr
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
    let mut view = MadView::from(MD.to_owned(), view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent{code, ..})) => {
                match code {
                    Up => view.try_scroll_lines(-1),
                    Down => view.try_scroll_lines(1),
                    PageUp => view.try_scroll_pages(-1),
                    PageDown => view.try_scroll_pages(1),
                    _ => break,
                }
            }
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = Alignment::Center;
    skin
}

pub fn credits() -> Result<(), Error> {
    let skin = make_skin();
    run_app(skin)
}

static MD: &str = r#"# Credits

## Documentation
SoupDevHub - README.md (most of it)
electron271 - README.md (minor changes), CONTRIBUTING.md INSTALL.md

## Code
electron271 - all of the code
dependabot - will hopefully help us

## Testing
SoupDevHub - Windows
electron271 - Linux
## Special thanks
My Cat - motivation
You - ..if you contributed (Actually, we really appreciate you for even launching this application)

## DISCLAMER (yes really)
q>q is not designed to be useful. It is a proof of concept application and also just a fun application to use when you're bored!

**Press [Q] or [ESC] to exit.**
"#;
