const MORSES: &[&str] = &[
    "shell", "halls", "slick", "trick", "boxes", "leaks", "strobe", "bistro", "flick", "bombs",
    "break", "brick", "steak", "sting", "vector", "beats",
];

/// Line mode
#[derive(Default)]
pub struct Morse {
    chars: Vec<char>,
}

impl Morse {
    pub fn answer(&mut self, char: char) -> Vec<&'static str> {
        let chars = &mut self.chars;
        if !chars.contains(&char) {
            chars.push(char);
        }
        MORSES
            .iter()
            .filter(|&&morse| {
                let mut morse_contains_all_chars = true;
                chars.iter().for_each(|char| {
                    if !morse.chars().any(|c| &c == char) {
                        morse_contains_all_chars = false
                    }
                });
                morse_contains_all_chars
            })
            .cloned()
            .collect()
    }
}
