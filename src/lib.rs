#![feature(test)]

use std::usize;

extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
extern crate web_sys;

use crate::CameFrom::Match;
use crate::CameFrom::SkipA;
use crate::CameFrom::SkipB;


extern crate test;
use test::Bencher;


#[wasm_bindgen]
#[derive(Debug)]
pub struct ScoredAlignment {
    score: isize,
    alignment: Vec<(Option<char>, Option<char>)>,
}

#[wasm_bindgen]
impl ScoredAlignment {
    pub fn score(&self) -> isize {
        self.score
    }

    pub fn alignment(&self) -> String {
        self.alignment
            .iter()
            .map(|v| format!("{}{}", v.0.unwrap_or('_'), v.1.unwrap_or('_')))
            .collect()
    }
}

pub trait ScoringRubric<T: PartialEq> {
    fn compare(&self, a: Option<&T>, b: Option<&T>) -> i8 {
        if a.is_some() != b.is_some() {
            -1 // gap in alignment
        } else {
            if a.unwrap() == b.unwrap() {
                1 // match
            } else {
                -1 // mismatch
            }
        }
    }
}

struct SimpleScoringRubric {}
impl<T: PartialEq> ScoringRubric<T> for SimpleScoringRubric {}

#[derive(Clone, Copy, Debug)]
pub enum CameFrom {
    Match,
    SkipA,
    SkipB,
}

#[wasm_bindgen]
pub struct AlignmentTable {
    a: String,
    b: String,
    alignment_matrix: Vec<Option<(isize, CameFrom)>>,
    scoring_rubric: Box<dyn ScoringRubric<char>>,
}

#[wasm_bindgen]
impl AlignmentTable {
    pub fn new(a: &str) -> AlignmentTable {
        init_panic_hook();
        let blen = (a.len() as f32 * 1.25) as usize;
        let capacity = (1 + a.len()) * (1 + blen);
        let scratch: Vec<Option<(isize, CameFrom)>> = vec![None; capacity];
        let mut ret = AlignmentTable {
            b: String::with_capacity(blen),
            a: a.to_string(),
            alignment_matrix: scratch,
            scoring_rubric: Box::new(SimpleScoringRubric {}),
        };
        ret.initialize();
        ret.align(0);
        ret
    }

    fn get_alignment_at(&self, (r, c): (usize, usize)) -> Option<(isize, CameFrom)> {
        self.alignment_matrix[r * (self.a.len() + 1) + c]
    }

    fn set_alignment_at(&mut self, (r, c): (usize, usize), v: (isize, CameFrom)) {
        self.alignment_matrix[r * (self.a.len() + 1) + c] = Some(v)
    }

    fn initialize(&mut self) {
        for c in 0..=self.a.len() {
            self.set_alignment_at((0, c), (-1 * (c as isize), SkipA))
        }
        for r in 0..=self.b.len() {
            self.set_alignment_at((r, 0), (-1 * (r as isize), SkipB))
        }
    }

    fn initialize_new_rows(&mut self, new_rows: usize) {
        for r in self.b.len() - new_rows..=self.b.len() {
            self.set_alignment_at((r, 0), (-1 * (r as isize), SkipB))
        }
    }

    pub fn score_at(&self, row: usize, col: usize) -> isize {
        let ret = self.get_alignment_at((row, col)).unwrap().0;
        ret
    }

    pub fn replace_b(&mut self, bitems: &str) -> usize {
        let common_prefix = bitems
            .chars()
            .zip(self.b.chars())
            .take_while(|(a, b)| a == b)
            .count();

        self.b.truncate(common_prefix);

        self.b.push_str(&bitems[common_prefix..]);

        self.initialize_new_rows(bitems.len() - common_prefix);

        self.align(common_prefix);

        common_prefix
    }

    pub fn type_into_b(&mut self, bitems: &str) -> usize {
        self.b.push_str(bitems);
        self.initialize_new_rows(bitems.len());
        self.align(self.b.len() - bitems.len());
        self.b.len()
    }

    pub fn backspace_into_b(&mut self, count: usize) -> usize {
        for _c in 0..count {
            self.b.pop();
        }
        self.b.len()
    }

