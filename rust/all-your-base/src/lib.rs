#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn require(predicate: bool, error: Error) -> Result<(), Error> {
    if predicate { Ok(()) } else { Err(error) }
}

fn convert_from_base(number: &[u32], base: u32) -> Result<u32, Error> {
    require(base >= 2, Error::InvalidInputBase)?;
    let mut val = 0;
    for &digit in number {
        require(digit < base, Error::InvalidDigit(digit))?;
        val = val * base + digit;
    }
    Ok(val)
}

fn convert_to_base(mut number: u32, base: u32) -> Result<Vec<u32>, Error> {
    require(base >= 2, Error::InvalidOutputBase)?;
    if number == 0 {
        return Ok(vec![0]);
    }
    let mut res = vec![];
    while number > 0 {
        res.push(number % base);
        number /= base;
    }
    res.reverse();
    Ok(res)
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    convert_from_base(number, from_base).and_then(|n| convert_to_base(n, to_base))
}
