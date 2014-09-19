/*! Does this work? */

/* Lints. These are temporary. */
#![allow(unused_imports)]
#![allow(unused_variable)]

/* Feature gates. Some of this might be unstable. */
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(default_type_params)]
#![feature(overloaded_calls)]
#![feature(advanced_slice_patterns)]
#![feature(tuple_indexing)]

/* Include any other required traits. */
extern crate openssl;
extern crate serialize;
extern crate ncurses;

/* Include all program submodules. */
pub mod store;
pub mod macro;
pub mod ui;



fn main()
{
    use std::io::stdin;
    use serialize::json;
    use store::database::Database;

    let database = Database::load("ndb");

    let ui_selection = ui::NCURSES;

    let database = Database::load("ndb");

    let mut interface = match ui_selection {
        ui::NCURSES => ui::ncurses::init(database),
        _           => ui::ncurses::init(database)
    };

    interface.run();
}
