use plotters::{prelude::*, coord::Shift};

use fdg_sim::{
  glam::Vec3,
  petgraph::{
    visit::{EdgeRef, IntoEdgeReferences},
    EdgeType, Undirected
  },
  Dimensions, ForceGraph, Simulation, SimulationParameters
};


// Note: the following replaces the same-named struct & method from fdg_img
//  butallows a custom backend

/// Parameters for drawing the SVG image.
pub struct Settings<N, E, Ty = Undirected> {
  /// Simulation Parameters
  pub sim_parameters: SimulationParameters<N, E, Ty>,
  /// Number of times to run the simulation
  pub iterations: usize,
  /// "Granularity of simulation updates"
  pub dt: f32,
  /// The radius of the nodes
  pub node_size: u32,
  /// RGBA color of the nodes
  pub node_color: RGBAColor,
  /// Width of the edge lines
  pub edge_size: u32,
  /// RGBA color of the edge lines
  pub edge_color: RGBAColor,
  /// RGBA background color
  pub background_color: RGBAColor,
  /// If true, the simulation will be printed on each
  pub print_progress: bool,
  /// If supplied, the names of nodes will be written
  pub text_style: Option<TextStyle<'static>>,
}

impl<N, E, Ty: EdgeType> Default for Settings<N, E, Ty> {
  fn default() -> Self {
    Self {
      sim_parameters: SimulationParameters::default(),
      iterations: 2000,
      dt: 0.035,
      node_size: 10,
      node_color: RGBAColor(0, 0, 0, 1.0),
      edge_size: 3,
      edge_color: RGBAColor(255, 0, 0, 1.0),
      background_color: RGBAColor(255, 255, 255, 1.0),
      print_progress: false,
      text_style: None,
    }
  }
}


/// Generate an image from a graph and a force.
pub fn gen_image<N, E, Ty: EdgeType, Backend: DrawingBackend>(
  graph: ForceGraph<N, E, Ty>,
  drawing_area: &DrawingArea<Backend, Shift>,
  settings: Option<Settings<N, E, Ty>>,
) -> Result<(), DrawingAreaErrorKind<Backend::ErrorType>> {

  // set up the simulation and settings
  let settings = settings.unwrap_or_default();
  let mut sim = Simulation::from_graph(graph, settings.sim_parameters);
  sim.parameters_mut().dimensions = Dimensions::Two;

  // get the nodes to their x/y positions through the simulation.
  for i in 0..settings.iterations {
    if settings.print_progress && i % 10 == 0 {
      println!("{}/{}", i, settings.iterations);
    }
    sim.update(settings.dt);
  }

  // get the size of the graph (avg of width and height to try to account for oddly shaped graphs)
  let (graph_x, graph_y): (f32, f32) = {
    let mut top = 0.0;
    let mut bottom = 0.0;
    let mut left = 0.0;
    let mut right = 0.0;

    for node in sim.get_graph().node_weights() {

      let loc = node.location;

      // add text width to the rightmost point to make sure text doesn't get cut off
      let rightmost = match settings.text_style.clone() {
        Some(ts) => {
          loc.x + ts.font.box_size(&node.name).ok().map(|x| x.0 as f32).unwrap_or(0.0)
        },
        None => loc.x,
      };

      if rightmost > right {
        right = rightmost;
      }

      if loc.x < left {
        left = loc.x;
      }

      if loc.y > top {
        top = loc.y
      }

      if loc.y < bottom {
        bottom = loc.y;
      }
    }

    ((right - left), (top - bottom))
  };

  let image_scale = 1.5;
  let (image_x, image_y) = (
    (graph_x * image_scale) as u32,
    (graph_y * image_scale) as u32,
  );

  // translate all points by graph average to (0,0)
  let mut location_sum = Vec3::ZERO;
  for node in sim.get_graph().node_weights() {
    location_sum += node.location;
  }

  let avg_vec = location_sum / sim.get_graph().node_count() as f32;
  for node in sim.get_graph_mut().node_weights_mut() {
    node.location -= avg_vec;
  }

  // translate all the points over into the image coordinate space
  for node in sim.get_graph_mut().node_weights_mut() {
    node.location.x += (image_x / 2) as f32;
    node.location.y += (image_y / 2) as f32;
  }

  // fill in the background
  drawing_area.fill(&settings.background_color).unwrap();

  // draw all the edges
  for edge in sim.get_graph().edge_references() {
    let source = &sim.get_graph()[edge.source()].location;
    let target = &sim.get_graph()[edge.target()].location;

    drawing_area.draw(&PathElement::new(
      vec![
        (source.x as i32, source.y as i32),
        (target.x as i32, target.y as i32),
      ],
      ShapeStyle {
        color: settings.edge_color,
        filled: true,
        stroke_width: settings.edge_size,
      },
    ))?;
  }

  // draw all the nodes
  for node in sim.get_graph().node_weights() {
    drawing_area.draw(&Circle::new(
      (node.location.x as i32, node.location.y as i32),
      settings.node_size,
      ShapeStyle {
        color: settings.node_color,
        filled: true,
        stroke_width: 1,
      },
    ))?;
  }

  // draw the text by nodes
  if let Some(text_style) = settings.text_style {
    for node in sim.get_graph().node_weights() {
      let pos = (
        node.location.x as i32 + (text_style.font.get_size() / 2.0) as i32,
        node.location.y as i32,
      );
      drawing_area.draw_text(node.name.as_str(), &text_style, pos)?;
    }
  }

  Ok(())

}
