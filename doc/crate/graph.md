Graph crates and js libraries
===

Related urls
---


petgraph (rust)
---

Widely used graph crate (almost ubiquitous)
Good structure, good algorithms
NO VISUALIZATION

sigma.js (js)
---

Used this for r-sat
Decent implementation of visualization algorithms
Canvas & WebGL, no svg
Sibling to graphology

graphology (js)
---

Graph backend, split off from sigma.js
Prints to svg
Looks a bit awkward, but haven't delved too deep

fdg (rust with some wasm)
---

Force directed graph drawing
Recommended from https://www.reddit.com/r/rust/comments/y2iylr/gui_tool_to_visualize_a_large_number_of_nodes_in/
Built on top of petgraph
Has a wasm implementation, though self professed as not a js expert
