// Beecrowd 1931 - Mania de Par
// https://judge.beecrowd.com/pt/problems/view/1931

use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

    //adjacency list: graph[u] = (v, cost)
    let mut graph: Vec<Vec<(usize, u64)>> = vec![Vec::new(); c + 1];
    for _ in 0..r {
        let u: usize = next(&mut tokens);
        let v: usize = next(&mut tokens);
        let g: u64 = next(&mut tokens);
        graph[u].push((v, g));
        graph[v].push((u, g));
    }

    match dijkstra(&graph, c) {
        Some(d) => println!("{}", d),
        None => println!("-1"),
    }
}

//dijkstra over states (city, parity of roads used so far)
//returns the minimum cost to reach city c with an even number of roads
fn dijkstra(graph: &[Vec<(usize, u64)>], c: usize) -> Option<u64> {
    let mut dist = vec![[u64::MAX; 2]; c + 1];
    let mut heap = BinaryHeap::new();

    dist[1][0] = 0;
    heap.push(Reverse((0u64, 1usize, 0usize)));

    while let Some(Reverse((d, u, p))) = heap.pop() {
        if d > dist[u][p] {
            continue;
        }
        for &(v, w) in &graph[u] {
            let np = 1 - p;
            let nd = d + w;
            if nd < dist[v][np] {
                dist[v][np] = nd;
                heap.push(Reverse((nd, v, np)));
            }
        }
    }

    if dist[c][0] == u64::MAX {
        None
    } else {
        Some(dist[c][0])
    }
}

//this function transforms beecrowd inputs into a Some typo
fn next<T: FromStr>(tokens: &mut std::str::SplitWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}
