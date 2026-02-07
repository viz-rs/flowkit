use eframe::egui::{
    self, Color32, Frame, Grid, Pos2, Rect, Sense, Shape, Stroke, StrokeKind, Ui, Vec2,
    Widget as _, emath, epaint::RectShape, pos2,
};

use egui_flowkit::{
    Connection, EdgeAnchor, EdgePath, EdgeType, StrokeOptions, StrokeTessellator,
    mesh::{Mode, Tessellator},
};

struct Edge {
    source_id: usize,
    target_id: usize,
    source: (Vec2, EdgeAnchor),
    target: (Vec2, EdgeAnchor),
    edge_type: EdgeType,
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        // multisampling to for anti-aliasing on lyon generated meshes
        multisampling: 4,
        ..Default::default()
    };

    eframe::run_native(
        "Flowkit Canvas",
        options,
        Box::new(|_cc| Ok(Box::<FlowkitCanvas>::default())),
    )
}

struct FlowkitCanvas {
    paint_connections: PaintConnections,
}

impl Default for FlowkitCanvas {
    fn default() -> Self {
        Self {
            paint_connections: PaintConnections::default(),
        }
    }
}

impl eframe::App for FlowkitCanvas {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.paint_connections.ui(ui);
        });
    }
}

pub struct PaintConnections {
    shapes: [Pos2; 4],

    edges: [Edge; 4],

    stroke: Stroke,

    fill: Color32,

    tess: Tessellator<StrokeTessellator>,

    mesh: bool,
}

impl Default for PaintConnections {
    fn default() -> Self {
        Self {
            shapes: [
                pos2(50.0, 50.0),
                pos2(60.0, 250.0),
                pos2(200.0, 200.0),
                pos2(250.0, 50.0),
            ],
            edges: [
                Edge {
                    source_id: 0,
                    target_id: 1,
                    source: (
                        Vec2::new(0.5, 0.0) * Vec2::new(50.0, 50.0),
                        EdgeAnchor::Right,
                    ),
                    target: (
                        Vec2::new(-0.5, 0.0) * Vec2::new(50.0, 50.0),
                        EdgeAnchor::Left,
                    ),
                    edge_type: EdgeType::Straight,
                },
                Edge {
                    source_id: 0,
                    target_id: 2,
                    source: (
                        Vec2::new(0.5, 0.0) * Vec2::new(50.0, 50.0),
                        EdgeAnchor::Right,
                    ),
                    target: (
                        Vec2::new(-0.5, 0.0) * Vec2::new(50.0, 50.0),
                        EdgeAnchor::Left,
                    ),
                    edge_type: EdgeType::Curve,
                },
                Edge {
                    source_id: 0,
                    target_id: 3,
                    source: (
                        Vec2::new(0.5, 0.0) * Vec2::new(50.0, 50.0),
                        EdgeAnchor::Right,
                    ),
                    target: (
                        Vec2::new(0.0, -0.5) * Vec2::new(50.0, 50.0),
                        EdgeAnchor::Top,
                    ),
                    edge_type: EdgeType::StraightStep,
                },
                Edge {
                    source_id: 1,
                    target_id: 2,
                    source: (
                        Vec2::new(0.5, 0.0) * Vec2::new(50.0, 50.0),
                        EdgeAnchor::Right,
                    ),
                    target: (
                        Vec2::new(0.0, 0.5) * Vec2::new(50.0, 50.0),
                        EdgeAnchor::Bottom,
                    ),
                    edge_type: EdgeType::SmoothStep,
                },
            ],
            stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
            fill: Color32::from_rgb(50, 100, 150).linear_multiply(0.25),

            tess: Tessellator::default(),
            mesh: false,
        }
    }
}

impl PaintConnections {
    pub fn ui(&mut self, ui: &mut Ui) {
        self.ui_control(ui);

        Frame::canvas(ui.style()).show(ui, |ui| {
            self.ui_content(ui);
        });
    }

    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.label("Mesh shape");
        ui.checkbox(&mut self.mesh, "uses lyon to create mesh");
        ui.end_row();

        ui.collapsing("Colors", |ui| {
            Grid::new("colors")
                .num_columns(2)
                .spacing([12.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Fill color");
                    ui.color_edit_button_srgba(&mut self.fill);
                    ui.end_row();

                    ui.label("Stroke options");
                    ui.add(&mut self.stroke);
                    ui.end_row();
                });
        });

        ui.collapsing("Global tessellation options", |ui| {
            let mut tessellation_options = ui.ctx().tessellation_options(|to| *to);
            tessellation_options.ui(ui);
            ui.ctx()
                .tessellation_options_mut(|to| *to = tessellation_options);
        });

        ui.label("Move the points by dragging them.");
        ui.label("Tessellation `feathering` option doesn't work in lyon.");
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::hover());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let shapes: Vec<Shape> = self
            .shapes
            .iter_mut()
            .enumerate()
            // .take(self.degree)
            .map(|(i, point)| {
                let size = Vec2::splat(50.0);

                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                let point_response = ui.interact(point_rect, point_id, Sense::drag());

                *point += point_response.drag_delta();
                *point = to_screen.from().clamp(*point);

                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let stroke = ui.style().interact(&point_response).fg_stroke;

                Shape::Rect(RectShape::new(
                    point_rect,
                    8.0,
                    self.fill,
                    stroke,
                    StrokeKind::Middle,
                ))
            })
            .collect();

        let tolerance = ui.ctx().tessellation_options(|to| *to).bezier_tolerance;

        let paths = self
            .edges
            .iter_mut()
            .map(|edge| {
                let &mut Edge {
                    source_id,
                    target_id,
                    source,
                    target,
                    edge_type,
                } = edge;

                let source_translation = self.shapes[source_id];
                let target_translation = self.shapes[target_id];

                let (source_offset, source_edge) = source;
                let (target_offset, target_edge) = target;

                let source_pos = to_screen.transform_pos(source_translation + source_offset);
                let target_pos = to_screen.transform_pos(target_translation + target_offset);

                let connection: Connection = EdgePath {
                    source: (glam::Vec2::new(source_pos.x, source_pos.y), source_edge),
                    target: (glam::Vec2::new(target_pos.x, target_pos.y), target_edge),
                    edge_type,
                    ..Default::default()
                }
                .into();

                if self.mesh {
                    connection.build_with(
                        Mode {
                            color: self.stroke.color,
                            options: StrokeOptions::tolerance(tolerance)
                                .with_line_width(self.stroke.width),
                        },
                        &mut self.tess,
                    )
                } else {
                    connection.build(self.stroke)
                }
            })
            .collect::<Vec<_>>();

        painter.extend(paths);
        painter.extend(shapes);

        response
    }
}
