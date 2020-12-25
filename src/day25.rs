static GENERATOR: u128 = 7;
static MODULO: u128 = 20201227;

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
        let door_private = brute_force_dlp(GENERATOR, MODULO, door_public);
        println!("Door Private: {}", door_private);
        let shared_secret = mod_exp(door_public, card_private, MODULO);
        assert_eq!(290487, shared_secret);
    }
}
