use makepad_flowkit::*;
use makepad_tessellation::prelude::*;
use makepad_widgets::*;

// Fixme: drag and drop nodes

live_design! {
  use link::theme::*;
  use link::shaders::*;
  use link::widgets::*;

  COLOR_BG = #f7f7f7

  FlowkitCanvas = {{FlowkitCanvas}} {
    width: Fill, height: Fill
    flow: Overlay
  }

  App = {{App}} {
      ui: <Window> {
          window: {
              title: "makepad Flowkit Canvas"
              inner_size: vec2(1400, 900)
          }
          body = <View> {
              width: Fill, height: Fill
              flow: Down
              show_bg: true
              draw_bg: {
                  fn pixel(self) -> vec4 {
                      return (COLOR_BG);
                  }
              }

              canvas = <FlowkitCanvas> {
                  width: Fill, height: Fill
                  visible: true
              }
          }
      }
  }
}

#[derive(Live, LiveHook, Widget)]
struct FlowkitCanvas {
    #[live]
    draw_shape: DrawColor,

    #[live]
    draw_line: DrawLine,

    #[deref]
    view: View,

    #[rust]
    positions: [glam::Vec2; 4],

    #[rust]
    edges: [(usize, usize, EdgePath); 4],

    #[rust]
    initialized: bool,

    #[rust]
    tess: Tessellator<StrokeTessellator>,
}

impl Widget for FlowkitCanvas {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.initialized {
            self.initialized = true;

            self.tess = Tessellator::default();

            self.positions = [
                glam::Vec2::new(50.0, 50.0) + glam::Vec2::new(100.0, 100.0) / 2.0,
                glam::Vec2::new(250.0, 250.0) + glam::Vec2::new(100.0, 100.0) / 2.0,
                glam::Vec2::new(550.0, 550.0) + glam::Vec2::new(100.0, 100.0) / 2.0,
                glam::Vec2::new(250.0, 450.0) + glam::Vec2::new(100.0, 100.0) / 2.0,
            ];

            self.edges = [
                (
                    0,
                    1,
                    EdgePath {
                        source: (
                            glam::Vec2::new(0.5, 0.0) * glam::Vec2::new(100.0, 100.0),
                            EdgeAnchor::Right,
                        ),
                        target: (
                            glam::Vec2::new(-0.5, 0.0) * glam::Vec2::new(100.0, 100.0),
                            EdgeAnchor::Left,
                        ),
                        edge_type: EdgeType::Straight,
                        ..Default::default()
                    },
                ),
                (
                    0,
                    2,
                    EdgePath {
                        source: (
                            glam::Vec2::new(-0.5, 0.0) * glam::Vec2::new(100.0, 100.0),
                            EdgeAnchor::Left,
                        ),
                        target: (
                            glam::Vec2::new(0.5, 0.0) * glam::Vec2::new(100.0, 100.0),
                            EdgeAnchor::Right,
                        ),
                        edge_type: EdgeType::StraightStep,
                        ..Default::default()
                    },
                ),
                (
                    0,
                    3,
                    EdgePath {
                        source: (
                            glam::Vec2::new(0.25, 0.5) * glam::Vec2::new(100.0, 100.0),
                            EdgeAnchor::Bottom,
                        ),
                        target: (
                            glam::Vec2::new(-0.25, -0.5) * glam::Vec2::new(100.0, 100.0),
                            EdgeAnchor::Top,
                        ),
                        edge_type: EdgeType::Curve,
                        ..Default::default()
                    },
                ),
                (
                    2,
                    3,
                    EdgePath {
                        source: (
                            glam::Vec2::new(-0.5, 0.0) * glam::Vec2::new(100.0, 100.0),
                            EdgeAnchor::Left,
                        ),
                        target: (
                            glam::Vec2::new(0.5, 0.0) * glam::Vec2::new(100.0, 100.0),
                            EdgeAnchor::Right,
                        ),
                        edge_type: EdgeType::SmoothStep,
                        ..Default::default()
                    },
                ),
            ];
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Begin drawing
        cx.begin_turtle(walk, Layout::flow_overlay());

        self.view.draw_walk(cx, scope, walk)?;

        self.draw_shape.color = vec4(0.30, 0.30, 0.45, 1.0);
        self.draw_shape.draw_abs(
            cx,
            Rect {
                pos: dvec2(50.0, 50.0),
                size: dvec2(100.0, 100.0),
            },
        );

        self.draw_shape.color = vec4(0.50, 0.50, 0.50, 1.0);
        self.draw_shape.draw_abs(
            cx,
            Rect {
                pos: dvec2(250.0, 250.0),
                size: dvec2(100.0, 100.0),
            },
        );

        self.draw_shape.color = vec4(0.20, 0.20, 0.20, 1.0);
        self.draw_shape.draw_abs(
            cx,
            Rect {
                pos: dvec2(550.0, 550.0),
                size: dvec2(100.0, 100.0),
            },
        );

        self.draw_shape.color = vec4(0.23, 0.23, 0.23, 1.0);
        self.draw_shape.draw_abs(
            cx,
            Rect {
                pos: dvec2(250.0, 450.0),
                size: dvec2(100.0, 100.0),
            },
        );

        for (source_id, target_id, edge) in self.edges {
            let source_shape_pos = self.positions[source_id];
            let target_shape_pos = self.positions[target_id];
            let (source_offset, source_edge) = edge.source;
            let (target_offset, target_edge) = edge.target;
            let source_pos = source_shape_pos + source_offset;
            let target_pos = target_shape_pos + target_offset;

            let connection: Connection = EdgePath {
                source: (glam::Vec2::new(source_pos.x, source_pos.y), source_edge),
                target: (glam::Vec2::new(target_pos.x, target_pos.y), target_edge),
                edge_type: edge.edge_type,
                ..Default::default()
            }
            .into();

            let buffers = connection.build_with(
                Mode {
                    // keeps line width at 1.0
                    options: StrokeOptions::DEFAULT.with_line_width(1.0),
                },
                &mut self.tess,
            );

            let color = vec4(1.0, 0.0, 0.0, 1.0);
            let width = 2.0;
            self.draw_line.draw_with(cx, buffers, color, width);
        }

        cx.end_turtle();
        DrawStep::done()
    }
}

#[derive(Live, LiveHook)]
struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        self.match_event(cx, event);
    }
}

impl MatchEvent for App {
    fn match_event(&mut self, _cx: &mut Cx, _event: &Event) {}
}

impl Default for App {
    fn default() -> Self {
        Self {
            ui: WidgetRef::default(),
        }
    }
}

app_main!(App);

fn main() {
    app_main();
}
