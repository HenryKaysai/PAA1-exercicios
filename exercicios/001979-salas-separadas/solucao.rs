// Beecrowd 1979 - Salas Separadas
// https://judge.beecrowd.com/pt/problems/view/1979

/*
saber se é possível, com apenas duas salas de aula disponíveis, combinando os alunos para que os amigos não fiquem juntos.
A primeira linha é um número inteiro N (2 ≤ N ≤ 100) (id do aluno).
A próxima linha é uma lista com as relações de amizade.
Não é relevante o número de pessoas em cada sala
Se houver uma amizade entre x e y há amizade entre y e x.
A entrada termina quando N = 0, e não deve ser processada.
Para cada caso, imprimir uma linha "SIM", se possível ter os alunos para que não haja dois amigos fazendo a
prova na mesma sala de aula, e "NAO" de outra forma.
*/
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut linhas = stdin.lock().lines();

    loop {
        let n: usize = linhas.next().unwrap().unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        // adj[i] = amigos do aluno i
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

        for _ in 0..n {
            let id: usize = linhas.next().unwrap().unwrap().trim().parse().unwrap();
            let amigos: Vec<usize> = linhas
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            // amizade é mútua: guarda nos dois sentidos
            for amigo in amigos {
                adj[id].push(amigo);
                adj[amigo].push(id);
            }
        }

        if eh_bipartido(&adj, n) {
            println!("SIM");
        } else {
            println!("NAO");
        }
    }
}

/*
1 = 0 (inicial)
pop 1 → amigos 2 e 4: sem sala → ambos ganham 1, entram na fila
pop 2 → amigos 1, 4, 4:
          1: cor 0, diferente de 1 → ok
          4: cor 1, IGUAL à do 2 (1) → conflito → return false
*/
fn eh_bipartido(adj: &Vec<Vec<usize>>, n: usize) -> bool {
    // cor[i]: -1 = sem sala, 0 = sala A, 1 = sala B
    let mut cor: Vec<i32> = vec![-1; n + 1];

    // passa por todos os alunos
    for inicio in 1..=n {
        if cor[inicio] != -1 {
            continue; // já tem sala
        }

        cor[inicio] = 0; // começa na sala A

        let mut fila: VecDeque<usize> = VecDeque::new();
        fila.push_back(inicio);

        while let Some(atual) = fila.pop_front() {
            for &amigo in &adj[atual] {
                if cor[amigo] == -1 {
                    // sem sala
                    cor[amigo] = 1 - cor[atual];
                    fila.push_back(amigo);
                } else if cor[amigo] == cor[atual] {
                    // amigos na mesma sala
                    return false;
                }
            }
        }
    }

    true
}
