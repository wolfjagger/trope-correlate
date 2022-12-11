use derive_more::Display;
use fdg_sim::{ForceGraph, Node as fdg_Node};
use petgraph::{
  Directed, Graph, Undirected,
  algo::is_isomorphic_matching
};


/// Data/weight for each node
#[derive(Clone, Debug, Display, PartialEq)]
pub struct TropeNode {
  pub name: String,
}
/// Data/weight for each edge
#[derive(Clone, Debug, Display, PartialEq)]
pub struct TropeEdge { }


/// Directed trope-related graph that can be plotted
pub struct DirectedTropeGraph {
  pub graph: Graph<TropeNode, TropeEdge, Directed>,
}

impl DirectedTropeGraph {

  pub fn new() -> Self {
    Self {
      graph: Graph::new(),
    }
  }

  pub fn force_graph(&self) -> ForceGraph<TropeNode, TropeEdge, Directed> {
    self.graph.map(
      |_idx, n| fdg_Node::new(&n.name, n.clone()),
      |_idx, e| e.clone()
    ).into()
  }

}

/// Undirected trope-related graph that can be plotted
pub struct UndirectedTropeGraph {
  pub graph: Graph<TropeNode, TropeEdge, Undirected>,
}

impl UndirectedTropeGraph {

  pub fn new() -> Self {
    Self {
      graph: Graph::new_undirected(),
    }
  }

  pub fn force_graph(&self) -> ForceGraph<TropeNode, TropeEdge, Undirected> {
    self.graph.map(
      |_idx, n| fdg_Node::new(&n.name, n.clone()),
      |_idx, e| e.clone()
    ).into()
  }

}


// Need PartialEq for using graph as yew state

impl PartialEq for DirectedTropeGraph {
  fn eq(&self, other: &Self) -> bool {
    is_isomorphic_matching(&self.graph, &other.graph, |n0, n1| n0 == n1, |e0, e1| e0 == e1)
  }
}

impl PartialEq for UndirectedTropeGraph {
  fn eq(&self, other: &Self) -> bool {
    is_isomorphic_matching(&self.graph, &other.graph, |n0, n1| n0 == n1, |e0, e1| e0 == e1)
  }
}
