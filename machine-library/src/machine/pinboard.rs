use crate::characters::{
    character::Character,
    transform::{CharacterTransform, Transform, DEFAULT_MAPPING_TABLE},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pin(Character, Character);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinBoard {
    transform: Transform,
}

impl CharacterTransform for PinBoard {
    fn transform(&self, c: u8) -> u8 {
        self.transform.transform(c)
    }
}

impl PinBoard {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }

    pub fn empty() -> Self {
        Self::new(Transform::from_table(DEFAULT_MAPPING_TABLE))
    }

    pub fn from_table(table: &str) -> Self {
        Self::new(Transform::from_table(table))
    }

    pub fn to_table(&self) -> String {
        self.transform.to_table()
    }

    pub fn set_pin(&mut self, pin: Pin) {
        let Pin(a, b) = pin;
        let from: u8 = a.into();
        let to: u8 = b.into();

        self.transform.map[from as usize] = to;
        self.transform.map[to as usize] = from;
    }

    pub fn get_pins(&self) -> Vec<Pin> {
        let mut pins = Vec::new();

        for (from, &to) in self.transform.map.iter().enumerate() {
            if from == (to as usize) {
                continue;
            }

            let from = from as u8;
            let to = to as u8;

            let first = from.min(to);
            let second = from.max(to);

            let pin = Pin(Character::new(first), Character::new(second));

            // Check if the pin is already in the vector
            if !pins.contains(&pin) {
                pins.push(pin);
            }
        }

        pins
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_pins() {
        let mut pinboard = super::PinBoard::empty();

        pinboard.set_pin(super::Pin('A'.into(), 'Z'.into()));

        let pins = pinboard.get_pins();

        assert_eq!(pins.len(), 1);
        assert_eq!(pins[0], super::Pin('A'.into(), 'Z'.into()));
    }
}
