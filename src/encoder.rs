use crate::mode::Mode;
use crate::version::Version;
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use reed_solomon::Encoder;

pub struct EncodeError {}

pub fn encode(version: &Version, mode: &Mode, data: &str) -> Result<BitVec, EncodeError> {
    let mut d = match mode {
        Mode::NUMBER => match encode_number(version, data) {
            Ok(d) => d,
            Err(e) => return Err(e),
        },
    };
    pad(version, &mut d);
    let ecc = calc_ecc(version, &d);
    d.extend(ecc);
    Ok(d)
}

fn pad(version: &Version, data: &mut BitVec) {
    data.resize(data_size(version), false);
}

fn calc_ecc(version: &Version, data: &BitVec) -> BitVec<Msb0, u8> {
    let enc = Encoder::new(ecc_len(version));
    let v = data.iter().map(|x| *x as u8).collect::<Vec<_>>();
    return enc.encode(&v).ecc().bits::<Msb0>().to_vec();
}

fn encode_number(version: &Version, number: &str) -> Result<BitVec, EncodeError> {
    let mut data: BitVec<Msb0, u16> = BitVec::new();
    let mut i = 0;
    loop {
        if number.len() <= i {
            break;
        }
        let sn: &str = match number.get(i..i + 3) {
            Some(n) => n,
            None => number.get(i..).unwrap(),
        };
        match number_to_bits(sn) {
            Ok(b) => data.extend(b),
            Err(e) => return Err(e),
        }
        i += 3;
    }

    let len = number.len();
    let mut res = number_indicator(version);
    let mut ind = len.bits::<Lsb0>().to_vec();
    ind.truncate(number_of_char_indicators(version));
    ind.reverse();
    res.extend(ind);
    res.extend(data);
    return Ok(res);
}

fn number_to_bits(number: &str) -> Result<BitVec<Lsb0, u16>, EncodeError> {
    let len = match number.len() {
        1 => 4,
        2 => 7,
        3 => 10,
        _ => return Err(EncodeError {}),
    };
    let n: u16 = number.parse().unwrap();
    let mut bits = n.bits::<Lsb0>().to_vec();
    bits.truncate(len);
    bits.reverse();
    return Ok(bits);
}

fn number_indicator(version: &Version) -> BitVec {
    match version {
        Version::M1 => BitVec::new(),
    }
}

fn number_of_char_indicators(version: &Version) -> usize {
    match version {
        Version::M1 => 3,
    }
}

fn ecc_len(version: &Version) -> usize {
    match version {
        Version::M1 => 2,
    }
}

fn data_size(version: &Version) -> usize {
    match version {
        Version::M1 => 20,
    }
}
