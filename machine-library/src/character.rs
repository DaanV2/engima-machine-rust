/// Character is a type that represents a single character in the alphabet of a machine. value between 0 and 26
pub struct Character {
    /// value between 0 and 26
    value: u8,
}

impl Character {
    pub const MIN: Character = Character { value: 0 };
    pub const MAX: Character = Character { value: 25 };

    pub const A: Character = Character { value: 0 };
    pub const B: Character = Character { value: 1 };
    pub const C: Character = Character { value: 2 };
    pub const D: Character = Character { value: 3 };
    pub const E: Character = Character { value: 4 };
    pub const F: Character = Character { value: 5 };
    pub const G: Character = Character { value: 6 };
    pub const H: Character = Character { value: 7 };
    pub const I: Character = Character { value: 8 };
    pub const J: Character = Character { value: 9 };
    pub const K: Character = Character { value: 10 };
    pub const L: Character = Character { value: 11 };
    pub const M: Character = Character { value: 12 };
    pub const N: Character = Character { value: 13 };
    pub const O: Character = Character { value: 14 };
    pub const P: Character = Character { value: 15 };
    pub const Q: Character = Character { value: 16 };
    pub const R: Character = Character { value: 17 };
    pub const S: Character = Character { value: 18 };
    pub const T: Character = Character { value: 19 };
    pub const U: Character = Character { value: 20 };
    pub const V: Character = Character { value: 21 };
    pub const W: Character = Character { value: 22 };
    pub const X: Character = Character { value: 23 };
    pub const Y: Character = Character { value: 24 };
    pub const Z: Character = Character { value: 25 };

    #[inline(always)]
    pub fn from_u8(c: u8) -> Character {
        Character { value: c }
    }

    #[inline(always)]
    pub fn to_u8(&self) -> u8 {
        self.value
    }

    pub fn is_valid(&self) -> bool {
        self.value <= Character::MAX.value
    }

    const CHAR_UPPER_A: u8 = 'A' as u8;
    const CHAR_LOWER_A: u8 = 'a' as u8;

    #[inline(always)]
    pub fn to_char(&self) -> char {
        (self.value + Character::CHAR_UPPER_A) as char
    }

    #[inline(always)]
    pub fn from_char(c: char) -> Character {
        //If a - z then use lower case offset
        if c >= 'a' && c <= 'z' {
            Character {
                value: (c as u8) - Character::CHAR_LOWER_A,
            }
        } else {
            Character {
                value: (c as u8) - Character::CHAR_UPPER_A,
            }
        }
    }
}

#[cfg(test)]
mod tests {

    //Test the struct is 1 byte large
    #[test]
    fn test_size() {
        use std::mem::size_of;
        assert_eq!(size_of::<super::Character>(), 1);
    }
}
