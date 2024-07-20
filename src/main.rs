mod modes;

use crate::modes::*;
use bomb::utils::*;

fn main() {
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
3. Button
4. Password
5. Morse
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
        3 => {
            clear();
            println!("Play button mode");
            loop {
                match button_mode() {
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
        4 => {
            clear();
            println!("Play password mode");
            loop {
                match password_mode() {
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
        5 => {
            clear();
            println!("Play morse mode");
            loop {
                match morse_mode() {
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

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
