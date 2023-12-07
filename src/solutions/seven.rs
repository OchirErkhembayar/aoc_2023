use std::collections::{BinaryHeap, HashMap};

pub fn part_one(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 7 part 1: {}", elapsed.as_micros());
    ans
}
pub fn part_one_inner(data: &str) -> i32 {
    let mut round = Vec::new();
    for line in data.lines() {
        println!("Line: {line}");
        let (cards, bid) = line.split_once(' ').unwrap();
        let bid: i32 = bid.parse().unwrap();
        let hand = Hand::new(cards.chars().collect::<Vec<_>>(), bid);
        if round.is_empty() {
            round.push(hand)
        } else {
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
    }
    let mut rank = 0;
    let mut sum = 0;
    round.reverse();
    for h in round.iter() {
        rank += 1;
        sum += h.bid * rank;
    }
    println!(
        "Round: {:?}",
        round.iter().map(|h| h.strength).collect::<Vec<_>>()
    );
    println!("Rank: {rank}");
    sum
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
        println!("Hands {:?}", hand_map);
        let highest = hand_map
            .iter()
            .reduce(|(hcard, hcount), (ccard, ccount)| {
                if ccount > hcount {
                    (ccard, ccount)
                } else {
                    (hcard, hcount)
                }
            })
            .unwrap()
            .1
            .clone();
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
                    if mine.is_digit(10) {
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
        Some(self.cmp(&other))
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
pub fn part_two_inner(data: &str) -> i32 {
    42
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
        assert!(fst.strength == 5);
        let fst = Hand::new(Vec::from(['1', 'Q', 'Q', 'Q', 'Q']), 10);
        assert!(fst.strength == 4);
        let fst = Hand::new(Vec::from(['1', '2', 'Q', 'Q', 'Q']), 10);
        assert!(fst.strength == 3);
        let fst = Hand::new(Vec::from(['1', '2', '3', 'Q', 'Q']), 10);
        assert!(fst.strength == 2);
        let fst = Hand::new(Vec::from(['1', '2', '3', '4', 'Q']), 10);
        assert!(fst.strength == 1);
    }
}
