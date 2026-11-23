// Beecrowd 1931 - Mania de Par
// https://judge.beecrowd.com/pt/problems/view/1931

use std::io::{self, Read};
use std::str::FromStr;

fn main() {

    //initializes the beecrowd input
    let mut input = String::new();

    //reads the values into a string
    io::stdin().read_to_string(&mut input).unwrap();

    //separate every word with white spaces
    let mut tokens = input.split_whitespace();

    //number of cities
    let c: usize = next(&mut tokens);

    //number of roads
    let r: usize = next(&mut tokens);

    for _ in 0..c {
        for p in 0..=3 {
            
        }
    }


}

//this function transforms beecrowd inputs into a Some typo
fn next<T: FromStr>(tokens: &mut std::str::SplitWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

