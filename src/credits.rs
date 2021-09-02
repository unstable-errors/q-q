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

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stderr, Write};
use termimad::*;

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // we don't want a too wide text column
    area
}

fn run_app(skin: MadSkin) -> Result<(), Error> {
    let mut w = stderr(); // we could also have used stdout
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
    let mut view = MadView::from(MD.to_owned(), view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            },
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
Well, this will be a long trip.. use the up and down arrows to scroll (⇅)

## Documentation
SoupDevHub - README.md (most of it)
electron271 - README.md (minor changes), CONTRIBUTING.MD

## Code
electron271 - all of the code
dependabot - will hopefully help us

## Testing
SoupDevHub - Windows
electron271 - Debian

## Special thanks
My Cat - motivation
You - ..if you contributed (Actually, we really appreciate you for even launching this application)

## DISCLAMER (yes really)
q>q is not designed to be useful. It is a proof of concept application and also just a fun application to use when you're bored!

Check out everyone mentioned (yes including yourself)


..now view some real application not this junk

"#;
