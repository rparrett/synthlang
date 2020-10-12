#![allow(clippy::non_ascii_literal)]

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use std::fmt;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
enum CompoundRule {
    DropLeft,
    DropRight,
    DropNone,
}

#[derive(Debug, Clone)]
enum SyllableType {
    VC,
    CV,
    CVC,
}

#[derive(Debug, Clone)]
enum SyllablePartType {
    Consonant,
    Vowel,
}

#[derive(Debug, Clone)]
struct SyllablePart {
    part_type: SyllablePartType,
    value: String,
}

#[derive(Debug, Clone)]
struct Syllable {
    parts: Vec<SyllablePart>,
}

#[derive(Debug, Clone)]
pub struct Word {
    parts: Vec<Syllable>,
    compound_rule: CompoundRule,
}

#[derive(Debug)]
pub struct SynthLang {
    pub consonants: Vec<String>,
    pub vowels: Vec<String>,
    pub vc_weight: i32,
    pub cv_weight: i32,
    pub cvc_weight: i32,
    rng: Pcg64,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut word = String::new();

        for part in &self.parts {
            let formatted = format!("{}", part);
            word.push_str(&formatted);
        }

        write!(f, "{}", SynthLang::remove_repeated_chars(&word))
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for part in &self.parts {
            write!(f, "{}", part)?;
        }
        Ok(())
    }
}

impl fmt::Display for SyllablePart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl SynthLang {
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn new(seed: u64) -> Self {
        let consonants = vec![
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "j".to_string(),
            "k".to_string(),
            "l".to_string(),
            "m".to_string(),
            "n".to_string(),
            "p".to_string(),
            "q".to_string(),
            "r".to_string(),
            "s".to_string(),
            "t".to_string(),
            "v".to_string(),
            "w".to_string(),
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
            "ng".to_string(),
            "sh".to_string(),
            "th".to_string(),
            "ch".to_string(),
        ];

        let spice = vec![
            // TODO very incomplete
            ("ñ".to_string(), SyllablePartType::Consonant),
            ("ń".to_string(), SyllablePartType::Consonant),
            ("ŋ".to_string(), SyllablePartType::Consonant),
            ("ç".to_string(), SyllablePartType::Consonant),
            ("ð".to_string(), SyllablePartType::Consonant),
            ("š".to_string(), SyllablePartType::Consonant),
            ("ś".to_string(), SyllablePartType::Consonant),
            ("đ".to_string(), SyllablePartType::Consonant),
            ("ġ".to_string(), SyllablePartType::Consonant),
            ("ł".to_string(), SyllablePartType::Consonant),
            ("ŕ".to_string(), SyllablePartType::Consonant),
            ("ĥ".to_string(), SyllablePartType::Consonant),
            ("ĵ".to_string(), SyllablePartType::Consonant),
            ("ć".to_string(), SyllablePartType::Consonant),
            ("ĉ".to_string(), SyllablePartType::Consonant),
            ("ź".to_string(), SyllablePartType::Consonant),
            ("ż".to_string(), SyllablePartType::Consonant),
            ("ẅ".to_string(), SyllablePartType::Consonant),
            ("ŵ".to_string(), SyllablePartType::Consonant),
            // TODO probably incomplete
            ("à".to_string(), SyllablePartType::Vowel),
            ("á".to_string(), SyllablePartType::Vowel),
            ("â".to_string(), SyllablePartType::Vowel),
            ("ã".to_string(), SyllablePartType::Vowel),
            ("ä".to_string(), SyllablePartType::Vowel),
            ("å".to_string(), SyllablePartType::Vowel),
            ("ā".to_string(), SyllablePartType::Vowel),
            ("ă".to_string(), SyllablePartType::Vowel),
            ("è".to_string(), SyllablePartType::Vowel),
            ("é".to_string(), SyllablePartType::Vowel),
            ("ê".to_string(), SyllablePartType::Vowel),
            ("ë".to_string(), SyllablePartType::Vowel),
            ("ē".to_string(), SyllablePartType::Vowel),
            ("ĕ".to_string(), SyllablePartType::Vowel),
            ("ė".to_string(), SyllablePartType::Vowel),
            ("ě".to_string(), SyllablePartType::Vowel),
            ("ì".to_string(), SyllablePartType::Vowel),
            ("í".to_string(), SyllablePartType::Vowel),
            ("î".to_string(), SyllablePartType::Vowel),
            ("ï".to_string(), SyllablePartType::Vowel),
            ("ĩ".to_string(), SyllablePartType::Vowel),
            ("ī".to_string(), SyllablePartType::Vowel),
            ("ĭ".to_string(), SyllablePartType::Vowel),
            ("ı".to_string(), SyllablePartType::Vowel),
            ("ò".to_string(), SyllablePartType::Vowel),
            ("ó".to_string(), SyllablePartType::Vowel),
            ("ô".to_string(), SyllablePartType::Vowel),
            ("õ".to_string(), SyllablePartType::Vowel),
            ("ö".to_string(), SyllablePartType::Vowel),
            ("ō".to_string(), SyllablePartType::Vowel),
            ("ŏ".to_string(), SyllablePartType::Vowel),
            ("ő".to_string(), SyllablePartType::Vowel),
            ("ø".to_string(), SyllablePartType::Vowel),
            ("ù".to_string(), SyllablePartType::Vowel),
            ("ú".to_string(), SyllablePartType::Vowel),
            ("û".to_string(), SyllablePartType::Vowel),
            ("ü".to_string(), SyllablePartType::Vowel),
            ("ũ".to_string(), SyllablePartType::Vowel),
            ("ū".to_string(), SyllablePartType::Vowel),
            ("ŭ".to_string(), SyllablePartType::Vowel),
            ("ů".to_string(), SyllablePartType::Vowel),
            ("ű".to_string(), SyllablePartType::Vowel),
        ];

        let vowels = vec![
            "a".to_string(),
            "e".to_string(),
            "i".to_string(),
            "o".to_string(),
            "u".to_string(),
        ];

        let mut rng = Pcg64::seed_from_u64(seed);

        let mut dipthongs = vec!["æ".to_string(), "œ".to_string()];
        for v1 in &vowels {
            for v2 in &vowels {
                dipthongs.push(format!("{}{}", v1, v2))
            }
        }

        let mut our_vowels: Vec<String> = vowels.choose_multiple(&mut rng, 6).cloned().collect();

        let mut our_consonants: Vec<String> =
            consonants.choose_multiple(&mut rng, 16).cloned().collect();

        for s in spice.choose_multiple(&mut rng, 2) {
            match s.1 {
                SyllablePartType::Vowel => {
                    our_vowels.push(s.0.clone());
                }
                SyllablePartType::Consonant => {
                    our_consonants.push(s.0.clone());
                }
            }
        }

        let mut our_dipthongs: Vec<String> =
            dipthongs.choose_multiple(&mut rng, 3).cloned().collect();

        our_vowels.append(&mut our_dipthongs);

        let possible_weights = [0, 25, 50, 100];
        let mut weights = (
            *possible_weights.choose(&mut rng).unwrap(),
            *possible_weights.choose(&mut rng).unwrap(),
            *possible_weights.choose(&mut rng).unwrap(),
        );

        if (0, 0, 0) == weights {
            weights.0 = 25;
            weights.1 = 25;
            weights.2 = 25;
        }

        Self {
            consonants: our_consonants,
            vowels: our_vowels,
            cv_weight: weights.0,
            vc_weight: weights.1,
            cvc_weight: weights.2,
            rng,
        }
    }

