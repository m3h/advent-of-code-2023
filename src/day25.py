#!/usr/bin/env python3
from pprint import pprint
from collections import defaultdict
import copy
import random

def merge(s, t, G):
    for v in range(len(G)):
        G[v][s] += G[v][t]
        G[v][t] = 0
    for v in range(len(G[t])):
        G[s][v] += G[t][v]
        G[t][v] = 0
    G[s][s] = 0

def min_cut_phase(G: list[list[int]], merge_mapping: dict[int, set[int]]):
    s = None
    t = random.choice(vertices(G))
    t = 2
    # t = 1
    c = None
    cut = None

    A = {t}

    while len(A) != len(vertices(G)):
        s = t

        max_v = None
        max_c = float('-inf')
        max_cut = set()
        for v in vertices(G):
            if v in A:
                continue
            vc = 0

            this_cut = set()
            for a in A:
                w = G[v][a]
                if w > 0:
                    vc += w
                    this_cut.add(a)
            if vc > max_c:
                max_c = vc
                max_v = v
                max_cut = this_cut
        c, t = max((sum([G[v][a] for a in A]), v) for v in vertices(G) if v not in A)
        # t = max_v
        # cut = max_cut
        # c = max_c
        A.add(t)

    S, T = set(merge_mapping[s]), set(merge_mapping[t])
    max_cut = [set(merge_mapping[v]) for v in max_cut]
    merge(s, t, G)
    merge_mapping[s] |= merge_mapping[t]
    del merge_mapping[t]


    return S, T, c, max_cut
    return T, max_cut, c
    return S, T, c

def vertices(G: list):
    return [v for v in range(len(G)) if sum(G[v]) > 0]

def min_cut(G):

    merge_mapping = {v: {v} for v in range(len(G))}
    s_min, t_min , c_min, cut_min = None, None, float('inf'), None
    while len(vertices(G)) > 1:
        s, t, c, cut = min_cut_phase(G, merge_mapping)

        if c < c_min:
            s_min, t_min, c_min, cut_min = s, t, c, cut
    
    return s_min, t_min, c_min, cut_min

def main():
    input = """jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"""

    with open('input/2023/day25.txt') as f:
        input = f.read()

    vertices = defaultdict(set)
    input_mapping = dict()
    node_count = 0
    def get_node_index(n: str) -> int:
        nonlocal node_count
        for k, v in input_mapping.items():
            if v == n:
                return k
        k = node_count
        input_mapping[k] = n
        node_count += 1
        return k 

    for line in input.strip().splitlines():
        line = line.strip()
        lhs, rhs = line.split(': ')
        rhs = set(rhs.split(' '))

        v = get_node_index(lhs)
        for n in rhs:
            vertices[v].add(get_node_index(n))

    G = [[0 for _ in range(node_count)] for _ in range(node_count)]
    # G = [list([0]*node_count)]*node_count
    for v in vertices:
        for c in vertices[v]:
            G[v][c] = 1
            G[c][v] = 1

    # 0-1-2
    # |\|
    # | 3
    # 4/
    # G = [
    #         #0, 1, 2, 3, 4
    #         [0, 1, 0, 1, 1,], # 0
    #         [1, 0, 1, 1, 0,], # 1
    #         [0, 1, 0, 0, 0,], # 2
    #         [1, 1, 0, 0, 0,], # 3
    #         [1, 0, 0, 1, 0,], # 4
    # ]
    # G = [
    #     #0, 1, 2, 3, 4, 5, 6, 7, 8
    #     [0, 0, 0, 0, 0, 0, 0, 0, 0], # 0
    #     [0, 0, 2, 0, 0, 3, 0, 0, 0], # 1
    #     [0, 2, 0, 3, 0, 2, 2, 0, 0], # 2
    #     [0, 0, 3, 0, 4, 0, 0, 2, 0], # 3
    #     [0, 0, 0, 4, 0, 0, 0, 2, 2], # 4
    #     [0, 3, 2, 0, 0, 0, 3, 0, 0], # 5
    #     [0, 0, 2, 0, 0, 3, 0, 1, 0], # 6
    #     [0, 0, 0, 2, 2, 0, 1, 0, 3], # 7
    #     [0, 0, 0, 0, 2, 0, 0, 3, 0], # 8
    # ]

    pprint(G)

    G_original = copy.deepcopy(G)
    s, t, cost, cut = min_cut(G)
    print(s, t, cost, cut)

    print('s', [input_mapping[v] for v in s])
    print('t', [input_mapping[v] for v in t])
    print('cut', [[input_mapping[v] for v in vs] for vs in cut])

    print('ansert')
    for vs in cut:
        for v_a in vs:
            for v_b in t:
                if G_original[v_a][v_b] > 0:
                    print(input_mapping[v_a], '/', input_mapping[v_b])
if __name__ == "__main__":
    main()
