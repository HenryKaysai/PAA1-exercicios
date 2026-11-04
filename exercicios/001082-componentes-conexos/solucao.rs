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
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut linhas = stdin.lock().lines();

    loop {
        let n: usize = linhas.next().unwrap().unwrap().trim().parse().unwrap();
        if n == '\n' {
            break;
        }
    }
}
