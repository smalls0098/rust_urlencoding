use std::str;

/// Percent-encodes every byte except alphanumerics and `-`, `_`, `.`, `~`. Assumes UTF-8 encoding.
pub fn encode(data: &str) -> String {
    let data = data.as_bytes();
    // add maybe extra capacity, but try not to exceed allocator's bucket size
    let mut escaped = String::with_capacity(data.len() | 15);
    encode_into(data, |s| Ok::<_, std::convert::Infallible>(escaped.push_str(s))).unwrap();
    escaped
}

fn encode_into<E>(mut data: &[u8], mut push_str: impl FnMut(&str) -> Result<(), E>) -> Result<(), E> {
    loop {
        // Fast path to skip over safe chars at the beginning of the remaining string
        let ascii_len = data.iter()
            .take_while(|&&c| matches!(c, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' |  b'-' | b'.' | b'_' | b'~')).count();

        let (safe, rest) = if ascii_len >= data.len() {
            (data, &[][..]) // redundatnt to optimize out a panic in split_at
        } else {
            data.split_at(ascii_len)
        };
        if !safe.is_empty() {
            push_str(unsafe { str::from_utf8_unchecked(safe) })?;
        }
        if rest.is_empty() {
            return Ok(());
        }

        match rest.split_first() {
            Some((byte, rest)) => {
                let enc = &[b'%', to_hex_digit(byte >> 4), to_hex_digit(byte & 15)];
                push_str(unsafe { str::from_utf8_unchecked(enc) })?;
                data = rest;
            }
            None => return Ok(()),
        };
    }
}

#[inline]
fn to_hex_digit(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        10..=255 => b'A' - 10 + digit,
    }
}
