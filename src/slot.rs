use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Slot {
    _valid: HashSet<char>,
}

impl Slot {
    /// Removes the char as a valid character for this slot
    pub fn remove(&mut self, c: &char) {
        self._valid.remove(c);
    }

    /// Restricts the slot to only allow this character
    pub fn restrict(&mut self, c: char) {
        self._valid.clear();
        self._valid.insert(c);
    }

    /// Checks if a character is in the slot's accepted character list
    pub fn contains(&self, c: &char) -> bool {
        self._valid.contains(c)
    }

    /// Creates a new Slot that will allow any character
    pub fn new() -> Self {
        Slot {
            _valid: HashSet::from_iter('a'..='z'),
        }
    }
}

impl FromIterator<char> for Slot {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Slot {
            _valid: HashSet::from_iter(iter),
        }
    }
}
