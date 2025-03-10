const LUT: [&str; 16] = [
    "Nak",
    "Nanak",
    "Nananak",
    "Nanananak",
    "Nak?",
    "nak?",
    "Naknak",
    "Naknaknak",
    "Nak.",
    "Naknak.",
    "Naknaknaknak",
    "nanak",
    "naknak",
    "nak!",
    "nak.",
    "naknaknak",
];

/// Encodes a string to naknak cipher.
pub fn encode(input: &str) -> String {
    let mut ret = String::new();
    for b in input.as_bytes() {
        let pos = (b >> 4) as usize;
        ret.push_str(LUT[pos]);
        ret.push(' ');
        let pos = (b & 0b00001111) as usize;
        ret.push_str(LUT[pos]);
        ret.push(' ');
    }
    // Remove the last whitespace
    ret.pop();
    ret
}

/// Decodes a naknak string.
pub fn decode(input: &str) -> Result<String, String> {
    let mut ret = String::new();
    let mut byte = 0u8;
    for (ind, s) in input.split_whitespace().enumerate() {
        let b = {
            let mut ret = None;
            for (i, &nak) in LUT.iter().enumerate() {
                if s == nak {
                    ret = Some(i as u8);
                    break;
                }
            }
            match ret {
                Some(b) => b,
                None => {
                    let msg = format!("Invalid code {}", s);
                    return Err(msg);
                }
            }
        };
        if ind % 2 == 0 {
            byte = b << 4;
        } else {
            byte |= b;
            ret.push(byte as char);
        }
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_str = "Hello, world!";
        let test_naknak = "Nak? Nak. Naknak nak? Naknak naknak Naknak naknak Naknak naknaknak Nananak naknak Nananak Nak Naknaknak Naknaknak Naknak naknaknak Naknaknak Nananak Naknak naknak Naknak Nak? Nananak Nanak";
        assert_eq!(encode(test_str), String::from(test_naknak));
        assert_eq!(decode(test_naknak).unwrap(), String::from(test_str));
        let test_str = "test";
        let test_naknak = "Naknaknak Nak? Naknak nak? Naknaknak Nanananak Naknaknak Nak?";
        assert_eq!(encode(test_str), String::from(test_naknak));
        assert_eq!(decode(test_naknak).unwrap(), String::from(test_str));
    }
}
