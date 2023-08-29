use anyhow::Context;
use bomb::{
    bomb_info::BombInfo,
    wires::{Color, Wires},
};

fn main() {
    clear();
    println!("Please input your serial number:");
    let serial = read_string().expect("Invalid input.");
    println!("Please input your batteries count:");
    let batteries_count = read_string().expect("Invalid input.");
    BombInfo::init(&serial, &batteries_count).unwrap();
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
0. Exit
"#,
    );
    let game_mode: u8 = read_u8()?;
    match game_mode {
        0 => return Ok(true),
        1 => {
            clear();
            println!("Play wires mode");
            println!("Enter wire colors from above to bottom using format like 'rwbdy' for red, white, blue, dark, yellow\n");
            loop {
                match wires_mode() {
                    Ok(_) => {
                        println!("type 0 for continue, other for menu.");
                        if !read_string()?.eq("0") {
                            break;
                        }
                        clear();
                        continue;
                    }
                    Err(e) => {
                        println!("Error: {e}");
                        println!("type 0 for continue, other for menu.");
                        if !read_string()?.eq("0") {
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

fn wires_mode() -> anyhow::Result<()> {
    println!("Wires mode, Enter your string:");
    let s = read_string()?;
    let colors = s
        .chars()
        .map(|c| match c {
            'r' => Ok(Color::Red),
            'w' => Ok(Color::White),
            'b' => Ok(Color::Blue),
            'd' => Ok(Color::Dark),
            'y' => Ok(Color::Yellow),
            _ => anyhow::bail!("{s} has invalid color:{c}"),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let wires = Wires::new(colors);
    let nth = wires.get_cut_num();
    println!("Cut-off order {}\n", nth + 1);
    Ok(())
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

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
