pub fn code_to_index(code: &String) -> Result<u64, char> {
    let mut index: u64 = 0;
    for chr in code.chars() {
        index <<= 6;
        index += _chr_to_index(chr)? as u64;
    }
    Ok(index)
}


pub fn index_to_code(index: u64, order: usize) -> String {
    const MASK: u64 = 63;
    let mut index_mut = index;
    let mut chars = Vec::<char>::new();
    for _ in 0..order {
        let value = (index_mut & MASK) as u8;
        let chr = _index_to_chr(value).unwrap();
        chars.push(chr);
        index_mut >>= 6;
    }
    chars.into_iter().rev().collect()
}


fn _chr_to_index(chr: char) -> Result<u8, char> {
    match chr {
        '0'..='9' => Ok(chr as u8 - 48 + 52),
        'A'..='Z' => Ok(chr as u8 - 65 + 26),
        'a'..='z' => Ok(chr as u8 - 97),
        '+' => Ok(62),
        '-' => Ok(63),
        _ => Err(chr)
    }
}


fn _index_to_chr(index: u8) -> Result<char, u8> {
    match index {
        0..=25 => Ok((index + 97) as char),
        26..=51 => Ok((index + 65 - 26) as char),
        52..=61 => Ok((index + 48 - 52) as char),
        62 => Ok('+'),
        63 => Ok('-'),
        _ => Err(index)
    }
}
