pub(super) use {
    button::button_mode, complex_wires::complex_wires_mode, morse::morse_mode,
    password::password_mode, wires::wires_mode,
};

mod wires {
    use bomb::Wires;

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

mod complex_wires {
    use bomb::ComplexWires;

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

mod button {
    use bomb::Button;

    use crate::read_string;

    pub fn button_mode() -> anyhow::Result<()> {
        println!("Enter button color and label using format like 'r abort|中止' for red, abort\n");
        let s = read_string()?;
        let button = Button::new(&s)?;
        let counter_measure = button.get_counter_measure()?;
        println!("{counter_measure}",);
        Ok(())
    }
}

mod password {
    use bomb::Password;

    use crate::read_string;

    pub fn password_mode() -> anyhow::Result<()> {
        let mut password = Password::default();
        let mut i = 0_u8;
        while i < 5 {
            println!("Please input position {} chars", i + 1);
            let chars = read_string()?.chars().collect::<Vec<_>>();
            if let Some(answer) = password.answer(chars, i) {
                println!("answer: {answer}");
                break;
            }
            i += 1;
        }
        Ok(())
    }
}

mod morse {
    use bomb::Morse;

    use crate::read_char;

    pub fn morse_mode() -> anyhow::Result<()> {
        let mut morse = Morse::default();
        loop {
            println!("Please input morse char, input 0 for break.");
            let char = read_char()?;
            if char == '0' {
                break;
            }

            let morses = morse.answer(char);
            println!("morses:\n{morses:#?}");
        }
        Ok(())
    }
}
