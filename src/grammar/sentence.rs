use super::letter::Letter;

pub struct Sentence {
    pub sentence: Vec<Letter>
}

impl Sentence {
    pub fn from_slice(data: &[Letter]) -> Sentence {
        Sentence {
            sentence: data.iter().cloned().collect()
        }
    }

    pub fn push(&mut self, letter: Letter) {
        self.sentence.push(letter);
    }
}