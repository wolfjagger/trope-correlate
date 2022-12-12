use derive_more::Display;
use fdg_sim::{ForceGraph, Node as fdg_Node, Simulation, SimulationParameters};
use petgraph::{
  Directed, EdgeType, Graph, Undirected,
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
  pub simulation: Simulation<TropeNode, TropeEdge, Directed>
}

impl DirectedTropeGraph {

  pub fn new(g: Graph<TropeNode, TropeEdge, Directed>) -> Self {
    let force_graph = begin_force_graph(&g);
    let simulation = Simulation::from_graph(force_graph, SimulationParameters::default());
    Self {
      graph: g,
      simulation
    }
  }

}

/// Undirected trope-related graph that can be plotted
pub struct UndirectedTropeGraph {
  pub graph: Graph<TropeNode, TropeEdge, Undirected>,
  pub simulation: Simulation<TropeNode, TropeEdge, Undirected>
}

impl UndirectedTropeGraph {

  pub fn new(g: Graph<TropeNode, TropeEdge, Undirected>) -> Self {
    let force_graph = begin_force_graph(&g);
    let simulation = Simulation::from_graph(force_graph, SimulationParameters::default());
    Self {
      graph: g,
      simulation
    }
  }

}


fn begin_force_graph<Ty>(graph: &Graph<TropeNode, TropeEdge, Ty>)
-> ForceGraph<TropeNode, TropeEdge, Ty>
where Ty: EdgeType {
  graph.map(
    |_idx, n| {
      let new_node = n.clone();
      let name = n.name.clone();
      fdg_Node::new(&name, new_node)
    },
    |_idx, e| e.clone()
  ).into()
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
