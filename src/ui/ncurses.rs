/*!
 * An ncurses based UI.
 */

use ncurses;
use store::database::Database;


pub struct UIState {
    search:      String,
    keycode:     i32,
    database:    Database,
    current_row: i32
}


pub fn init(db: Database) -> UIState
{
    ncurses::initscr();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_INVISIBLE);
    ncurses::start_color();
    ncurses::use_default_colors();
    ncurses::init_pair(1, ncurses::COLOR_WHITE, -1);
    ncurses::init_pair(2, ncurses::COLOR_BLUE, -1);
    ncurses::init_pair(3, ncurses::COLOR_GREEN, -1);
    ncurses::init_pair(4, ncurses::COLOR_WHITE, ncurses::COLOR_BLUE);
    ncurses::keypad(ncurses::stdscr, true);

    UIState {
        search:      String::with_capacity(32),
        keycode:     0,
        database:    db,
        current_row: 0
    }
}


impl UIState
{
    pub fn draw(&self, width: i32, height: i32)
    {
        ncurses::clear();

        /* Draw the Title. */
        ncurses::attron(ncurses::COLOR_PAIR(2));
        ncurses::mvprintw(0, 0, "Password Dictionary");
        ncurses::attroff(ncurses::COLOR_PAIR(2));

        /* Draw Seperators. */
        ncurses::attron(ncurses::COLOR_PAIR(2));
        ncurses::move(2, 0);
        ncurses::hline(b'-' as u32, width);
        ncurses::attroff(ncurses::COLOR_PAIR(2));

        /* Render Search Box. */
        let message = "| Start typing to Filter: ";
        ncurses::mvprintw(2, 2, message);
        ncurses::mvprintw(2, 2 + message.len() as i32, self.search.as_slice());
        ncurses::mvprintw(2, 2 + (message.len() + self.search.len()) as i32, " |");

        /* Render Password List. */
        let mut row = 0;

        for (service, view) in self.database.view(self.search.as_slice().trim_right()).iter()
        {
            if row == self.current_row
            {
                ncurses::attron(ncurses::COLOR_PAIR(4));
            }

            ncurses::mvprintw(3 + row, 0, if view.open { "  + " } else { "  | " });
            ncurses::mvprintw(3 + row, 4, service.as_slice());
            ncurses::move(3 + row, 4 + service.len() as i32);
            ncurses::hline(b' ' as u32, width - (4 + service.len()) as i32);

            if row == self.current_row
            {
                ncurses::attroff(ncurses::COLOR_PAIR(4));
            }

            if row == self.current_row {
                for entry in view.passwords.iter() {
                    row += 1;

                    if row >= height {
                        break;
                    }

                    ncurses::mvprintw(3 + row, 0, "  |    ");
                    ncurses::attron(ncurses::COLOR_PAIR(3));
                    ncurses::mvprintw(3 + row, 7, entry.username.as_slice());
                    let info = format!(
                        "Len: {}, Type: {}",
                        entry.length,
                        entry.encoding
                    );
                    ncurses::mvprintw(3 + row, width - info.len() as i32, info.as_slice());
                    ncurses::attroff(ncurses::COLOR_PAIR(3));
                }
            }

            if row >= height {
                break;
            }

            row += 1;
        }

        /* Flush Screen. */
        ncurses::refresh();
    }

    pub fn run(&mut self)
    {
        loop
        {
            /* Get Window size for UI Drawing. */
            let mut cx = 0i32;
            let mut cy = 0i32;
            ncurses::getmaxyx(ncurses::stdscr, &mut cy, &mut cx);

            /* Clear and draw the interface. */
            self.draw(cx, cy);

            /* Handle search input. */
            let character = ncurses::getch();

            match character
            {
                /// Handle character inputs.
                65 .. 91 | 97 .. 123 | 32 => {
                    let character = ::std::char::from_u32(character as u32);

                    if character.is_some() {
                        let character = character.unwrap();
                        self.search.push_char(character);
                    }

                    self.current_row = 0;
                }

                /// Handle backspace key.
                ncurses::KEY_BACKSPACE | 127 => {
                    self.search.pop_char();
                    self.current_row = 0;
                }

                /// Handle pressing the down key.
                258 => {
                    if self.current_row < cy {
                        self.current_row += 1;
                    }
                }

                /// Handle pressing the up key.
                259 => {
                    if self.current_row > 0
                    {
                        self.current_row -= 1;
                    }
                }

                /// For all other keypresses, keep the keycode.
                _ => {
                    self.keycode = character;
                }
            }
        }

        ncurses::endwin();
    }
}
