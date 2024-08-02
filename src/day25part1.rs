#[derive(Debug, Clone)]
struct Graph {
    edges: std::collections::HashMap<usize, std::collections::HashMap<usize, usize>>,
    vertex_names: std::collections::HashMap<String, usize>,
    vertex_ids: std::collections::HashMap<usize, String>,
    merge_mapping: std::collections::HashMap<usize, std::collections::HashSet<usize>>,
    vertex_count: usize,
}

// https://e-maxx.ru/bookz/files/stoer_wagner_mincut.pdf
// see Stoer-Wagner Minimum Cut Algorithm

impl Graph {
    fn vertex_id_lookup(&mut self, s: &str) -> usize {
        match self.vertex_names.get(s) {
            Some(v) => *v,
            None => {
                let id = self.vertex_count;
                self.vertex_count += 1;

                self.vertex_names.insert(s.to_string(), id);
                self.vertex_ids.insert(id, s.to_string());
                id
            }
        }
    }
    fn vertex_name_lookup(&self, id: usize) -> Option<&String> {
        self.vertex_ids.get(&id)
    }
    fn new() -> Graph {
        Graph {
            edges: std::collections::HashMap::new(),
            vertex_names: std::collections::HashMap::new(),
            vertex_ids: std::collections::HashMap::new(),
            merge_mapping: std::collections::HashMap::new(),
            vertex_count: 0,
        }
    }

    fn _insert_vertex_half(&mut self, v: usize, n: usize, w: usize) {
        let mut map = self.edges.get_mut(&v);
        if map.is_some() {
            map.unwrap().insert(n, w);
        } else {
            self.edges.insert(v, std::collections::HashMap::new());
            self._insert_vertex_half(v, n, w);
        }
    }

    fn insert_vertex(&mut self, v: &str, n: &str, w: usize) {
        let v = self.vertex_id_lookup(v);
        let n = self.vertex_id_lookup(&n);

        self._insert_vertex_half(v, n, w);
        self._insert_vertex_half(n, v, w);

        self.merge_mapping
            .insert(v, std::collections::HashSet::from([v]));
        self.merge_mapping
            .insert(n, std::collections::HashSet::from([n]));
    }

    fn merge(&mut self, s: usize, t: usize) {
        let t_merge_mapping = self.merge_mapping.remove(&t).unwrap();
        let mut s_merge_mapping = self.merge_mapping.get_mut(&s).unwrap();
        s_merge_mapping.extend(t_merge_mapping);

        let t_edges = self.edges.remove(&t).unwrap();
        for (v, w) in t_edges {
            let new_s_w = *self.edges.get(&s).unwrap().get(&v).unwrap_or(&0) + w;

            self.edges.get_mut(&v).unwrap().remove(&t);
            if v == s {
                continue;
            }

            self.edges.get_mut(&v).unwrap().insert(s, new_s_w);
            self.edges.get_mut(&s).unwrap().insert(v, new_s_w);
        }
    }

    fn min_cut_phase(
        &mut self,
    ) -> (
        std::collections::HashSet<usize>,
        std::collections::HashSet<usize>,
        usize,
    ) {
        let mut s = 0;
        let mut t = *self.edges.keys().nth(0).unwrap();

        let mut c = usize::MIN;

        let mut a = std::collections::HashSet::from([t]);

        while a.len() != self.edges.len() {
            s = t;

            c = usize::MIN;
            for v in self.edges.keys().filter(|k| !a.contains(&k)) {
                let mut cost_v = 0;

                for (n, w) in self.edges.get(v).unwrap() {
                    if a.contains(n) {
                        cost_v += w;
                    }
                }
                if cost_v > c {
                    c = cost_v;
                    t = *v;
                }
            }
            a.insert(t);
        }

        let cut_rhs: std::collections::HashSet<_> = self
            .edges
            .get(&t)
            .unwrap()
            .iter()
            .filter(|(v, w)| a.contains(&v))
            .flat_map(|(v, w)| self.merge_mapping.get(v).unwrap().iter())
            .map(|v| *v)
            .collect::<std::collections::HashSet<_>>()
            .clone();
        let cut_lhs = self.merge_mapping.get(&t).unwrap().clone();

        self.merge(s, t);

        return (cut_lhs, cut_rhs, c);
    }

    fn min_cut(
        &self,
    ) -> (
        std::collections::HashSet<String>,
        std::collections::HashSet<String>,
        usize,
    ) {
        let mut g = self.clone();

        let mut cut_lhs_min = std::collections::HashSet::new();
        let mut cut_rhs_min = std::collections::HashSet::new();
        let mut cost_min = usize::MAX;

        while g.edges.len() > 1 {
            println!("{0}", g.edges.len());
            let (cut_lhs, cut_rhs, cost) = g.min_cut_phase();

            if cost < cost_min {
                cost_min = cost;
                cut_lhs_min = cut_lhs;
                cut_rhs_min = cut_rhs;
            }
        }

        let cut_lhs = cut_lhs_min
            .iter()
            .map(|v| self.vertex_name_lookup(*v).unwrap().clone())
            .collect();
        let cut_rhs = cut_rhs_min
            .iter()
            .map(|v| self.vertex_name_lookup(*v).unwrap().clone())
            .collect();
        return (cut_lhs, cut_rhs, cost_min);
    }
}
impl From<&str> for Graph {
    fn from(s: &str) -> Self {
        let mut g = Graph::new();

        for line in s.trim().lines() {
            let lhs_rhs_split = line.split(": ").collect::<Vec<_>>();
            let v = lhs_rhs_split.get(0).unwrap().trim();
            let rhs = lhs_rhs_split.get(1).unwrap().trim();

            for n in rhs.split_whitespace() {
                let n = n.trim();
                g.insert_vertex(v, n, 1);
            }
        }
        return g;
    }
}

#[aoc(day25, part1)]
fn day25part1(input: &str) -> usize {
    // let input = "jqt: rhn xhk nvd
    // rsh: frs pzl lsr
    // xhk: hfx
    // cmg: qnr nvd lhk bvb
    // rhn: xhk bvb hfx
    // bvb: xhk hfx
    // pzl: lsr hfx nvd
    // qnr: nvd
    // ntq: jqt hfx bvb xhk
    // nvd: lhk
    // lsr: lhk
    // rzs: qnr cmg lsr rsh
    // frs: qnr lhk lsr";

    // a-b-c
    // |\|
    // | d
    // e/

    let graph = Graph::from(input);
    println!("{graph:?}");

    for v in graph.edges.keys() {
        println!("{0}", v);
        println!("{0}", graph.vertex_name_lookup(*v).unwrap());
        println!("{0:?}", graph.edges.get(v).unwrap());
        println!(
            "{0:?}",
            graph
                .edges
                .get(v)
                .unwrap()
                .iter()
                .map(|(v, w)| format!("{0} {1}", graph.vertex_name_lookup(*v).unwrap(), w))
                .collect::<Vec<_>>()
        );
        println!();
    }
    let (lhs, rhs, cost) = graph.min_cut();

    let rhs: Vec<_> = graph
        .edges
        .keys()
        .filter_map(|v| {
            let v_name = graph.vertex_name_lookup(*v).unwrap();
            if !lhs.contains(v_name) {
                Some(v_name)
            } else {
                None
            }
        })
        .collect();
    println!("lhs: {lhs:?}");
    println!("rhs: {rhs:?}");
    println!("cost: {cost}");

    return lhs.len() * rhs.len();
}
