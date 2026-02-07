# Flowkit

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/viz-rs/flowkit#license)
[![Crates.io](https://img.shields.io/crates/v/flowkit.svg)](https://crates.io/crates/flowkit)
[![Downloads](https://img.shields.io/crates/d/flowkit.svg)](https://crates.io/crates/flowkit)
[![Docs](https://docs.rs/flowkit/badge.svg)](https://docs.rs/flowkit/latest/flowkit/)

A universal UI workflow library.

Neat, beautiful, and easy to use.

## Features

- Support multiple kinds of [`EdgeType`]s:
  - **SmoothStep**: smoothing and rounding corners by the [`squircle`]
  - **StraightStep**: straight steps
  - **Straight**: a straight line
  - **Curve**: a bezier curve

- Adapt to multiple frameworks
  - [bevy_flowkit]
  - [egui_flowkit]
  - [gpui_flowkit]
  - [makepad_flowkit]

- Support SVG's path output

[`EdgeType`]: https://docs.rs/flowkit/latest/flowkit/edge/enum.EdgeType.html
[`squircle`]: https://www.figma.com/blog/desperately-seeking-squircles/
[bevy_flowkit]: https://docs.rs/bevy_flowkit
[egui_flowkit]: https://docs.rs/egui_flowkit
[gpui_flowkit]: https://docs.rs/gpui_flowkit
[makepad_flowkit]: https://docs.rs/makepad_flowkit

## Examples

- [node based](examples/bevy/src/node_based.rs): A node-based UI example for Bevy Engine.

  ```console
  cargo run --example node_based
  ```

- [egui canvas](examples/egui/src/canvas.rs): A simple flow chat for egui.

  ```console
  cargo run --example egui_canvas
  ```

- [gpui canvas](examples/gpui/src/canvas.rs): A simple flow chat for gpui.

  ```console
  cargo run --example gpui_canvas
  ```

- [makepad canvas](examples/makepad/src/canvas.rs): A simple flow chat for makepad.

  ```console
  cargo run --example makepad_canvas
  ```

## License

Flowkit is free, open source and permissively licensed!
Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
