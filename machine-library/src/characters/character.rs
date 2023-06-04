#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Character {
    value: u8,
}

impl Character {
    #[inline(always)]
    pub fn new(value: u8) -> Self {
        Character { value }
    }

    pub fn is_valid(&self) -> bool {
        self.value <= Character::Z.value
    }

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
}

impl From<u8> for Character {
    #[inline(always)]
    fn from(value: u8) -> Self {
        Character { value }
    }
}

impl Into<u8> for Character {
    #[inline(always)]
    fn into(self) -> u8 {
        self.value
    }
}

const CHAR_A_UPPERCASE: u8 = b'A';
const CHAR_A_LOWERCASE: u8 = b'a';

impl From<char> for Character {
    fn from(value: char) -> Self {
        let mut v: u8 = value as u8;

        v = match v >= CHAR_A_LOWERCASE {
            true => v - CHAR_A_LOWERCASE,
            false => v - CHAR_A_UPPERCASE,
        };

        Character { value: v }
    }
}

impl Into<char> for Character {
    #[inline(always)]
    fn into(self) -> char {
        let mut v = self.value;
        v += CHAR_A_UPPERCASE;

        v as char
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_constants() {
        assert_eq!(Character::A, Character { value: 0 });
        assert_eq!(Character::Z, Character { value: 25 });
    }

    #[test]
    pub fn test_from_u8() {
        assert_eq!(Character::from(0), Character::A);
        assert_eq!(Character::from(25), Character::Z);
    }

    #[test]
    pub fn test_into_u8() {
        assert_eq!(<Character as Into<u8>>::into(Character::A), 0);
        assert_eq!(<Character as Into<u8>>::into(Character::Z), 25);
    }

    #[test]
    pub fn test_from_char() {
        assert_eq!(Character::from('A'), Character::A);
        assert_eq!(Character::from('Z'), Character::Z);
        assert_eq!(Character::from('a'), Character::A);
        assert_eq!(Character::from('z'), Character::Z);
    }

    #[test]
    pub fn test_into_char() {
        assert_eq!(<Character as Into<char>>::into(Character::A), 'A');
        assert_eq!(<Character as Into<char>>::into(Character::Z), 'Z');
    }
}
