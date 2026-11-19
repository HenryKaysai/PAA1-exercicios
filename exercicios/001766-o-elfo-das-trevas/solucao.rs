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
    for k in 1..=t {
        //second line(number of raindeers)
        let deers: usize = next(&mut tokens);

        //third line(number of pulling raindeers)
        let pulling_deers: usize = next(&mut tokens);

        //creates an empty vector with struct type
        let mut deer_vec: Vec<LasRenasLocas> = Vec::new();

        //fills the vector with raindeers and carachteristics
        for _ in 0..deers {
            deer_vec.push(stash_deers(&mut tokens));
        }

        ordering_deers(&mut deer_vec);

        println!("CENARIO {{{}}}", k);


        //prints the output
        for (i, deer) in deer_vec.iter().take(pulling_deers).enumerate() {
            println!("{} - {}", i + 1, deer.name);
        }
    }
}

struct LasRenasLocas {
    name: String,
    weight: i32,
    age: i32,
    height: f64,
}

fn ordering_deers(vec: &mut Vec<LasRenasLocas>) {
    vec.sort_by(|a, b| {
        b.weight
            .cmp(&a.weight)
            .then_with(|| a.age.cmp(&b.age))
            .then_with(|| a.height.partial_cmp(&b.height).unwrap())
            .then_with(|| a.name.cmp(&b.name))
    });
}

//this function fills the struct LasRenasLocas when called
fn stash_deers(deer: &mut std::str::SplitWhitespace) -> LasRenasLocas{
    LasRenasLocas{
        name: next(deer),
        weight: next(deer),
        age: next(deer),
        height: next(deer),
    }
}

//this function transforms beecrowd inputs into a Some typo
fn next<T: FromStr>(tokens: &mut std::str::SplitWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

