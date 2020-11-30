use std::collections::HashMap;
fn main() {
    println!("{}", high_score(459, 72103));
}

struct Game {
    circle: Vec<usize>,
    curr_marble: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            circle: vec![0],
            curr_marble: 0,
        }
    }

    fn circ_ind(&self, idx: i32) -> usize {
        let idx = idx % self.circle.len() as i32;
        if idx < 0 {
            (self.circle.len() as i32 + idx) as usize
        } else {
            idx as usize
        }
    }

    fn remove(&mut self) -> usize {
        self.curr_marble = self.circ_ind(self.curr_marble as i32 - 7);
        self.circle.remove(self.curr_marble)
    }

    fn insert(&mut self, marble: usize) {
        self.curr_marble = self.circ_ind(self.curr_marble as i32 + 1) + 1;
        self.circle.insert(self.curr_marble, marble)
    }
}

fn high_score(num_players: usize, num_marbles: usize) -> usize {
    let players = 1..(num_players + 1);
    let score: HashMap<usize, usize> = players.clone().map(|p| (p, 0)).collect();
    let (_, score) = (1..num_marbles + 1).zip(players.cycle()).fold(
        (Game::new(), score),
        |(mut state, mut score), (m, p)| {
            if m % 23 == 0 {
                //println!("{}, {}", m, p);
                let removed = state.remove();
                *(score.get_mut(&p).unwrap()) += m + removed;
            } else {
                state.insert(m)
            }
            // println!("[{}] {:?}", p, state.circle);
            (state, score)
        },
    );
    score.iter().map(|(_, s)| *s).max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(high_score(9, 25), 32);
        assert_eq!(high_score(10, 1618), 8317);
        assert_eq!(high_score(13, 7999), 146373);
        assert_eq!(high_score(17, 1104), 2764);
        assert_eq!(high_score(21, 6111), 54718);
        assert_eq!(high_score(30, 5807), 37305);
    }
}
