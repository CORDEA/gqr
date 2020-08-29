use crate::mode::Mode;
use crate::version::Version;

pub struct EncodeError {}

pub fn encode(version: &Version, mode: &Mode, data: &str) -> Result<String, EncodeError> {
    return match mode {
        Mode::NUMBER => encode_number(version, data),
    };
}

fn encode_number(version: &Version, number: &str) -> Result<String, EncodeError> {
    let mut i = 0;
    let mut data = Vec::new();
    loop {
        if number.len() <= i {
            break;
        }
        let n: &str = match number.get(i..i + 3) {
            Some(n) => n,
            None => number.get(i..).unwrap(),
        };
        match number_to_bits(n) {
            Ok(bits) => data.push(bits),
            Err(e) => return Err(e),
        };
        i += 3;
    }

    let len = format!("{:b}", number.len());
    let pad = (0..(number_of_char_indicators(version) - len.len() as u16))
        .map(|_| "0")
        .collect::<Vec<_>>()
        .concat();
    return Ok(format!(
        "{}{}{}{}",
        number_indicator(version),
        pad,
        len,
        data.concat()
    ));
}

fn number_to_bits(number: &str) -> Result<String, EncodeError> {
    let n: u16 = number.parse().unwrap();
    let result = match number.len() {
        1 => Ok(format!("{:04b}", n)),
        2 => Ok(format!("{:07b}", n)),
        3 => Ok(format!("{:010b}", n)),
        _ => Err(EncodeError {}),
    };
    return result;
}

fn number_indicator(version: &Version) -> String {
    return match version {
        Version::M1 => String::new(),
    };
}

fn number_of_char_indicators(version: &Version) -> u16 {
    return match version {
        Version::M1 => 3,
    };
}
