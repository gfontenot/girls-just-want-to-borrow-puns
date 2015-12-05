pub struct Rhyme {
    word: String,
    freq: i8,
    score: i16,
    flags: String,
    syllables: i8,
}

impl Rhyme {
    pub fn new(word: &str, freq: i8, score: i16, flags: &str, syllables: i8) -> Rhyme {
        return Rhyme {
            word: word.to_string(),
            freq: 1,
            score: 300,
            flags: flags.to_string(),
            syllables: 1
        };
    }
}
