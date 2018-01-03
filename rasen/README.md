rasen
================

The `rasen` crate contains the core graph compiler itself. It provides graph building utilities (the `Graph` struct),
various types (`rasen::types::*`) and operations (`rasen::node::*`) definitions, and SPIR-V compilation utilities (the
`Builder` struct).

It's perfectly possible to use this crate as-is by creating a `Graph` struct and buidling the module node-by-node,
though this method tends to be quite verbose:
```rust
extern crate rasen;

use rasen::prelude::*;

fn main() {
    let mut graph = Graph::new();

    // A vec3 input at location 0
    let normal = graph.add_node(Node::Input(0, TypeName::Vec(3)));

    // Some ambient light constants
    let min_light = graph.add_node(Node::Constant(TypedValue::Float(0.1)));
    let max_light = graph.add_node(Node::Constant(TypedValue::Float(1.0)));
    let light_dir = graph.add_node(Node::Constant(TypedValue::Vec3(0.3, -0.5, 0.2)));

    // The Material color (also a constant)
    let mat_color = graph.add_node(Node::Constant(TypedValue::Vec4(0.25, 0.625, 1.0, 1.0)));

    // Some usual function calls
    let normalize = graph.add_node(Node::Normalize);
    let dot = graph.add_node(Node::Dot);
    let clamp = graph.add_node(Node::Clamp);
    let multiply = graph.add_node(Node::Multiply);

    // And a vec4 output at location 0
    let color = graph.add_node(Node::Output(0, TypeName::Vec(4)));

    // Normalize the normal
    graph.add_edge(normal, normalize, 0);

    // Compute the dot product of the surface normal and the light direction
    graph.add_edge(normalize, dot, 0);
    graph.add_edge(light_dir, dot, 1);

    // Restrict the result into the ambient light range
    graph.add_edge(dot, clamp, 0);
    graph.add_edge(min_light, clamp, 1);
    graph.add_edge(max_light, clamp, 2);

    // Multiply the light intensity by the surface color
    graph.add_edge(clamp, multiply, 0);
    graph.add_edge(mat_color, multiply, 1);

    // Write the result to the output
    graph.add_edge(multiply, color, 0);

    let bytecode = build_program(&graph, ShaderType::Fragment).unwrap();
    // bytecode is now a Vec<u8> you can pass to Vulkan to create the shader module
}
```