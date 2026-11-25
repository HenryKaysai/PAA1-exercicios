# Problema 1152: Estradas Escuras

**Problema:** [Estradas Escuras](https://judge.beecrowd.com/pt/problems/view/1152)  

---

## 📌 Descrição do Problema

Uma cidade tem m junções ligadas por n estradas. Manter cada estrada iluminada custa k por dia. O governo quer desligar o máximo de luzes possíveis, desde que ainda dê para ir de qualquer lugar a qualquer outrl só por estradas iluminadas.

## 💡 Abordagem

As estradas que ficam acesas formam uma Árvore Geradora Mínima. Usei Kruskal.

- Ler a entrada com a função next. Guardar todas as entradas num vetor e somar o custo total
- Ordenar o vetor com sort
- Cada junção é pai de si mesma
- Para cada estrada, a função achar sobe de pai em pai até achar a raiz de cada ponta. Se esses valores forem diferentes, junta os dois (union-find).

---

## ❌ O Que Não Funcionou

- Comecei pensando em tirar as estradas mais caras uma a uma enquanto o grafo continuasse conexo. Mas cada remoção exigiria uma BFS para verificar a conectividade kkk.
- A soma dos custos pode passar do limite de i32, então usei u64.

---

## 📊 Análise de Complexidade

### Tempo

- leitura: O(n)
- ordenação das estradas: O(n log n)
- para cada estrada, duas chamadas de achar. Custa O(m) no pior caso

**O(n log n )** no pior caso

### Espaço

- vetor de estradas: O(n)
- vetor pai: O(m)
- a entrada inteira em uma String

**O(n + m)**

---

## 🔍 Corretude

- Kruskal está correto pela propriedade do corte: a estrada mais barata que liga dois grupos diferentes sempre pode fazer parte de alguma árvore mínima. Como as estradas são processadas da mais barata para a mais cara, toda estrada aceita é segura.
- Uma estrada é rejeitada só quando as duas pontas são iguais, pois criariam um ciclo.
- As estradas aceitas conectam todas as junções com custo mínimo.

---

## 🚀 O Que Poderia ser Melhorado

- compressão de caminho 
- tratar erros de entrada em vez de usar unwrap() 

---

## 📸 Evidência de Submissão

[[accepted.png]]
