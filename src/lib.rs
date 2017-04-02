use std::collections::HashSet;

/// Calculates the collatz conjecture for a given number `starting_number`.
/// Returns a `Vec<u32>` of the number trail.
///
/// # Examples
/// ```
/// let ls: Vec<u32> = collatzconjecture::number(10000);
/// ```
pub fn number_trail(starting_number: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    let mut temp = starting_number;
    v.push(temp);

    while temp != 1 {
        temp = collatizer(temp);
        v.push(temp);
    }

    v
}

/// Calculates the collatz conjecture for each number up to `final_number`.
/// Returns a `HashSet<u32>` without duplicates of trail hits.
///
/// # Examples
///
/// ```
/// let hset: HashSet<u32> = collatzconjecture::up_to(1000);
/// ```
pub fn up_to(final_number: u32) -> HashSet<u32> {
    let mut hset: HashSet<u32> = HashSet::new();

    for i in 1..final_number {
        let mut temp = i;

        if hset.insert(temp) == false {
            continue;
        }

        while temp != 1 {
            temp = collatizer(temp);
            hset.insert(temp);
        }
    }

    hset
}

fn collatizer(number: u32) -> u32 {
    if number % 2 == 0 {
        number / 2
    } else {
        number * 3 + 1
    }
}
