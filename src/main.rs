extern crate rand;
use std::fmt;


#[derive(PartialEq)]
#[derive(Debug)]
enum Card {
    Defect,
    Cooperate,
}

trait Strategy: fmt::Debug {
    fn play (&self, id: usize, history: &Vec<Vec<Card>>) -> Card;
}

#[derive(Debug)]
struct TitForTat;
impl Strategy for TitForTat {
    fn play (&self, id: usize, history: &Vec<Vec<Card>>) -> Card {
        if history.is_empty() || history.last().unwrap()[1-id] == Card::Cooperate {
            Card::Cooperate
        } else {
            Card::Defect
        }
    }
}

#[derive(Debug)]
struct Random;
impl Strategy for Random {
    fn play (&self, id: usize, history: &Vec<Vec<Card>>) -> Card {
        if rand::random() {
            Card::Cooperate
        } else {
            Card::Defect
        }
    }
}

#[derive(Debug)]
struct AlwaysDefect;
impl Strategy for AlwaysDefect {
    fn play (&self, id: usize, history: &Vec<Vec<Card>>) -> Card {
        Card::Defect
    }
}

#[derive(Debug)]
struct AlwaysCooperate;
impl Strategy for AlwaysCooperate {
    fn play (&self, id: usize, history: &Vec<Vec<Card>>) -> Card {
        Card::Cooperate
    }
}


struct Subject {
    score: i32,
    strategy: Box<Strategy>,
}

impl fmt::Debug for Subject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}: {})", self.strategy, self.score)
    }
}

#[derive(Debug)]
struct Generation {
    subjects: Vec<Subject>,
}
impl Generation {
    fn play(&mut self) {
        let n = self.subjects.len();
        for i in 0..n {
            println!("testing {}", i);
            for j in 0..n {
                let mut h = vec![];
                for _ in 0..100 {
                    let mut res = vec![];
                    res.push(self.subjects[i].strategy.play(0, &h));
                    res.push(self.subjects[j].strategy.play(1, &h));

                    if res[0] == Card::Cooperate && res[1] == Card::Cooperate {
                        self.subjects[i].score+= 3;
                    }
                    if res[0] == Card::Cooperate && res[1] == Card::Defect {
                        self.subjects[i].score+= 0;
                    }
                    if res[0] == Card::Defect && res[1] == Card::Defect {
                        self.subjects[i].score+= 1;
                    }
                    if res[0] == Card::Defect && res[1] == Card::Cooperate {
                        self.subjects[i].score+= 5;
                    }

                    h.push(res);
                }
            }
        }
    }
}

fn main() {
    let mut gen = Generation {
        subjects: vec![
            Subject {
                score: 0,
                strategy: Box::new(TitForTat {}),
            },
            Subject {
                score: 0,
                strategy: Box::new(Random {}),
            },
            Subject {
                score: 0,
                strategy: Box::new(AlwaysDefect {}),
            },
            // Subject {
            //     score: 0,
            //     strategy: Box::new(AlwaysCooperate {}),
            // },
        ],
    };

    gen.play();
    gen.subjects.sort_by(|a, b| b.score.cmp(&a.score));
    println!("Hello, world!\n{:#?}", gen.subjects);
}
