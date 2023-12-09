use std::collections::HashMap;

pub fn part_one(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 7 part 1: {}", elapsed.as_micros());
    ans
}

#[derive(Eq, Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    strength: i32,
    bid: i32,
}

impl Hand {
    fn new(chars: Vec<char>, bid: i32) -> Self {
        let mut hand_map = HashMap::new();
        for card in chars.iter() {
            let entry = hand_map.entry(card).or_insert(0);
            *entry += 1;
        }
        let mut highest = *hand_map
            .iter()
            .reduce(|(hcard, hcount), (ccard, ccount)| {
                if ccount > hcount {
                    (ccard, ccount)
                } else {
                    (hcard, hcount)
                }
            })
            .unwrap()
            .1;
        if highest > 3 {
            highest += 2;
        } else if highest == 3 {
            if hand_map.iter().any(|(_, v)| v == &2) {
                highest += 2;
            } else {
                highest += 1;
            }
        } else if highest == 2 && hand_map.iter().filter(|(_, v)| v == &&2).count() == 2 {
            highest += 1;
        }
        Self {
            cards: chars,
            strength: highest,
            bid,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn lt(&self, other: &Self) -> bool {
        !self.gt(other)
    }
    fn gt(&self, other: &Self) -> bool {
        if self.strength == other.strength {
            for (mine, theirs) in self.cards.iter().zip(&other.cards) {
                if mine != theirs {
                    if mine.is_ascii_digit() {
                        return *mine as i32 > *theirs as i32;
                    } else {
                        match mine {
                            'A' => {
                                if theirs == &'A' {
                                    continue;
                                } else {
                                    return true;
                                }
                            }
                            'K' => {
                                if theirs == &'A' {
                                    return false;
                                } else if theirs == &'K' {
                                    continue;
                                } else {
                                    return true;
                                }
                            }
                            'Q' => {
                                if ['A', 'K'].contains(theirs) {
                                    return false;
                                } else if theirs == &'Q' {
                                    continue;
                                } else {
                                    return true;
                                }
                            }
                            'J' => {
                                if ['A', 'K', 'Q'].contains(theirs) {
                                    return false;
                                } else if theirs == &'J' {
                                    continue;
                                } else {
                                    return true;
                                }
                            }
                            'T' => {
                                if ['A', 'K', 'Q', 'J'].contains(theirs) {
                                    return false;
                                } else if theirs == &'T' {
                                    continue;
                                } else {
                                    return true;
                                }
                            }
                            _ => panic!("WTF!"),
                        }
                    }
                }
            }
            panic!("Same strength hands");
        }
        self.strength > other.strength
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strength.cmp(&other.strength)
    }
}

pub fn part_two(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 7 part 1: {}", elapsed.as_micros());
    ans
}

pub fn part_one_inner(data: &str) -> i32 {
    let mut round = Vec::new();
    for line in data.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();
        let bid: i32 = bid.parse().unwrap();
        let hand = Hand::new(cards.chars().collect::<Vec<_>>(), bid);
        let mut found = false;
        for (i, h) in round.clone().iter().enumerate() {
            if &hand > h {
                round.insert(i, hand.clone());
                found = true;
                break;
            }
        }
        if !found {
            round.push(hand);
        }
    }
    let mut rank = 0;
    let mut sum = 0;
    round.reverse();
    for h in round.iter() {
        rank += 1;
        sum += h.bid * rank;
    }
    sum
}

pub fn part_two_inner(data: &str) -> i32 {
    let mut hands = Vec::new();
    for line in data.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|c| match c {
                'A' => A,
                'K' => K,
                'Q' => Q,
                'T' => T,
                '9' => NINE,
                '8' => EIGHT,
                '7' => SEVEN,
                '6' => SIX,
                '5' => FIVE,
                '4' => FOUR,
                '3' => THREE,
                '2' => TWO,
                'J' => J,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let bid: i32 = bid.parse().unwrap();
        let cards = Cards::new(cards, bid);
        if hands.is_empty() {
            hands.push(cards);
        } else {
            let pos = hands.iter().position(|c| cards.stronger(c));
            if let Some(pos) = pos {
                hands.insert(pos, cards);
            } else {
                hands.push(cards);
            }
        }
    }
    hands.reverse();
    let mut sum = 0;
    let mut rank = 0;
    for hand in &hands {
        rank += 1;
        sum += rank * hand.bid;
    }
    sum
}

const J: i32 = 1;
const TWO: i32 = 2;
const THREE: i32 = 3;
const FOUR: i32 = 4;
const FIVE: i32 = 5;
const SIX: i32 = 6;
const SEVEN: i32 = 7;
const EIGHT: i32 = 8;
const NINE: i32 = 9;
const T: i32 = 10;
const Q: i32 = 11;
const K: i32 = 12;
const A: i32 = 13;

const FIVE_KIND: i32 = 7;
const FOUR_KIND: i32 = 6;
const FULL_HOUSE: i32 = 5;
const THREE_KIND: i32 = 4;
const TWO_PAIR: i32 = 3;
const ONE_PAIR: i32 = 2;
const HIGH_CARD: i32 = 1;

type Card = i32;

type Strength = i32;

#[derive(Debug)]
struct Cards {
    cards: Vec<Card>,
    bid: i32,
    score: i32,
}

impl Cards {
    fn new(cards: Vec<Card>, bid: i32) -> Self {
        let mut freqs: HashMap<Card, i32> = HashMap::new();
        cards.iter().for_each(|&c| {
            *freqs.entry(c).or_insert(0) += 1;
        });
        let mut freqs_vec = freqs
            .clone()
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>();
        freqs_vec.sort_by(|(_, v), (_, v1)| v1.cmp(v));
        for i in 0..(freqs_vec.len() - 1) {
            assert!(freqs_vec[i].1 >= freqs_vec[i + 1].1);
        }
        let score = Self::get_score(freqs_vec);
        Self { cards, bid, score }
    }

