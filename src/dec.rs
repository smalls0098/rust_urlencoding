use std::fmt::Display;
use std::fmt;
use std::error::Error;
use std::string::FromUtf8Error;

#[inline]
pub(crate) fn from_hex_digit(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        _ => None,
    }
}

/// Decode percent-encoded string assuming UTF-8 encoding.
///
/// Unencoded `+` is preserved literally, and _not_ changed to a space.
pub fn decode(string: &str) -> Result<String, FromUrlEncodingError> {
    let mut out: Vec<u8> = Vec::with_capacity(string.len());
    let mut bytes = string.as_bytes().iter().copied();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                match bytes.next() {
                    Some(first) => match from_hex_digit(first) {
                        Some(first_val) => match bytes.next() {
                            Some(second) => match from_hex_digit(second) {
                                Some(second_val) => {
                                    out.push((first_val << 4) | second_val);
                                },
                                None => {
                                    out.push(b'%');
                                    out.push(first);
                                    out.push(second);
                                },
                            },
                            None => {
                                out.push(b'%');
                                out.push(first);
                            },
                        },
                        None => {
                            out.push(b'%');
                            out.push(first);
                        },
                    },
                    None => out.push(b'%'),
                };
            },
            other => out.push(other),
        }
    }
    String::from_utf8(out).map_err(|error| FromUrlEncodingError::Utf8CharacterError {error})
}

/// Error when decoding invalid UTF-8
#[derive(Debug)]
pub enum FromUrlEncodingError {
    /// Not used. Exists for backwards-compatibility only
    UriCharacterError { character: char, index: usize },
    /// Percent-encoded string contained bytes that can't be expressed in UTF-8
    Utf8CharacterError { error: FromUtf8Error },
}

impl Error for FromUrlEncodingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FromUrlEncodingError::UriCharacterError {character: _, index: _} => None,
            FromUrlEncodingError::Utf8CharacterError {error} => Some(error)
        }
    }
}

impl Display for FromUrlEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            FromUrlEncodingError::UriCharacterError {character, index} =>
                write!(f, "invalid URI char [{}] at [{}]", character, index),
            FromUrlEncodingError::Utf8CharacterError {error} =>
                write!(f, "invalid utf8 char: {}", error)
        }
    }
}
