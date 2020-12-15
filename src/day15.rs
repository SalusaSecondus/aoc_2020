use std::collections::HashMap;

fn start(nums: &[u32]) -> HashMap<u32, usize> {
    let mut result = HashMap::new();
    for (idx, n) in nums.iter().enumerate() {
        result.insert(*n, idx + 1);
    }
    result
}

fn next_number(history: &mut HashMap<u32, usize>, turn: usize, prev_number: u32) -> u32 {
    if let Some(prev_turn) = history.get(&prev_number) {
        let result: u32 = (turn - prev_turn) as u32 - 1;
        history.insert(prev_number, turn - 1);
        result
    } else {
        history.insert(prev_number, turn - 1);
        0
    }
}

fn get_value_on_turn(seed: &[u32], target_turn: usize) -> u32 {
    let mut state = start(&seed[..seed.len() - 1]);
    let start = seed.len() + 1;
    let mut prev = *seed.last().unwrap();
    for turn in start..target_turn + 1 {
        if turn % 10000 == 0 {
            println!("Turn {}", turn);
        }
        prev = next_number(&mut state, turn, prev);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_smoke1() {
        assert_eq!(436, get_value_on_turn(&[0, 3, 6], 2020));
        assert_eq!(1, get_value_on_turn(&[1, 3, 2], 2020));
        assert_eq!(10, get_value_on_turn(&[2, 1, 3], 2020));
    }

    #[test]
    fn day15_1() {
        println!(
            "Day 15.1: {}",
            get_value_on_turn(&[11, 0, 1, 10, 5, 19], 2020)
        );
        println!(
            "Day 15.2: {}",
            get_value_on_turn(&[11, 0, 1, 10, 5, 19], 30000000)
        );
    }
}
