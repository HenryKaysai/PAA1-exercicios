# Problema 1799: O Rato no Labirinto

Problema: [O Rato no Labirinto](https://judge.beecrowd.com/pt/problems/view/1799)

---

## 📌 Descrição do Problema

O labirinto tem pontos com nome e passagens entre eles, que dá pra andar nos dois sentidos. O rato sai da Entrada, tem que pegar o queijo (*) e depois ir pra Saida

---

## 💡 Abordagem

Todas as passagens têm o mesmo tamanho, então BFS já dá o menor caminho. Faço uma BFS da Entrada até o queijo, outra do queijo até a Saida e somo as duas

A leitura é igual à dos outros exercícios, com a função next

---

## ❌ O Que Não Funcionou

Criei o vetor de vizinhos com tamanho N e no teste apareceram mais nomes do que isso, então deu `index out of bounds`. Mudei pra o vetor ir crescendo conforme aparecem nomes novos

Pensei em usar Dijkstra, mas como não tem peso a BFS resolve

---

## 📊 Análise de Complexidade

### Tempo

Cada passagem lida chama indice duas vezes, e cada chamada percorre o vetor de nomes, então a leitura fica O(M · N). As duas BFS custam O(N + M)

Total: O(M · N) por causa da busca dos nomes

### Espaço

Vetor de nomes, lista de adjacência e vetor de distâncias: O(N + M)

---

## 🔍 Corretude

Na BFS os pontos saem da fila em ordem de distância, então a primeira vez que um ponto é visitado já é pelo menor caminho

O rato tem que passar pelo queijo, então o caminho é Entrada até o queijo mais queijo até a Saida. As duas partes não dependem uma da outra, então pegar o menor de cada uma dá o menor total

---

## 🚀 O Que Poderia ser Melhorado

Usar um HashMap pros nomes em vez de procurar no vetor

Como o grafo não tem direção, uma BFS só, saindo do queijo, já dava as duas distâncias

---

## 📸 Evidência de Submissão

![Accepted no Beecrowd](accepted.png)
