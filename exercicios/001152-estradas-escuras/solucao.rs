// Beecrowd 1152 - Estradas Escuras
// https://judge.beecrowd.com/pt/problems/view/1152

use std::io::{self, Read};
use std::str::FromStr;

fn next<T: FromStr>(tokens: &mut std::str::SplitWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn achar(pai: &Vec<usize>, x: usize) -> usize {
    let mut atual = x;
    while pai[atual] != atual {
        atual = pai[atual];
    }
    atual
}

fn main() {
    let mut entrada = String::new();
    io::stdin().read_to_string(&mut entrada).unwrap();
    let mut tokens = entrada.split_whitespace();

    loop {
        let m: usize = next(&mut tokens);
        let n: usize = next(&mut tokens);
        if m == 0 && n == 0 {
            break;
        }

        let mut estradas: Vec<(u64, usize, usize)> = Vec::new();
        let mut total: u64 = 0;
        for _ in 0..n {
            let x: usize = next(&mut tokens);
            let y: usize = next(&mut tokens);
            let z: u64 = next(&mut tokens);
            total = total + z;
            estradas.push((z, x, y));
        }

        estradas.sort();

        let mut pai: Vec<usize> = Vec::new();
        for i in 0..m {
            pai.push(i);
        }

        let mut gasto: u64 = 0;
        for i in 0..estradas.len() {
            let (z, x, y) = estradas[i];
            let chefe_x = achar(&pai, x);
            let chefe_y = achar(&pai, y);
            if chefe_x != chefe_y {
                pai[chefe_x] = chefe_y;
                gasto = gasto + z;
            }
        }

        println!("{}", total - gasto);
    }
}
