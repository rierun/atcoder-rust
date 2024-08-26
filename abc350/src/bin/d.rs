#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};

use petgraph::graph::{NodeIndex, UnGraph};

fn dfs

#[fastout]
fn main() {
    // 無向グラフ
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g: UnGraph<(), ()> = UnGraph::new_undirected();
    let mut nodes = vec![];

    for _ in 0..n {
        nodes.push(g.add_node(()));
    }

    for (a, b) in ab {
        g.add_edge(nodes[a - 1], nodes[b - 1], ());
    }
}
