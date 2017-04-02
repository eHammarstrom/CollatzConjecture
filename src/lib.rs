extern crate rayon;
extern crate bit_vec;

use bit_vec::BitVec;

use rayon::prelude::*;

/// Calculates the collatz conjecture for a given number `starting_number`.
/// Returns a `LinkedList<u32>` of the number trail.
///
/// # Examples
/// ```
/// let ls: LinkedList<u32> = collatzconjecture::number_trail(10000);
/// ```
pub fn number_trail(starting_number: u32) -> BitVec<u32> {
    let mut temp = starting_number;

    let mut bv = BitVec::from_elem(temp as usize, true);

    while temp != 1 {
        temp = collatizer(temp);

        let len = bv.len();
        if len <= (temp as usize) {
            bv.grow(temp as usize + 1 - len, false);
        }

        bv.set(temp as usize, true);
    }

    bv
}

/// Calculates the collatz conjecture for each number up to `final_number`.
/// Returns a `Vec<u32>` without duplicates of trail hits.
///
/// # Examples
///
/// ```
/// let set: Vec<u32> = collatzconjecture::up_to(1000);
/// ```
pub fn up_to(final_number: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    let bv: BitVec<u32> = bit_vec_up_to(final_number);

    for (i, x) in bv.iter().enumerate() {
        if x == true {
            v.push(i as u32 + 1);
        }
    }

    v
}

fn bit_vec_up_to(final_number: u32) -> BitVec<u32> {
    let bvs: Vec<BitVec<u32>> = (1..final_number)
        .into_par_iter()
        .map(|x| number_trail(x))
        .collect();

    let mut final_bv = bvs[0].clone();

    for x in bvs {
        final_bv.union(&x);
    }

    final_bv
}

fn collatizer(number: u32) -> u32 {
    if number % 2 == 0 {
        number / 2
    } else {
        number * 3 + 1
    }
}