use gpui::{
    AppContext, Application, Context, Hsla, InteractiveElement, IntoElement, ParentElement,
    PathBuilder, PathStyle, Render, SharedString, StrokeOptions, Styled, TitlebarOptions, Window,
    WindowOptions, canvas, div, hsla, px, rgb, white,
};
use gpui_flowkit::{Connection, EdgeAnchor, EdgePath, EdgeType};

// Fixme: drag and drop nodes

struct FlowkitCanvas {}

impl FlowkitCanvas {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for FlowkitCanvas {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let positions = [
            glam::Vec2::new(50.0, 50.0) + glam::Vec2::new(100.0, 100.0) / 2.0,
            glam::Vec2::new(250.0, 250.0) + glam::Vec2::new(100.0, 100.0) / 2.0,
            glam::Vec2::new(550.0, 550.0) + glam::Vec2::new(100.0, 100.0) / 2.0,
            glam::Vec2::new(250.0, 450.0) + glam::Vec2::new(100.0, 100.0) / 2.0,
        ];
        let edges = [
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

        div()
            .bg(hsla(0.0, 0.0, 0.97, 1.0))
            .size_full()
            .child(
                canvas(
                    |_, _, _| {},
                    move |_, _, window, _| {
                        for (source_id, target_id, edge) in edges {
                            let source_shape_pos = positions[source_id];
                            let target_shape_pos = positions[target_id];
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

                            let builder = PathBuilder::from(connection).with_style(
                                PathStyle::Stroke(StrokeOptions::DEFAULT.with_line_width(2.0)),
                            );

                            if let Ok(path) = builder.build() {
                                window.paint_path(path, Hsla::red());
                            }
                        }
                    },
                )
                .absolute()
                .left(px(0.0))
                .top(px(0.0))
                .size_full(),
            )
            .children([
                div()
                    .id(0)
                    .absolute()
                    .w(px(100.0))
                    .h(px(100.0))
                    .left(px(50.0))
                    .top(px(50.0))
                    .border(px(1.0))
                    .border_color(rgb(0x92c5ff))
                    .flex()
                    .justify_center()
                    .items_center()
                    .child("Node"),
                div()
                    .id(1)
                    .absolute()
                    .w(px(100.0))
                    .h(px(100.0))
                    .left(px(250.0))
                    .top(px(250.0))
                    .border(px(1.0))
                    .border_color(rgb(0x92c5ff))
                    .bg(rgb(0xff))
                    .rounded_xs()
                    .flex()
                    .justify_center()
                    .items_center()
                    .text_color(white())
                    .child("Node"),
                div()
                    .id(2)
                    .absolute()
                    .w(px(100.0))
                    .h(px(100.0))
                    .left(px(550.0))
                    .top(px(550.0))
                    .border(px(1.0))
                    .border_color(rgb(0x92c5ff))
                    .flex()
                    .justify_center()
                    .items_center()
                    .child("Node"),
                div()
                    .id(3)
                    .absolute()
                    .w(px(100.0))
                    .h(px(100.0))
                    .left(px(250.0))
                    .top(px(450.0))
                    .border(px(1.0))
                    .border_color(rgb(0x92c5ff))
                    .flex()
                    .justify_center()
                    .items_center()
                    .child("Node"),
            ])
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                focus: true,
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::new_static("gpui Flowkit Canvas")),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| FlowkitCanvas::new(window, cx)),
        )
        .unwrap();
        cx.on_window_closed(|cx| {
            cx.quit();
        })
        .detach();
        cx.activate(true);
    });
}
