mod modes;

use crate::modes::{complex_wires::complex_wires_mode, wires::wires_mode};
use anyhow::Context;
use bomb::bomb_info::BombInfo;

fn main() {
    clear();
    println!("Please input your serial number:");
    let serial = read_string().expect("Invalid input.");
    println!("Please input your batteries count:");
    let batteries_count = read_string().expect("Invalid input.");
    println!("Please input y/n has parallel port:");
    let has_parallel_port = read_bool().expect("Invalid input.");
    BombInfo::init(&serial, &batteries_count, has_parallel_port).unwrap();
    clear();
    loop {
        match one_game() {
            Ok(true) => break,
            Ok(false) => {
                clear();
                continue;
            }
            Err(e) => {
                clear();
                println!("Error: {e}");
                continue;
            }
        }
    }
    println!("Game ended.");
}

/// True for end game.
fn one_game() -> anyhow::Result<bool> {
    print!(
        r#"choose mode:
1. Wires
2. Complex Wires
0. Exit
"#,
    );
    let game_mode: u8 = read_u8()?;
    match game_mode {
        0 => return Ok(true),
        1 => {
            clear();
            println!("Play wires mode");
            loop {
                match wires_mode() {
                    Ok(_) => {
                        println!("type any for continue, 0 for menu.");
                        if read_string()?.eq("0") {
                            break;
                        }
                        clear();
                        continue;
                    }
                    Err(e) => {
                        println!("Error: {e}");
                        println!("type any for continue, 0 for menu.");
                        if read_string()?.eq("0") {
                            break;
                        }
                        clear();
                        continue;
                    }
                }
            }
        }
        2 => {
            clear();
            println!("Play complex wires mode");
            loop {
                match complex_wires_mode() {
                    Ok(_) => {
                        println!("type any for continue, 0 for menu.");
                        if read_string()?.eq("0") {
                            break;
                        }
                        clear();
                        continue;
                    }
                    Err(e) => {
                        println!("Error: {e}");
                        println!("type any for continue, 0 for menu.");
                        if read_string()?.eq("0") {
                            break;
                        }
                        clear();
                        continue;
                    }
                }
            }
        }
        _ => {
            println!("Invalid game mode");
        }
    };

    Ok(false)
}

fn read_u8() -> anyhow::Result<u8> {
    let s = read_string()?;
    let n: u8 = s
        .parse()
        .context(format!("Invalid input, must be a number.: {s}"))?;
    Ok(n)
}

fn read_string() -> anyhow::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn read_bool() -> anyhow::Result<bool> {
    let s = read_string()?;
    let b = match s.as_str() {
        "n" => false,
        "y" => true,
        _ => anyhow::bail!("Invalid input, must be y/n.: {s}"),
    };
    Ok(b)
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
