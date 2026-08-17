use std::error::Error;

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if !hex.len().is_multiple_of(2) {
        return Err("Hex string must have even length".into());
    }

    // from_str_radix would happily accept a leading sign, so the characters are
    // checked up front rather than letting "+f" through as a byte.
    if let Some(bad) = hex.chars().find(|c| !c.is_ascii_hexdigit()) {
        return Err(format!("'{bad}' is not a hexadecimal digit").into());
    }

    // create vector with enough bytes capacity
    let mut bytes = Vec::with_capacity(hex.len() / 2);

    for i in (0..hex.len()).step_by(2) {
        /*
           Give me the next two hexadecimal characters.
           Convert the two hex characters into a byte.
        */
        let byte = u8::from_str_radix(&hex[i..i + 2], 16)?;
        // from_str_radix - Parse a string as a number using a particular base i.e 16
        bytes.push(byte);
    }

    Ok(bytes)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// === Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        assert_eq!(hex_to_bytes("00ff10").unwrap(), vec![0x00, 0xff, 0x10]);
        assert_eq!(bytes_to_hex(&[0x00, 0xff, 0x10]), "00ff10");
        assert_eq!(hex_to_bytes("").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn rejects_odd_length() {
        assert!(hex_to_bytes("abc").is_err());
    }

    #[test]
    fn rejects_non_hex_characters() {
        assert!(hex_to_bytes("zz").is_err());
        assert!(hex_to_bytes("+f").is_err());
        assert!(hex_to_bytes("ab cd").is_err());
    }
}
