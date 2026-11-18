// Beecrowd 1082 - Componentes Conexos
// https://judge.beecrowd.com/pt/problems/view/1082

/*
Se houver um caminho entre u e v.
Um grafo com apenas um componente é um grafo conexo.
Um grafo é desconexo se for formado por dois ou mais componentes conexos.
Componente conexo: Os componentes de um grafo são subgrafos conectados deste grafo.

Cada entrada tem cada um dos vértices, bordas e os
links entre os nós pelas bordas
liste todos os componentes conexos que existem no grafo

A primeira linha, número inteiro N: representa o número de casos de teste
Cada caso contém dois números, respectivamente, o número de vértices (V) e
bordas (E)
Cada E representando uma das bordas que conectam esse vértice
Cada vértice é representado por uma letra minúscula do alfabeto
Isso significa 26 vértices no máximo (a-z)
Cada grafo tem pelo menos um componente conexo.

Para cada caso, imprima a mensagem Caso #n: indicando o número de
caso de teste
Siga o vértice de cada segmento, um segmento por linha, separado por vírgulas
Terminando o caso de teste uma mensagem deve ser impressa
indicando o número de componentes conectados do grafo. Cada caso de
teste deve ter uma linha em branco impressa no final, incluindo a última.
Importante: os vértices devem ser impressos em ordem crescente e se o caminho
existe de a a b significa que existe caminho de b para a.
*/

use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut entrada = String::new();
    io::stdin().read_to_string(&mut entrada).unwrap();
    let mut tokens = entrada.split_whitespace();

    for i in 1..=tokens.next().unwrap().parse().unwrap() {
        let v: usize = tokens.next().unwrap().parse().unwrap();
        let a: usize = tokens.next().unwrap().parse().unwrap();
        let mut lista: Vec<Vec<usize>> = vec![Vec::new(); v];
        let mut visitado = vec![false; v];


        for _ in 0..a {
            let x: &str = tokens.next().unwrap();
            let y: &str = tokens.next().unwrap();

            let idx: usize = (x.as_bytes()[0] - b'a') as usize;
            let idy: usize = (y.as_bytes()[0] - b'a') as usize;

            lista[idx].push(idy);
            lista[idy].push(idx);
        }
        println!("Case #{}:", i);
        let mut componentes: usize = 0;

        for s in 0..v {
            if visitado[s] {
                continue;
            }
            componentes += 1;
            let mut componente: Vec<usize> = Vec::new();
            let mut fila: VecDeque<usize> = VecDeque::new();

            visitado[s] = true;
            fila.push_back(s);

            while let Some(atual) = fila.pop_front() {
                componente.push(atual);
                for &vizinho in &lista[atual] {
                    if !visitado[vizinho] {
                        visitado[vizinho] = true;
                        fila.push_back(vizinho);
                    }
                }
            }
            componente.sort();

            for &vertice in &componente {
                let letra: char = (vertice as u8 + b'a') as char;
                print!("{},", letra);
            }
            println!();
        }
        println!("{} connected components\n", componentes);
    }
}
