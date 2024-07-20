use std::char;

const PASSWORDS: &[&str] = &[
    "about", "after", "again", "below", "could", "every", "first", "found", "great", "house",
    "large", "learn", "never", "other", "place", "plant", "point", "right", "small", "sound",
    "spell", "still", "study", "their", "there", "these", "thing", "think", "three", "water",
    "where", "which", "world", "would", "write",
];

/// Line mode
pub struct Password {
    candidates: Vec<&'static str>,
}

impl Password {
    pub fn answer(&mut self, chars: Vec<char>, position: u8) -> Option<&'static str> {
        let candidates = &mut self.candidates;
        candidates.retain(|&candidate| {
            chars.contains(&candidate.chars().nth(position as usize).unwrap())
        });
        if candidates.len() > 1 {
            None
        } else {
            Some(*candidates.first().unwrap())
        }
    }
}

impl Default for Password {
    fn default() -> Self {
        Self {
            candidates: PASSWORDS.to_vec(),
        }
    }
}