    fn syllable(&mut self) -> Syllable {
        let choices = [
            (SyllableType::CV, self.cv_weight),
            (SyllableType::VC, self.vc_weight),
            (SyllableType::CVC, self.cvc_weight),
        ];
        let syllable_type = &choices
            .choose_weighted(&mut self.rng, |item| item.1)
            .unwrap()
            .0;

        let mut parts = vec![];

        match syllable_type {
            SyllableType::CV => {
                parts.push(SyllablePart {
                    part_type: SyllablePartType::Consonant,
                    value: self.consonants.choose(&mut self.rng).unwrap().to_string(),
                });
                parts.push(SyllablePart {
                    part_type: SyllablePartType::Vowel,
                    value: self.vowels.choose(&mut self.rng).unwrap().to_string(),
                });
            }
            SyllableType::VC => {
                parts.push(SyllablePart {
                    part_type: SyllablePartType::Vowel,
                    value: self.vowels.choose(&mut self.rng).unwrap().to_string(),
                });
                parts.push(SyllablePart {
                    part_type: SyllablePartType::Consonant,
                    value: self.consonants.choose(&mut self.rng).unwrap().to_string(),
                });
            }
            SyllableType::CVC => {
                parts.push(SyllablePart {
                    part_type: SyllablePartType::Consonant,
                    value: self.consonants.choose(&mut self.rng).unwrap().to_string(),
                });
                parts.push(SyllablePart {
                    part_type: SyllablePartType::Vowel,
                    value: self.vowels.choose(&mut self.rng).unwrap().to_string(),
                });
                parts.push(SyllablePart {
                    part_type: SyllablePartType::Consonant,
                    value: self.consonants.choose(&mut self.rng).unwrap().to_string(),
                });
            }
        }

        Syllable { parts }
    }

