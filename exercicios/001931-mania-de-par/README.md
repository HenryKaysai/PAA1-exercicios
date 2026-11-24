# Problema 1931: Mania de Par

**Problema:** [Mania de Par](https://judge.beecrowd.com/pt/problems/view/1931)  

---

## 📌 Descrição do Problema

Temos `C` cidades ligadas por `R` estradas de mão dupla, cada uma com um custo. Queremos ir da cidade 1 até a cidade `C`, mas só podemos viajar se for um número par de estradas. 

---

## 💡 Abordagem

Um Dijkstra comum não serve, porque o menor caminho pode ter número ímpar de estradas. O que resolve é guardar o estado.

- cada estado é `(cidade, paridade)`, onde a paridade é o número de estradas usadas até agora
- usar uma estrada `u → v` de custo `w` leva de `(u, p)` para `(v, 1 - p)` com custo `w`.
- começa em `(1, 0)` com custo 0 e roda normalmente com `BinaryHeap` (usando `Reverse` para virar min-heap), mantendo `dist[cidade][paridade]`.
-  `dist[C][0]`. Se continuar infinito,  `-1`.

Usa o mesmo esquema dos outros exercícios: entrada inteira em uma `String`, `split_whitespace` e a função genérica `next<T: FromStr>`.

---

## ❌ O Que Não Funcionou

Minha primeira ideia era ler a entrada e percorrer com loops simples, sem ter pensado ainda na estrutura. Antes de implementar, descartei duas ideias:

- o caminho mais curto pode ter número ímpar de estradas, e o melhor caminho par pode ser outro.
- Achar o menor caminho e "consertar" somando uma ida e volta. 
---

## 📊 Análise de Complexidade

### Tempo

O grafo de estados tem `2C` vértices e `2 · 2R = 4R` arestas. Dijkstra com heap binário custa O((V + E) log V), então:

**O((C + R) log C)**

### Espaço

- A lista de adjacência guarda `2R` entradas → O(R).
- `dist` guarda `2(C + 1)` valores → O(C).
- O heap tem no máximo O(R) estados.

**O(C + R)**

## 🔍 Corretude

- Um caminho de `(1, 0)` até `(C, 0)` no grafo de estados corresponde a uma viagem de 1 até `C` porque a paridade do estado muda a cada estrada usada. Então as viagens válidas são exatamente os caminhos entre esses dois estados.
- Dijkstra encontra o menor custo para cada estado.
- Se `(C, 0)` não é alcançável, não existe viagem par e a resposta é `-1` .
- Repetir estradas é permitido no grafo de estados, pois ele pode voltar a uma cidade com outra paridade.

---

## 🚀 O Que Poderia ser Melhorado

- Os custos são acumulados em `u64`, ocupa mais memória que `u32`. Dá para ajustar os limites do problema.
- O código tem `unwrap()` na leitura, então uma entrada malformada causaria panic. 
- A busca poderia parar assim que `(C, 0)` fosse retirado do heap, em vez de processar todos os estados.
- Se houvesse várias consultas, valeria reaproveitar o grafo de estados.

---

## 📸 Evidência de Submissão

[[accepted.png]]
