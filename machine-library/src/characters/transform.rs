use super::character::Character;

pub trait CharacterTransform {
    /// Returns the encrypted or decrypted character.
    fn transform(&self, c: u8) -> u8;

    /// Returns the encrypted or decrypted character.
    fn transform_char(&self, c: Character) -> Character {
        let c = c.into();
        let c = self.transform(c);
        Character::from(c)
    }
}

/// A string that maps a character to another character.
/// Mapped to: 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
pub type MappingTable = str;

pub const DEFAULT_MAPPING_TABLE: &MappingTable = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const DEFAULT_REVERSE_MAPPING_TABLE: &MappingTable = "ZYXWVUTSRQPONMLKJIHGFEDCBA";

/// A struct that maps a character to another character. Used for either encryption or decryption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transform {
    pub map: [u8; 26],
}

impl CharacterTransform for Transform {
    fn transform(&self, c: u8) -> u8 {
        let index = c as usize;

        self.map[index]
    }
}

impl Transform {
    /// Creates a new Transform struct.
    pub fn new(map: [u8; 26]) -> Transform {
        Transform { map }
    }

    pub fn from_table(table: &MappingTable) -> Transform {
        let mut map = [0; 26];
        for (i, c) in table.chars().enumerate() {
            let c = Character::from(c);
            map[i] = c.into();
        }
        Transform::new(map)
    }

    pub fn to_table(&self) -> String {
        let mut table = String::with_capacity(self.map.len());
        for c in self.map.iter() {
            let c = Character::from(*c);
            table.push(c.into());
        }
        table
    }

    /// Returns the reverse of the current Transform.
    /// If the current Transform is used for encryption, the reverse Transform is used for decryption.
    pub fn reverse(&self) -> Transform {
        let mut map = [0; 26];
        for (i, c) in self.map.iter().enumerate() {
            map[*c as usize] = i as u8;
        }
        Transform::new(map)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    pub fn test_struct_size() {
        assert_eq!(size_of::<Transform>(), 26);
    }

    #[test]
    pub fn test_to_and_from_table() {
        let table = DEFAULT_REVERSE_MAPPING_TABLE;
        let transform = Transform::from_table(table);
        assert_eq!(transform.to_table(), table);
    }

    #[test]
    pub fn test_transform() {
        let table = DEFAULT_REVERSE_MAPPING_TABLE;
        let transform = Transform::from_table(table);

        for i in 0..26 {
            let c = Character::from(i as u8);
            let c = transform.transform_char(c);
            assert_eq!(c, Character::from(25 - i as u8));
        }
    }
}
