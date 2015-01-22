/// `extern` is necessary to specify an explicit dependency on another crate.
/// In this case, the crate happens to be a locally-available one.
///
/// One way to think of it would be a manner of specifying in-code what other
/// jar/artifacts should be used for further namespace/module resolution.
extern crate rust_for_java_devs;

/// Import the public structure that is the entry point for the library
use rust_for_java_devs::reasonable_implementation::{Market, Supermarket};
use std::rand;
use std::rand::Rng;
use std::collections::hash_map::{HashMap, Entry};
use std::num::ToPrimitive;

/// `static` variables are essentially static constants available to
/// everything in the module.
static NUM_TEST_ITERATIONS:i32 = 1000;
static MAX_ITEMS_STRING_SIZE:usize = 1000;

/// The #[test] header here is an annotation to the function,
/// called an attribute, which specifies that the following
/// should be used in the test suite.
#[test]
fn canonical_input() {
    let s = Supermarket::new(); 
    let items = "ABBACBBAB".to_string();
    // The presence of `i32` after the number clarifies which size of int
    // we are using here.
    assert_eq!(240i32, s.checkout(items))
}

#[test]
fn empty_input() {
    let s = Supermarket::new();
    let items = "".to_string();
    assert_eq!(0i32, s.checkout(items))
}

#[test]
fn ignores_unrelated_items() {
    let s = Supermarket::new();
    let items = "XKD".to_string();
    assert_eq!(0i32, s.checkout(items))
}

#[test]
fn mixes_standard_and_unregistered_items_prices() {
    let s = Supermarket::new();
    let items = "AXBC".to_string();
    assert_eq!(100i32, s.checkout(items))
}

#[test]
fn single_bundle_gets_combo_price() {
    let s = Supermarket::new();
    let items = "BBBBB".to_string();
    assert_eq!(150i32, s.checkout(items))
}

#[test]
fn single_bundle_with_leftovers_gives_deal_price_plus_individual() {
    let s = Supermarket::new();
    let items = "BBBBB B".to_string();
    assert_eq!(200i32, s.checkout(items))
}

#[test]
fn multiple_bundles_each_get_deal_price() {
    let s = Supermarket::new();
    let items = "BBBBB BBBBB".to_string();
    assert_eq!(300i32, s.checkout(items))
}

#[test]
fn multiple_bundles_each_get_deal_price_plus_leftovers() {
    let s = Supermarket::new();
    let items = "BBBBB BBBBB BB".to_string();
    assert_eq!(400i32, s.checkout(items))
}

fn generate_char_sequence(c:char) -> (String, i32) {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1, MAX_ITEMS_STRING_SIZE);
    let mut seq = "".to_string();
    // The underscore prefix is a hint that we do not intend to actually use
    // the value stored in this variable.
    for _i in range(0, n) {
        seq.push(c);
    }
    (seq, n.to_i32().unwrap())
}

#[test]
fn correctly_sums_sequences_of_many_sizes_of_as() {
    let s = Supermarket::new();
    for _i in range(1, NUM_TEST_ITERATIONS) {
        let (items, len) = generate_char_sequence('A');
        assert_eq!(len * 20, s.checkout(items)); 
    }
}

#[test]
fn correctly_sums_sequences_of_many_sizes_of_bs() {
    let s = Supermarket::new();
    for _i in range(1, NUM_TEST_ITERATIONS) {
        let (items, len) = generate_char_sequence('B');
        assert_eq!(((len / 5) * 150) + ((len % 5) * 50), s.checkout(items)); 
    }
}

#[test]
fn correctly_sums_sequences_of_many_sizes_of_cs() {
    let s = Supermarket::new();
    for _i in range(1, NUM_TEST_ITERATIONS) {
        let (items, len) = generate_char_sequence('C');
        assert_eq!(len * 30, s.checkout(items)); 
    }
}


fn generate_mixed_char_sequence(chars:&[char]) -> (String, HashMap<char, i32>) {
    let mut rng = rand::thread_rng();
    let n:usize = rng.gen_range(0, MAX_ITEMS_STRING_SIZE);
    let mut count: HashMap<char, i32> = HashMap::new();
    let mut s = "".to_string();
    for _i in range(0, n) {
        match rng.choose(chars) {
            Some(&c) => {
                s.push(c);
                match count.entry(c) {
                    Entry::Vacant(slot) => {slot.insert(1); },
                    Entry::Occupied(mut slot) => {*slot.get_mut() += 1;}
                }
            }
            None => ()
        }
    }
    (s, count)
}

/// Rigid, inflexible, and basically correct scoring of initial
/// products based on simple item counts.
fn simple_expected_price(counts: HashMap<char, i32>) -> i32 {
    let a_cost = match counts.get(&'A') {
        None => 0,
        Some(&count) => count*20
    };
    
    let b_cost = match counts.get(&'B') {
        None => 0,
        Some(&count) => {
           ((count / 5) * 150) + ((count % 5) * 50)
        }
    };

    let c_cost = match counts.get(&'C') {
        None => 0,
        Some(&count) => count*30
    };
    
    a_cost + b_cost + c_cost
}

#[test]
fn correctly_sums_random_sequence_of_valid_codes() {
    let standard_codes = ['A', 'B', 'C'];
    let s = Supermarket::new();
    for _i in range(0, NUM_TEST_ITERATIONS) {
        let (items, counts) = generate_mixed_char_sequence(&standard_codes);
        assert_eq!(simple_expected_price(counts), s.checkout(items));
    }
}

