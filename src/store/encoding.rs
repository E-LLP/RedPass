/*!
 * This module encodes randomly generated bytes into human readable passwords.
 *
 * Password generation produces a set of random bytes. These bytes are useless
 * alone, and need to be encoded in a format that can be used as a password.
 * Encodings exist for passwords in alphanumeric format, passwords with special
 * characters, hexadecimal, so forth and so forth.
 */

use serialize::{Encodable, Decodable};


/// Type of an encoding function.
pub type Encoder = fn(&[u8]) -> String;


/// Used to select the encoding function for a password.
#[deriving(Show, Encodable, Decodable)]
pub enum Encoding {
    Ascii85,
    Hexadecimal,
    Base64,
    Unknown(String)
}


impl Encoding {
    /// Using the current Encoding, encode the data and produce a vector with
    /// the resulting bytes stored. This vector can be treated as a slice to get
    /// at the resulting encoding.
    pub fn encode_bytes(&self, data: &[u8]) -> String {
        use self::encoders::*;

        match *self {
            Ascii85     => ascii85(data),
            Hexadecimal => hexadecimal(data),
            Base64      => base64(data),
            Unknown(_)  => {
                /* TODO: Lookup custom encoding functions somehow. Lua scripts
                 * would be cool. */
                String::from_str("")
            }
        }
    }
}


/// Collection of pre-defined encoder functions.
pub mod encoders {
    pub fn ascii85(data: &[u8]) -> String {
        let mut result = Vec::from_elem((data.len() / 4) * 5, 0u8);

        /* Iterate over groups of 4 characters at a time in the data. */
        for i in range(0, data.len() / 4) {
            /* Calculate the binary value of the 4 characters treated as a
             * single 32 bit integer. */
            let mut n: u32 = (0 | data[i * 4 + 0] as u32) << 8;
            n = (n | data[i * 4 + 1] as u32) << 8;
            n = (n | data[i * 4 + 2] as u32) << 8;
            n =  n | data[i * 4 + 3] as u32;

            /* Encode the integer as an Ascii85 character. */
            for c in range(0u, 5u) {
                *result.get_mut(i*5 + 4-c) = (n % 85) as u8 + 33;
                n /= 85;
            }
        }

        /* We have the result in bytes now. We have to convert this into an
         * ascii string. */
        result.into_ascii().into_string()
    }


    pub fn hexadecimal(data: &[u8]) -> String {
        use serialize::hex::ToHex;

        data.to_hex()
    }


    pub fn base64(data: &[u8]) -> String {
        String::from_str("")
    }
}
