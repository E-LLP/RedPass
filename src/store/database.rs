/*!
 * This module deals with storing and loading password database.
 */

use store::password::Password;
use std::collections::TreeMap;

pub struct DatabaseView<'r> {
    pub open: bool,
    pub passwords: Vec<&'r Password>
}


pub struct Database {
    pub passwords: Vec<Password>
}


impl Database {
    /// Initializes a new database.
    pub fn new() -> Database {
        Database {
            passwords: Vec::new()
        }
    }

    /// Load the database from a file.
    pub fn load(filename: &str) -> Database {
        use serialize::json;
        use std::io::{
            BufferedReader,
            File
        };

        let mut file = Path::new(filename);
        let mut file = BufferedReader::new(File::open(&file));

        let mut lines: Vec<String> = file
            .lines()
            .filter_map(|x| x.ok())
            .collect();

        Database {
            passwords: lines
                .iter()
                .map(|x| json::decode::<Password>(x.as_slice()))
                .filter_map(|x| x.ok())
                .collect()
        }
    }

    /// Insert a new `Password` into the database.
    pub fn insert(&mut self, data: Password)
    {
        self.passwords.push(data);
    }

    /// Write the database to a file.
    pub fn write(&self, filename: &str)
    {
        use serialize::json;
        use std::io::{
            File
        };

        let mut output = File::create(&Path::new(filename));
        for entry in self.passwords.iter() {
            output.write_line(json::encode(&entry).as_slice());
        }
    }

    /// Create a new view into the database, this is a collection of passwords
    /// who's service matches the passed subsequence. It's used for filtering
    /// the database with a fuzzy search.
    pub fn view(&self, service: &str) -> TreeMap<&str, DatabaseView>
    {
        /* Simple function that checks for subsequences. */
        fn subsequence(needle: &str, haystack: &str) -> bool
        {
            use std::char::to_lowercase;

            if needle == "" {
                return true;
            }

            /* For each character in the needle, see if it appears in the
             * haystack in a case insensitive manner. */
            match haystack.find(|x| to_lowercase(x) == to_lowercase(needle.char_at(0))) {
                Some(x) => subsequence(needle.slice_from(1), haystack.slice_from(x)),
                None    => false
            }
        }

        /* Create a mapping, Service -> Entries */
        let mut mapping: TreeMap<&str, DatabaseView> = TreeMap::new();

        /* Scan the passwords searching for the ones that contain the service
         * subsequence. */
        for password in self.passwords.iter()
        {
            /* If this service matches, insert into the mapping. */
            if subsequence(service, password.service.as_slice())
            {
                if mapping.contains_key(&password.service.as_slice())
                {
                    mapping
                        .find_mut(&password.service.as_slice())
                        .unwrap()
                        .passwords
                        .push(password);
                }
                else
                {
                    mapping.insert(
                        password.service.as_slice(),
                        DatabaseView {
                            open:      false,
                            passwords: vec![password]
                        }
                    );
                }

                //mapping.find_with_or_insert_with(
                //    password.service.as_slice(),
                //    password,
                //    |key, current, new| {
                //        current.passwords.push(new);
                //    },
                //    |_, new| {
                //        DatabaseView {
                //            open: false,
                //            passwords: vec![new]
                //        }
                //    }
                //);
            }
        }

        mapping

        // self.passwords
        //     .iter()
        //     .filter_map(|x| {
        //         let sub = subsequence(service, x.service.as_slice());
        //         if sub { Some(x) }
        //         else   { None }
        //     })
        //     .collect()
    }
}
