use crate::day;
use std::collections::{VecDeque, HashSet};

type Card = u32;
type Score = u128;

pub struct Day22 {

}

impl day::Day for Day22 {

    fn puzzle1(&self) {
        println!("Day 22, puzzle 1");

        let result = get_result_1();

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 22, puzzle 2");

        let result = get_result_2();

        println!("{}", result);
    }

}

fn get_result_1() -> Score {
    let c1 = vec![6,25,8,24,30,46,42,32,27,48,5,2,14,28,37,17,9,22,40,33,3,50,47,19,41];
    let c2 = vec![1,18,31,39,16,10,35,29,26,44,21,7,45,4,20,38,15,11,34,36,49,13,23,43,12];

    play_game_1(&c1, &c2)
}

fn get_result_2() -> Score {
    let c1 = vec![6,25,8,24,30,46,42,32,27,48,5,2,14,28,37,17,9,22,40,33,3,50,47,19,41];
    let c2 = vec![1,18,31,39,16,10,35,29,26,44,21,7,45,4,20,38,15,11,34,36,49,13,23,43,12];

    play_game_2(&c1, &c2)
}

struct Player {
    // The top of the deck is at the front.
    cards: VecDeque<Card>
}

impl Player {
    /// Constructs a new player, the top of the deck is the first card in the list
    fn new(cards: &[Card]) -> Player {
        Player { cards: cards.iter().map(|&c| c).collect::<VecDeque<Card>>() }
    }

    fn count_score(&self) -> Score {
        let mut score: Score = 0;
        let mut mult = self.cards.len() as Score;
        for card in self.cards.iter() {
            score += *card as Score * mult;
            mult -= 1;
        }
        score
    }

    fn deck_fingerprint(&self) -> String {
        self.cards.iter().map(|&c| format!("{},", c)).collect::<String>()
    }

    fn is_looser(&self) -> bool {
        self.cards.is_empty()
    }
}

fn play_game_1(cards_player1: &[Card], cards_player2: &[Card]) -> Score {
    let mut player1 = Player::new(cards_player1);
    let mut player2 = Player::new(cards_player2);

    while play_round_1(&mut player1, &mut player2) { }

    if player1.is_looser() {
        player2.count_score()
    } else {
        player1.count_score()
    }
}

/// Plays a round, return true if continue, false if ended
fn play_round_1(p1: &mut Player, p2: &mut Player) -> bool {

    let c1 = p1.cards.pop_front().unwrap();
    let c2 = p2.cards.pop_front().unwrap();
    if c1 > c2 {
        p1.cards.push_back(c1);
        p1.cards.push_back(c2);
        !p2.is_looser()
    } else {
        p2.cards.push_back(c2);
        p2.cards.push_back(c1);
        !p1.is_looser()
    }
}

fn play_game_2(cards_player1: &[Card], cards_player2: &[Card]) -> Score {
    let (p1, p2, w1) = play_sub_game_2(cards_player1, cards_player2);
    if w1 {
        p1.count_score()
    } else {
        p2.count_score()
    }
}

fn play_sub_game_2(cards_player1: &[Card], cards_player2: &[Card]) -> (Player, Player, bool) {
    let mut player1 = Player::new(cards_player1);
    let mut player2 = Player::new(cards_player2);

    let mut cardset1 = HashSet::new();
    let mut cardset2 = HashSet::new();

    let mut play = true;
    while play {
        if !cardset1.insert(player1.deck_fingerprint()) {
            return (player1, player2, true);
        }
        if !cardset2.insert(player2.deck_fingerprint()) {
            return (player1, player2, true);
        }
        play = play_round_2(&mut player1, &mut player2);
    }

    let w1 = player2.is_looser();
    (player1, player2, w1)
}

/// Plays a round, return true if continue, false if ended
fn play_round_2(p1: &mut Player, p2: &mut Player) -> bool {
    let c1 = p1.cards.pop_front().unwrap();
    let c2 = p2.cards.pop_front().unwrap();
    if c1 <= p1.cards.len() as Card && c2 <= p2.cards.len() as Card {
        let (_sp1, _sp2, w1) = play_sub_game_2(
            &p1.cards.iter().take(c1 as usize).map(|&c| c).collect::<Vec<Card>>(),
            &p2.cards.iter().take(c2 as usize).map(|&c| c).collect::<Vec<Card>>()
        );
        if w1 {
            p1.cards.push_back(c1);
            p1.cards.push_back(c2);
            !p2.is_looser()
        } else {
            p2.cards.push_back(c2);
            p2.cards.push_back(c1);
            !p1.is_looser()
        }
    } else if c1 > c2 {
        p1.cards.push_back(c1);
        p1.cards.push_back(c2);
        !p2.is_looser()
    } else {
        p2.cards.push_back(c2);
        p2.cards.push_back(c1);
        !p1.is_looser()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_game_1() {
        assert_eq!(play_game_1(&vec![9, 2, 6, 3, 1], &vec![5, 8, 4, 7, 10]), 306);
    }

    #[test]
    fn test_play_game_2() {
        assert_eq!(play_game_2(&vec![9, 2, 6, 3, 1], &vec![5, 8, 4, 7, 10]), 291);
    }

}