use anyhow::Context;

pub fn read_u8() -> anyhow::Result<u8> {
    let s = read_string()?;
    let n: u8 = s
        .parse()
        .context(format!("Invalid input, must be a number.: {s}"))?;
    Ok(n)
}

pub fn read_string() -> anyhow::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn read_char() -> anyhow::Result<char> {
    let s = read_string()?;
    let c: char = s
        .parse()
        .context(format!("Invalid input, must be a char.: {s}"))?;
    Ok(c)
}
