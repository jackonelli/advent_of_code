#![feature(test)]
use std::collections::{hash_map::DefaultHasher, HashSet, VecDeque};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;

type Deck = VecDeque<usize>;

fn main() {
    let file = "input/22/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");

    let (player_1, player_2) = parse_players(contents);
    println!("Star 1: {}", star_1(player_1.clone(), player_2.clone()));
    println!("Star 2: {}", star_2(player_1, player_2));
}

enum Winner {
    Player1(Deck),
    Player2(Deck),
}

impl Winner {
    fn score(&self) -> usize {
        let winning_deck = match self {
            Winner::Player1(deck) => deck,
            Winner::Player2(deck) => deck,
        };
        winning_deck
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, val)| (idx + 1) * val)
            .sum()
    }
}

fn play_game(mut player_1: Deck, mut player_2: Deck) -> Winner {
    let mut previous_decks_1 = HashSet::new();
    let mut previous_decks_2 = HashSet::new();
    while !player_1.is_empty() && !player_2.is_empty() {
        let hash_1 = hash(&player_1);
        let hash_2 = hash(&player_2);
        if previous_decks_1.contains(&hash_1) || previous_decks_2.contains(&hash_2) {
            return Winner::Player1(player_1);
        }
        previous_decks_1.insert(hash_1);
        previous_decks_2.insert(hash_2);

        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();

        let (player, first_reinsert, second_reinsert) =
            if player_1.len() >= card_1 && player_2.len() >= card_2 {
                match play_game(
                    player_1.clone().into_iter().take(card_1).collect(),
                    player_2.clone().into_iter().take(card_2).collect(),
                ) {
                    Winner::Player1(_) => (&mut player_1, card_1, card_2),
                    Winner::Player2(_) => (&mut player_2, card_2, card_1),
                }
            } else if card_1 > card_2 {
                (&mut player_1, card_1, card_2)
            } else {
                (&mut player_2, card_2, card_1)
            };
        player.push_back(first_reinsert);
        player.push_back(second_reinsert);
    }

    winner(player_1, player_2)
}

fn hash(d1: &VecDeque<usize>) -> u64 {
    let mut hasher = DefaultHasher::new();
    d1.hash(&mut hasher);
    hasher.finish()
}

fn star_2(player_1: Deck, player_2: Deck) -> usize {
    let winner = play_game(player_1, player_2);
    winner.score()
}

fn star_1(player_1: Deck, player_2: Deck) -> usize {
    let (mut player_1, mut player_2) = (player_1, player_2);
    while !player_1.is_empty() && !player_2.is_empty() {
        let card_1 = player_1.pop_front().unwrap();
        let card_2 = player_2.pop_front().unwrap();
        if card_1 > card_2 {
            player_1.push_back(card_1);
            player_1.push_back(card_2);
        } else {
            player_2.push_back(card_2);
            player_2.push_back(card_1);
        }
    }
    let winner = winner(player_1, player_2);
    winner.score()
}

fn winner(player_1: Deck, player_2: Deck) -> Winner {
    if player_1.is_empty() {
        Winner::Player2(player_2)
    } else {
        Winner::Player1(player_1)
    }
}

fn parse_players(contents: String) -> (Deck, Deck) {
    let mut data = contents.trim().split("\n\n");
    let player_1: Deck = data
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|d| d.parse::<usize>().unwrap())
        .collect();
    let player_2: Deck = data
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|d| d.parse::<usize>().unwrap())
        .collect();
    (player_1, player_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    extern crate test;
    #[bench]
    fn day_22_star_1(b: &mut Bencher) {
        let file = "input/22/input";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");

        let (player_1, player_2) = parse_players(contents);
        b.iter(|| star_1(player_1.clone(), player_2.clone()))
    }

    #[bench]
    fn day_22_star_2(b: &mut Bencher) {
        let file = "input/22/input";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");

        let (player_1, player_2) = parse_players(contents);
        b.iter(|| star_2(player_1.clone(), player_2.clone()))
    }
}
