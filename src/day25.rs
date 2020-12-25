use std::collections::HashMap;

static GENERATOR: u128 = 7;
static MODULO: u128 = 20201227;

// https://en.wikipedia.org/wiki/Baby-step_giant-step
fn fast_dlp(base: u128, modulo: u128, target: u128) -> u128 {
    let (alpha, n, beta) = (base, modulo, target);
    // 1. m ← Ceiling(√n)
    let m = (n as f64).sqrt().ceil() as u128;

    // For all j where 0 ≤ j < m:
    //     Compute αj and store the pair (j, αj) in a table.
    let mut table = HashMap::new();
    {
        let mut alpha = 1;
        for j in 0..m {
            table.insert(alpha, j);
            alpha = (alpha * base) % n;
        }
    }

    // Compute α−m
    let a_neg_m = mod_exp(alpha, n - m - 1, n);

    // γ ← β. (set γ = β)
    let mut gamma = beta;

    // For all i where 0 ≤ i < m:
    for i in 0..m {
        // Check to see if γ is the second component (αj) of any pair in the table.
        if let Some(j) = table.get(&gamma) {
            // If so, return im + j.
            return (i * m + j) % n;
        }
        // If not, γ ← γ • α−m.
        gamma = (gamma * a_neg_m) % n;
    }

    panic!("No DLP found");
}

fn brute_force_dlp(base: u128, modulo: u128, target: u128) -> u128 {
    let mut result = 0;
    let mut power = 1;

    while power != target {
        power = (power * base) % modulo;
        result += 1;
    }
    result
}

fn mod_exp(base: u128, exponent: u128, modulo: u128) -> u128 {
    if exponent == 0 {
        1
    } else if exponent == 1 {
        base
    } else {
        let root = mod_exp(base, exponent >> 1, modulo) as u128;
        if exponent % 2 == 1 {
            base * root * root % modulo
        } else {
            root * root % modulo
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_smoke1() {
        let card_public = 5764801;
        let door_public = 17807724;

        let card_private = brute_force_dlp(GENERATOR, MODULO, card_public);
        assert_eq!(8, card_private);
        let door_private = brute_force_dlp(GENERATOR, MODULO, door_public);
        assert_eq!(11, door_private);

        let shared_secret = mod_exp(door_public, card_private, MODULO);
        assert_eq!(14897079, shared_secret);

        let shared_secret = mod_exp(card_public, door_private, MODULO);
        assert_eq!(14897079, shared_secret);
    }

    #[test]
    fn day25_1() {
        let card_public = 10212254;
        let door_public = 12577395;

        let card_private = brute_force_dlp(GENERATOR, MODULO, card_public);
        println!("Card Private: {}", card_private);
        // Next two aren't necessary, I just was curious
        // let door_private = brute_force_dlp(GENERATOR, MODULO, door_public);
        // println!("Door Private: {}", door_private);
        let shared_secret = mod_exp(door_public, card_private, MODULO);
        assert_eq!(290487, shared_secret);
    }

    #[test]
    fn day25_fast_smoke1() {
        let card_public = 5764801;
        let door_public = 17807724;

        let card_private = fast_dlp(GENERATOR, MODULO, card_public);
        assert_eq!(8, card_private);
        let door_private = fast_dlp(GENERATOR, MODULO, door_public);
        assert_eq!(11, door_private);

        let shared_secret = mod_exp(door_public, card_private, MODULO);
        assert_eq!(14897079, shared_secret);

        let shared_secret = mod_exp(card_public, door_private, MODULO);
        assert_eq!(14897079, shared_secret);
    }

    #[test]
    fn day25_fast_1() {
        let card_public = 10212254;
        let door_public = 12577395;

        let card_private = fast_dlp(GENERATOR, MODULO, card_public);
        println!("Card Private: {}", card_private);
        // Next two aren't necessary, I just was curious
        // let door_private = fast_dlp(GENERATOR, MODULO, door_public);
        // println!("Door Private: {}", door_private);
        let shared_secret = mod_exp(door_public, card_private, MODULO);
        assert_eq!(290487, shared_secret);
    }
}
