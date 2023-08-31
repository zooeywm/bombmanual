use crate::bomb_info::BombInfo;

enum Tactic {
    C,
    D,
    S,
    P,
    B,
}

pub struct ComplexWires {
    pub has_blue: bool,
    pub has_red: bool,
    pub had_star: bool,
    pub has_led: bool,
}

impl ComplexWires {
    fn get_tactic(&self) -> Tactic {
        use Tactic::*;

        let Self {
            has_blue,
            has_red,
            had_star,
            has_led,
        } = self;

        match (
            *has_blue as u8,
            *has_red as u8,
            *had_star as u8,
            *has_led as u8,
        ) {
            (0, 0, 0, 0) => C,
            (0, 0, 0, 1) => D,
            (0, 0, 1, 0) => C,
            (0, 0, 1, 1) => B,
            (0, 1, 0, 0) => S,
            (0, 1, 0, 1) => B,
            (0, 1, 1, 0) => C,
            (0, 1, 1, 1) => B,
            (1, 0, 0, 0) => S,
            (1, 0, 0, 1) => P,
            (1, 0, 1, 0) => D,
            (1, 0, 1, 1) => P,
            (1, 1, 0, 0) => S,
            (1, 1, 0, 1) => S,
            (1, 1, 1, 0) => P,
            (1, 1, 1, 1) => D,
            _ => unreachable!(),
        }
    }

    pub fn is_cut(&self) -> bool {
        use Tactic::*;
        let tactic = self.get_tactic();
        match tactic {
            C => false,
            D => true,
            S => BombInfo::is_serial_last_even(),
            P => BombInfo::has_parallel_port(),
            B => BombInfo::batteries_count() >= 2,
        }
    }
}
