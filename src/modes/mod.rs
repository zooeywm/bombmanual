pub(super) mod wires {
    use bomb::wires::{Color::*, Wires};

    use crate::read_string;

    pub fn wires_mode() -> anyhow::Result<()> {
        println!("Enter wire colors from above to bottom using format like 'rwbdy' for red, white, blue, dark, yellow\n");
        let s = read_string()?;
        let colors = s
            .chars()
            .map(|c| match c {
                'r' => Ok(Red),
                'w' => Ok(White),
                'b' => Ok(Blue),
                'd' => Ok(Dark),
                'y' => Ok(Yellow),
                _ => anyhow::bail!("{s} has invalid color:{c}"),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let wires = Wires::new(colors);
        let nth = wires.get_cut_num();
        println!("Cut-off order {}\n", nth + 1);
        Ok(())
    }
}

pub(super) mod complex_wires {
    use bomb::complex_wires::ComplexWires;

    use crate::read_string;

    pub fn complex_wires_mode() -> anyhow::Result<()> {
        println!(
            "Enter wire has_red, has_blue, has_star, has_led using y/n in format like 'ynyn'\n"
        );
        let s = read_string()?;
        if s.chars().count() != 4 {
            anyhow::bail!("Invalid input, must be 4 characters of y/n.")
        };
        let bools = s
            .chars()
            .map(|el| match el {
                'y' => Ok(true),
                'n' => Ok(false),
                _ => anyhow::bail!("Invalid input, must be 4 characters of y/n."),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let complex_wires = ComplexWires {
            has_blue: bools[0],
            has_red: bools[1],
            had_star: bools[2],
            has_led: bools[3],
        };

        let is_cut = complex_wires.is_cut();
        let s = match is_cut {
            true => "Cut it!",
            false => "Don't cut!",
        };
        println!("{s}");
        Ok(())
    }
}