    pub fn word(&mut self) -> Word {
        let mut syllables = vec![];

        let choices = [(1, 1), (2, 2)];
        let num_syllables = choices
            .choose_weighted(&mut self.rng, |item| item.1)
            .unwrap()
            .0;

        for _ in 0..num_syllables {
            syllables.push(self.syllable());
        }

        // 50/50 drop something
        let compound_rule = match self.rng.gen_range(0, 4) {
            0 => CompoundRule::DropLeft,
            1 => CompoundRule::DropRight,
            _ => CompoundRule::DropNone,
        };

        Word {
            parts: syllables,
            compound_rule,
        }
    }

    pub fn compound(&mut self, left: &Word, right: &Word) -> Word {
        let mut new = vec![];

        match left.compound_rule {
            CompoundRule::DropLeft if left.parts.len() >= 2 => {
                new.extend(left.parts.iter().skip(1).cloned());
            }
            CompoundRule::DropRight if left.parts.len() >= 2 => {
                new.extend(left.parts.iter().rev().skip(1).rev().cloned());
            }
            _ => {
                new.extend(left.parts.iter().cloned());
            }
        }

        match right.compound_rule {
            CompoundRule::DropLeft if right.parts.len() >= 2 => {
                new.extend(right.parts.iter().skip(1).cloned());
            }
            CompoundRule::DropRight if right.parts.len() >= 2 => {
                new.extend(right.parts.iter().rev().skip(1).rev().cloned());
            }
            _ => {
                new.extend(right.parts.iter().cloned());
            }
        }

        // 50/50 drop something
        let compound_rule = match self.rng.gen_range(0, 4) {
            0 => CompoundRule::DropLeft,
            1 => CompoundRule::DropRight,
            _ => CompoundRule::DropNone,
        };

        Word {
            parts: new,
            compound_rule,
        }
    }

    fn remove_repeated_chars(input: &str) -> String {
        let mut output = String::new();
        let mut prev = '\0';
        let mut count = 1;

        for c in input.chars() {
            if c == prev {
                count += 1
            } else {
                count = 1
            }

            if count <= 2 {
                output.push(c)
            }

            prev = c
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use heck::TitleCase;

    #[test]
    fn it_works() {
        let mut r = SynthLang::new(3);

        println!("{:?}", r);
        println!();
        println!(
            "The distinguished language of {}",
            r.word().to_string().to_title_case()
        );

        println!();
        println!("{:10}{:10}", r.word().to_string(), "dress");
        println!("{:10}{:10}", r.word().to_string(), "child");
        println!("{:10}{:10}", r.word().to_string(), "shoes");
        println!("{:10}{:10}", r.word().to_string(), "orange");
        println!("{:10}{:10}", r.word().to_string(), "note");
        println!("{:10}{:10}", r.word().to_string(), "cake");
        println!("{:10}{:10}", r.word().to_string(), "soup");
        println!("{:10}{:10}", r.word().to_string(), "soldier");
        println!("{:10}{:10}", r.word().to_string(), "reporter");
        println!();

        let green = r.word();
        let red = r.word();
        let mountain = r.word();
        let stream = r.word();

        println!(
            "{:20}{:10}{:10}{:20}",
            r.compound(&green, &mountain).to_string().to_title_case(),
            green.to_string(),
            mountain.to_string(),
            "Green Mountain",
        );
        println!(
            "{:20}{:10}{:10}{:20}",
            r.compound(&green, &stream).to_string().to_title_case(),
            green.to_string(),
            stream.to_string(),
            "Green River",
        );
        println!(
            "{:20}{:10}{:10}{:20}",
            r.compound(&red, &mountain).to_string().to_title_case(),
            red.to_string(),
            mountain.to_string(),
            "Red Mountain",
        );
        println!(
            "{:20}{:10}{:10}{:20}",
            r.compound(&red, &stream).to_string().to_title_case(),
            red.to_string(),
            stream.to_string(),
            "Red River",
        );
        println!();

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn repeats() {
        let s = "aaa bab cccccc ok".to_string();
        assert_eq!(
            SynthLang::remove_repeated_chars(&s),
            "aa bab cc ok".to_string()
        );
    }
}
