use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader, path::Path};

use crate::{letter::Letter, slot::Slot};

#[derive(Debug, Clone, Default)]
pub struct Wordle {
    _wordlist: Vec<String>,
    _slots: [Slot; 5],
    _required: HashSet<char>,
}

#[derive(Debug)]
pub enum WordleParseError {
    InvalidToken(char),
    InvalidLength(usize),
}

impl Wordle {
    /// Applies a wordle line to update the filters
    ///
    /// Prefixing a letter with ! is a grey, and blocks it from appearing in the results, unless that letter is already marked as required by another slot.
    /// A lowercase letter is yellow, marking it as requried for the wordle, but blocking it from that slot.
    /// A uppercase letter is green, marking it as requried for the wordle and making it the only acceptable letter for that slot.
    pub fn update<S: Into<String>>(&mut self, s: S) -> Result<(), WordleParseError> {
        let line = Self::parse_line(s.into())?;

        for (idx, slot) in line.iter().enumerate() {
            match slot {
                // This slot may only have this character
                Letter::Green(c) => {
                    self._slots[idx].restrict(*c);
                    self._required.insert(*c);
                }

                // This slot can no longer have this character
                Letter::Yellow(c) => {
                    self._slots[idx].remove(c);
                    self._required.insert(*c);
                }

                // No slots may contain this character
                Letter::Grey(c) => {
                    for slot in &mut self._slots {
                        slot.remove(c);
                    }
                }
            }
        }

        self._wordlist.retain(|word| {
            ({
                let slots = &self._slots;
                word.chars().enumerate().all(|(i, c)| slots[i].contains(&c))
            }) && self._required.iter().all(|c| word.contains(*c))
        });

        Ok(())
    }

    /// Returns the length of the current wordlist
    pub fn len(&self) -> usize {
        self._wordlist.len()
    }

    pub fn words(&self) -> impl IntoIterator<Item = String> {
        self._wordlist.clone().into_iter()
    }

    fn parse_line<S: Into<String>>(s: S) -> Result<[Letter; 5], WordleParseError> {
        let mut letters = Vec::<Letter>::new();
        let mut block = false;

        for ch in s.into().chars() {
            if ch == '!' {
                block = true;
                continue;
            }

            if block {
                letters.push(Letter::Grey(ch));
                block = false;
            } else {
                letters.push(match ch {
                    '?' => Letter::Grey(' '),
                    c @ 'a'..='z' => Letter::Yellow(c),
                    c @ 'A'..='Z' => Letter::Green(c),
                    c => return Err(WordleParseError::InvalidToken(c)),
                })
            }
        }

        match <[Letter; 5]>::try_from(letters.as_slice()) {
            Ok(line) => Ok(line),
            Err(_) => Err(WordleParseError::InvalidLength(letters.len())),
        }
    }
}

impl<T> From<T> for Wordle
where
    T: AsRef<Path>,
{
    fn from(path: T) -> Self {
        let wordlist = File::open(path).expect("Failed to open wordlist");
        let buf = BufReader::new(wordlist);
        Wordle {
            _wordlist: buf
                .lines()
                .map(|w| w.unwrap())
                .filter(|w| w.len() == 5)
                .collect(),
            _slots: [
                Slot::new(),
                Slot::new(),
                Slot::new(),
                Slot::new(),
                Slot::new(),
            ],
            _required: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Letter::*;
    use crate::{slot::Slot, wordle::Wordle};

    fn filter<S: Into<String>>(word: S, slots: &[Slot; 5]) -> bool {
        word.into()
            .chars()
            .enumerate()
            .all(|(i, c)| slots[i].contains(&c))
    }

    #[test]
    fn parse() {
        assert_eq!(
            Wordle::parse_line("c!r!an!e").unwrap(),
            [Yellow('c'), Grey('r'), Grey('a'), Yellow('n'), Grey('e')]
        );
        assert_eq!(
            Wordle::parse_line("!p!lo!y!s").unwrap(),
            [Grey('p'), Grey('l'), Yellow('o'), Grey('y'), Grey('s')]
        );
    }

    #[test]
    fn filter_test() {
        let slots: [Slot; 5] = [
            Slot::from_iter('a'..='d'),
            Slot::from_iter('e'..='h'),
            Slot::from_iter('i'..='m'),
            Slot::from_iter('n'..='t'),
            Slot::from_iter('u'..='z'),
        ];

        assert!(filter("aeinu", &slots));
        assert!(!filter("zeinu", &slots));
    }

    #[test]
    fn tonic() {
        let mut wordle = Wordle::from("/home/alefir/.local/share/wordle_words");

        assert!(wordle.update("c!r!an!e").is_ok());
        assert!(wordle.update("!p!lo!y!s").is_ok());

        for word in wordle.words() {
            // Check that the C, O, and N that are required are present in all words
            // Check that none of the forbidden letters are present in any word
            let results = [
                word.contains('c'),
                word.contains('o'),
                word.contains('n'),
                !word.contains('r'),
                !word.contains('a'),
                !word.contains('e'),
                !word.contains('p'),
                !word.contains('l'),
                !word.contains('y'),
                !word.contains('s'),
            ];

            if results.iter().any(|t| !t) {
                println!("Failed at {}", word);
                assert!(false);
            }
        }
    }
}
