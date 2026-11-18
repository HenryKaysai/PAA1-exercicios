// Beecrowd 1766 - O Elfo das Trevas
// https://judge.beecrowd.com/pt/problems/view/1766

use std::io::{self, Read};
use std::str::FromStr;



fn main() {
    //initializes the beecrowd input
    let mut input = String::new();
    //reads the values into a string
    io::stdin().read_to_string(&mut input).unwrap();
    //separate every word with white spaces
    let mut tokens = input.split_whitespace();
    //number o cases
    let t: usize = next(&mut tokens);

    //while test cases exist
    for _ in 0..t {
        //second line(number of raindeers)
        let deers: usize = next(&mut tokens);
        //third line(number of pulling raindeers)
        let pulling_deers: usize = next(&mut tokens);

        let mut deer_vec: Vec<LasRenasLocas> = Vec::new();

        for _ in 0..deers {
            deer_vec.push(stash_deers(&mut tokens));
        }
        println!("{:#?}", deer_vec);
    }


}

#[derive(Debug)]
struct LasRenasLocas {
    name: String,
    weight: i32,
    age: i32,
    height: f64,
}

fn stash_deers(deer: &mut std::str::SplitWhitespace) -> LasRenasLocas{
    LasRenasLocas{
        name: next(deer),
        weight: next(deer),
        age: next(deer),
        height: next(deer),
    }
}

fn next<T: FromStr>(tokens: &mut std::str::SplitWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

