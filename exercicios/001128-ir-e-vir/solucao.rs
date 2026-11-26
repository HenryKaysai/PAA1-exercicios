// Beecrowd 1128 - Ir e Vir
// https://judge.beecrowd.com/pt/problems/view/1128

use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

fn next<T: FromStr>(tokens: &mut std::str::SplitWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn chega_em_todos(vizinhos: &Vec<Vec<usize>>, origem: usize) -> bool {
    let mut visitado: Vec<bool> = vec![false; vizinhos.len()];
    let mut fila: VecDeque<usize> = VecDeque::new();
    visitado[origem] = true;
    fila.push_back(origem);
    while let Some(u) = fila.pop_front() {
        for &v in &vizinhos[u] {
            if !visitado[v] {
                visitado[v] = true;
                fila.push_back(v);
            }
        }
    }
    for i in 1..vizinhos.len() {
        if !visitado[i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut entrada = String::new();
    io::stdin().read_to_string(&mut entrada).unwrap();
    let mut tokens = entrada.split_whitespace();

    loop {
        let n: usize = next(&mut tokens);
        let m: usize = next(&mut tokens);
        if n == 0 && m == 0 {
            break;
        }

        let mut ida: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let mut volta: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

        for _ in 0..m {
            let v: usize = next(&mut tokens);
            let w: usize = next(&mut tokens);
            let p: usize = next(&mut tokens);
            ida[v].push(w);
            volta[w].push(v);
            if p == 2 {
                ida[w].push(v);
                volta[v].push(w);
            }
        }

        if chega_em_todos(&ida, 1) && chega_em_todos(&volta, 1) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
