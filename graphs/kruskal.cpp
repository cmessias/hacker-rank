// https://www.hackerrank.com/challenges/kruskalmstrsub

#include <algorithm>
#include <iostream>
#include <ostream>
#include <vector>

using std::vector;

vector<size_t> make_set(size_t n) {
  vector<size_t> set(n);
  for (size_t i = 0; i < n; ++i) {
    set[i] = i;
  }
  return set;
}

size_t find(size_t x, vector<size_t> &parent) {
  if (x == parent[x]) {
    return x;
  } else {
    return parent[x] = find(parent[x], parent);
  }
}

void make_union(size_t x, size_t y, vector<size_t> &parent, vector<int> &rank) {
  size_t x_parent = find(x, parent);
  size_t y_parent = find(y, parent);

  if (x_parent == y_parent) {
    return;
  }

  if (rank[x_parent] < rank[y_parent]) {
    std::swap(x_parent, y_parent);
  }

  parent[y_parent] = x_parent;

  if (rank[x_parent] == rank[y_parent]) {
    rank[x_parent] += 1;
  }
}

struct Edge {
  int32_t from;
  int32_t to;
  int32_t weight;

public:
  Edge(int32_t from, int32_t to, int32_t weight)
      : from(from), to(to), weight(weight){};
};

int64_t kruskals(size_t g_nodes, vector<Edge> edges) {
  vector<size_t> parent = make_set(g_nodes);
  vector<int32_t> rank(g_nodes, 0);

  std::sort(edges.begin(), edges.end(),
            [](Edge &u, Edge &v) { return u.weight < v.weight; });

  int64_t total = 0;

  for (auto edge : edges) {
    size_t x = find(edge.from, parent);
    size_t y = find(edge.to, parent);
    if (x != y) {
      total += edge.weight;
      make_union(x, y, parent, rank);
    }
  }

  return total;
}

int main() {
  size_t n_nodes, n_edges;
  std::cin >> n_nodes >> n_edges;

  vector<Edge> edges;
  edges.reserve(n_edges);

  for (size_t i = 0; i < n_edges; ++i) {
    int32_t u, v, w;
    std::cin >> u >> v >> w;

    Edge edge{u - 1, v - 1, w};
    edges.push_back(edge);
  }

  std::cout << kruskals(n_nodes, edges) << std::endl;
}
