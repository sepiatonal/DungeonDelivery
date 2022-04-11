//Random utility & placeholder functions.
use rand::prelude::*;

pub fn exploding_die(sides: u16) -> i16 { //placeholder die rolling function. initializes a new rng thread every time it is called. Replace if this becomes a bottleneck.
    let mut rng = thread_rng();
    let mut result = rng.gen_range(0..sides);
    let mut total = result + 1;

    while result == sides - 1 || total <= 1000 {
        result = rng.gen_range(0..sides);
        total += result;
}

    return total.try_into().unwrap();
}
