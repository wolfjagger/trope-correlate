use petgraph::{
  Directed, Graph, Undirected,
  algo::is_isomorphic_matching
};


/// Data/weight for each node
#[derive(Debug, PartialEq)]
pub struct TropeNode { }
/// Data/weight for each edge
#[derive(Debug, PartialEq)]
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