    fn stronger(&self, other: &Self) -> bool {
        let my_score = self.score;
        let their_score = other.score;
        if my_score != their_score {
            my_score > their_score
        } else {
            for (&mine, &theirs) in self.cards.iter().zip(&other.cards) {
                if mine == theirs {
                    continue;
                }
                return mine > theirs;
            }
            unreachable!();
        }
    }

    fn get_score(freqs: Vec<(Card, i32)>) -> Strength {
        match freqs.iter().position(|(c, _)| *c == J) {
            Some(pos) => {
                let jf = freqs[pos].1;
                if jf == 5 {
                    return FIVE_KIND;
                }
                if freqs[0].1 == 1 {
                    return ONE_PAIR;
                }
                if pos == 0 {
                    let snd = freqs[1].1;
                    assert!(snd <= jf);
                    assert!(freqs[0].0 == J);
                    match jf {
                        4 => FIVE_KIND,
                        3 => match jf + snd {
                            5 => FIVE_KIND,
                            4 => FOUR_KIND,
                            _ => unreachable!(),
                        },
                        2 => match snd {
                            2 => FOUR_KIND,
                            1 => THREE_KIND,
                            _ => unreachable!("Snd {} Freqs {:?} J {}", snd, freqs, jf),
                        },
                        1 => TWO_PAIR,
                        _ => unreachable!(),
                    }
                } else {
                    let fst = freqs[0].1;
                    assert!(jf <= fst);
                    match fst {
                        5 => unreachable!(),
                        4 => FIVE_KIND,
                        3 => match fst + jf {
                            5 => FIVE_KIND,
                            4 => FOUR_KIND,
                            _ => unreachable!(),
                        },
                        2 => match fst + jf {
                            4 => FOUR_KIND,
                            3 => {
                                if freqs[1].1 == 2 {
                                    assert!(jf == 1);
                                    FULL_HOUSE
                                } else {
                                    THREE_KIND
                                }
                            }
                            _ => unreachable!(),
                        },
                        1 => {
                            if jf == 0 {
                                1
                            } else {
                                assert!(jf == 1);
                                1 + jf
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            None => {
                // No jacks
                let cf1 = freqs[0].1;
                match cf1 {
                    5 => FIVE_KIND,
                    4 => FOUR_KIND,
                    3 => {
                        if freqs[1].1 == 2 {
                            FULL_HOUSE
                        } else {
                            THREE_KIND
                        }
                    }
                    2 => {
                        if freqs[1].1 == 2 {
                            TWO_PAIR
                        } else {
                            ONE_PAIR
                        }
                    }
                    1 => HIGH_CARD,
                    _ => panic!("Weird card"),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gt_hands() {
        let fst = Hand::new(Vec::from(['Q', 'Q', 'Q', 'J', 'A']), 10);
        let snd = Hand::new(Vec::from(['T', '5', '5', 'J', '5']), 10);
        assert!(fst > snd);

        let fst = Hand::new(Vec::from(['T', 'T', 'T', 'J', 'A']), 10);
        let snd = Hand::new(Vec::from(['T', 'T', 'T', 'J', 'K']), 10);
        assert!(fst > snd);

        let fst = Hand::new(Vec::from(['3', 'T', 'T', 'J', 'A']), 10);
        let snd = Hand::new(Vec::from(['4', 'T', 'T', 'J', 'K']), 10);
        assert!(fst < snd);

        let fst = Hand::new(Vec::from(['3', 'T', 'T', 'A', 'A']), 11);
        let snd = Hand::new(Vec::from(['4', 'T', 'T', 'J', 'K']), 10);
        assert!(fst > snd);
    }

    #[test]
    fn lt_hands() {
        let fst = Hand::new(Vec::from(['T', '5', '5', 'J', '5']), 10);
        let snd = Hand::new(Vec::from(['Q', 'Q', 'Q', 'J', 'A']), 10);
        assert!(fst < snd);
    }

    #[test]
    fn eq_hands() {
        let fst = Hand::new(Vec::from(['Q', 'Q', 'Q', 'J', 'A']), 10);
        let snd = Hand::new(Vec::from(['Q', 'Q', 'Q', 'J', 'A']), 10);
        assert!(fst == snd);
    }

    #[test]
    fn five_kind() {
        let fst = Hand::new(Vec::from(['Q', 'Q', 'Q', 'Q', 'Q']), 10);
        assert!(fst.strength == 7);
        let fst = Hand::new(Vec::from(['1', 'Q', 'Q', 'Q', 'Q']), 10);
        assert!(fst.strength == 6);
        let fst = Hand::new(Vec::from(['2', '2', 'Q', 'Q', 'Q']), 10);
        assert!(fst.strength == 5);
        let fst = Hand::new(Vec::from(['1', '2', 'Q', 'Q', 'Q']), 10);
        assert!(fst.strength == 4);
        let fst = Hand::new(Vec::from(['1', '3', '3', 'Q', 'Q']), 10);
        assert!(fst.strength == 3);
        let fst = Hand::new(Vec::from(['1', '2', '3', 'Q', 'Q']), 10);
        assert!(fst.strength == 2);
        let fst = Hand::new(Vec::from(['1', '2', '3', '4', 'Q']), 10);
        assert!(fst.strength == 1);
    }
}
