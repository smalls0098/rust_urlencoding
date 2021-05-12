use std::io::Write;
use std::io;

/// Percent-encodes every byte except alphanumerics and `-`, `_`, `.`, `~`. Assumes UTF-8 encoding.
pub fn encode(data: &str) -> String {
    let mut escaped = Vec::with_capacity(data.len());
    encode_into(data, &mut escaped).unwrap();
    // Encoded string is guaranteed to be ASCII
    unsafe {
        String::from_utf8_unchecked(escaped)
    }
}

#[inline]
fn encode_into<W: Write>(data: &str, mut escaped: W) -> io::Result<()> {
    for byte in data.as_bytes().iter() {
        match *byte {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' |  b'-' | b'.' | b'_' | b'~' => {
                escaped.write_all(std::slice::from_ref(byte))?;
            },
            other => {
                escaped.write_all(&[b'%', to_hex_digit(other >> 4), to_hex_digit(other & 15)])?;
            },
        }
    }
    Ok(())
}

#[inline]
fn to_hex_digit(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        10..=255 => b'A' - 10 + digit,
    }
}
