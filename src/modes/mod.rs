pub(super) mod wires {
    use bomb::wires::Wires;

    use crate::read_string;

    pub fn wires_mode() -> anyhow::Result<()> {
        println!("Enter wire colors from above to bottom using format like 'rwbdy' for red, white, blue, dark, yellow\n");
        let s = read_string()?;
        let wires = Wires::new(&s)?;
        let nth = wires.get_cut_num()?;
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
        let complex_wires = ComplexWires::new(&s)?;
        let is_cut = complex_wires.is_cut()?;
        let s = match is_cut {
            true => "Cut it!",
            false => "Don't cut!",
        };
        println!("{s}");
        Ok(())
    }
}

pub(super) mod button {
    use bomb::button::Button;

    use crate::read_string;

    pub fn button_mode() -> anyhow::Result<()> {
        println!("Enter button color and label using format like 'r abort|中止' for red, abort\n");
        let s = read_string()?;
        let button = Button::new(&s)?;
        let countermeasure = button.get_countermeasure()?;
        println!("{countermeasure}",);
        Ok(())
    }
}
