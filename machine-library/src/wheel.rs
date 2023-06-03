use crate::character::Character;

/// Represents a mapping between two characters, used to configure the enigma machine for encoding or decoding
pub struct CharacterMapping(Character, Character);

/// Represents the data for a wheel, used to configure the enigma machine for encoding or decoding
pub struct WheelData {
    pub mapping: [CharacterMapping; 26],
}

impl WheelData {
    pub fn new(mapping: [CharacterMapping; 26]) -> Self {
        WheelData { mapping }
    }

    pub fn encoding_wheel(&self, position: usize) -> Wheel {
        let mut transform = [0; 26];

        for i in 0..26 {
            let from = self.mapping[i].0.to_u8();
            let to = self.mapping[i].1.to_u8();
            transform[from as usize] = to;
        }

        Wheel::new(WheelTransformer::new(transform), position)
    }

    pub fn decoding_wheel(&self, position: usize) -> Wheel {
        let mut transform = [0; 26];

        for i in 0..26 {
            let from = self.mapping[i].0.to_u8();
            let to = self.mapping[i].1.to_u8();
            transform[to as usize] = from;
        }

        Wheel::new(WheelTransformer::new(transform), position)
    }
}

pub struct WheelTransformer {
    pub transform: [u8; 26],
}

impl WheelTransformer {
    pub fn new(transform: [u8; 26]) -> Self {
        WheelTransformer { transform }
    }

    #[inline(always)]
    pub fn transform(&self, input: u8) -> u8 {
        self.transform[input as usize]
    }
}

/// Represents a wheel in the enigma machine, can only encode or decode a single character, depending on how it is configured
pub struct Wheel {
    pub position: usize,
    pub transformer: WheelTransformer,
}

impl Wheel {
    pub fn new(transformer: WheelTransformer, position: usize) -> Self {
        Wheel {
            position,
            transformer,
        }
    }

    #[inline(always)]
    pub fn transform(&self, input: u8) -> u8 {
        let input = (input + self.position as u8) % 26;

        self.transformer.transform(input)
    }

    /// Rotates the wheel by one position, returning true if the wheel has completed a full rotation
    pub fn rotate(&mut self) -> bool {
        self.position = self.position + 1;

        if self.position >= 26 {
            self.position = self.position - 26;
            return true;
        }

        return false;
    }

    /// Rotates the wheel by the specified amount, returns the amount of turns the wheel has completed
    pub fn rotate_amount(&mut self, input: u8) -> usize {
        let pos = self.position + input as usize;
        self.position = pos % 26;

        return pos / 26;
    }
}
