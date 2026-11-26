// Beecrowd 1799 - O Rato no Labirinto
// https://judge.beecrowd.com/pt/problems/view/1799

use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

fn next<T: FromStr>(tokens: &mut std::str::SplitWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn indice(nomes: &mut Vec<String>, nome: String) -> usize {
    for i in 0..nomes.len() {
        if nomes[i] == nome {
            return i;
        }
    }
    nomes.push(nome);
    nomes.len() - 1
}

fn bfs(vizinhos: &Vec<Vec<usize>>, origem: usize, destino: usize) -> i32 {
    let mut dist: Vec<i32> = vec![-1; vizinhos.len()];
    let mut fila: VecDeque<usize> = VecDeque::new();
    dist[origem] = 0;
    fila.push_back(origem);
    while let Some(u) = fila.pop_front() {
        for &v in &vizinhos[u] {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                fila.push_back(v);
            }
        }
    }
    dist[destino]
}

fn main() {
    let mut entrada = String::new();
    io::stdin().read_to_string(&mut entrada).unwrap();
    let mut tokens = entrada.split_whitespace();

    let n: usize = next(&mut tokens);
    let m: usize = next(&mut tokens);

    let mut nomes: Vec<String> = Vec::new();
    let mut vizinhos: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let a: String = next(&mut tokens);
        let b: String = next(&mut tokens);
        let ia = indice(&mut nomes, a);
        let ib = indice(&mut nomes, b);
        while vizinhos.len() < nomes.len() {
            vizinhos.push(Vec::new());
        }
        vizinhos[ia].push(ib);
        vizinhos[ib].push(ia);
    }

    let comeco = indice(&mut nomes, String::from("Entrada"));
    let queijo = indice(&mut nomes, String::from("*"));
    let fim = indice(&mut nomes, String::from("Saida"));
    while vizinhos.len() < nomes.len() {
        vizinhos.push(Vec::new());
    }

    println!("{}", bfs(&vizinhos, comeco, queijo) + bfs(&vizinhos, queijo, fim));
}