    pub fn backword_into_b(&mut self, count: usize) -> usize {
        for _c in 0..count {
            while self.b.pop().unwrap_or('_') == ' ' {}
            while self.b.pop().unwrap_or(' ') != ' ' {}
        }
        if self.b.len() > 0 {
            self.b.push(' ')
        }
        self.b.len()
    }

    pub fn align(&mut self, previous_b_length: usize) -> Option<bool> {
        let start_from = previous_b_length + 1;

        for r in start_from..=self.b.len() {
            for c in 1..=self.a.len() {
                let a0 = self.a.as_bytes()[(c - 1)] as char;
                let b0 = self.b.as_bytes()[(r - 1)] as char;
                let a = Some(&a0);
                let b = Some(&b0);
                let advance_both =
                    self.score_at(r - 1, c - 1) + self.scoring_rubric.compare(a, b) as isize;
                let advance_a =
                    self.score_at(r, c - 1) + self.scoring_rubric.compare(a, None) as isize;
                let advance_b =
                    self.score_at(r - 1, c) + self.scoring_rubric.compare(None, b) as isize;
                let best = *[
                    (advance_both, Match),
                    (advance_a, SkipA),
                    (advance_b, SkipB),
                ]
                .iter()
                .max_by_key(|v| v.0)
                .unwrap();

                self.set_alignment_at((r, c), best);
            }
        }

        Some(true) // Some(()) isn't supported by wasm-bindgen
    }

    pub fn best_scored_alignment(&self) -> ScoredAlignment {
        let best_col = (0..=self.a.len())
            .max_by_key(|&c| self.score_at(self.b.len(), c))
            .unwrap();
        let final_position = (self.b.len(), best_col);
        let mut current_position = final_position;
        let mut best_alignment = vec![];
        while current_position != (0usize, 0usize) {
            let prev_step = self
                .get_alignment_at((current_position.0, current_position.1))
                .unwrap()
                .1;
            match prev_step {
                Match => {
                    best_alignment.push((
                        Some(self.a.as_bytes()[current_position.1 - 1] as char),
                        Some(self.b.as_bytes()[current_position.0 - 1] as char),
                    ));
                    current_position = (current_position.0 - 1, current_position.1 - 1);
                }
                SkipA => {
                    best_alignment.push((
                        Some(self.a.as_bytes()[current_position.1 - 1] as char),
                        None,
                    ));
                    current_position = (current_position.0, current_position.1 - 1);
                }
                SkipB => {
                    best_alignment.push((
                        None,
                        Some(self.b.as_bytes()[current_position.0 - 1] as char),
                    ));
                    current_position = (current_position.0 - 1, current_position.1);
                }
            };
        }
        let ret = ScoredAlignment {
            score: self.score_at(final_position.0, final_position.1),
            alignment: best_alignment.into_iter().rev().collect(),
        };
        ret
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}


#[bench]
fn late_chars(b: &mut test::Bencher) {
    let copy = "The male begins courtship by flying noisily, and then in a graceful, circular glide with its wings outstretched and head down. After landing, the male will go to the female with a puffed out breast, bobbing head, and loud calls. Once the pair is mated, they will often spend time preening each other's feathers.";
    let mut t = AlignmentTable::new(copy);
    b.iter(|| {
        for _i in 0..1000 {
            t.replace_b("The male begins courtship by flying noisily, and then in a graceful, circular glide with its wings outstretched and head down. After landing, the male will go to the female with a puffed out breast, bobbing head, and loud calls. Once the pair is mated, they will often spend time preening each other's ");
            t.replace_b("The male begins courtship by flying noisily, and then in a graceful, circular glide with its wings outstretched and head down. After landing, the male will go to the female with a puffed out breast, bobbing head, and loud calls. Once the pair is mated, they will often spend time preening each other's f");
        }
    })
    //t.type_into_b(" enabled");
}


#[cfg(test)]
mod tests {
    use crate::AlignmentTable;

    #[test]
    fn it_works() {
        let target = "If enacted";
        let mut t = AlignmentTable::new(&target);
        t.type_into_b("If enabled");
        //t.type_into_b(" enabled");
        //let replaced = t.replace_b("If enabled");
        println!(
            "SCoredAlign {} scopre {:?}",
            t.best_scored_alignment().score,
            t.best_scored_alignment().alignment()
        );
        assert_eq!(t.best_scored_alignment().score, 6);
        //println!("Repalced {}", replaced);
    }
}
