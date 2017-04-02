extern crate rayon;

use std::collections::HashSet;
use std::collections::LinkedList;
use std::thread;
use std::sync::mpsc;

use rayon::prelude::*;

/// Calculates the collatz conjecture for a given number `starting_number`.
/// Returns a `Vec<u32>` of the number trail.
///
/// # Examples
/// ```
/// let ls: Vec<u32> = collatzconjecture::number(10000);
/// ```
pub fn number(starting_number: u32) -> LinkedList<u32> {
    let mut number_trail: LinkedList<u32> = LinkedList::new();

    let mut temp = starting_number;
    number_trail.push_front(temp);

    while temp != 1 {
        temp = collatizer(temp);
        number_trail.push_front(temp);
    }

    number_trail
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
    let mut ans: Vec<LinkedList<u32>> = Vec::new();

    (1..final_number)
        .into_par_iter()
        .map(|x| number(x)) // fn number(u32) -> LinkedList<u32>
        .collect_into(&mut ans);

    for ls in ans {
        for i in ls {
            hset.insert(i);
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