use bevy::{
    camera_controller::pan_camera::{PanCamera, PanCameraPlugin},
    color::palettes::{
        css::{PINK, WHITE},
        tailwind::{
            BLUE_500, GRAY_500, STONE_50, STONE_500, STONE_600, YELLOW_50, YELLOW_500, YELLOW_600,
        },
    },
    platform::collections::HashMap,
    prelude::*,
};

use bevy_flowkit::*;
use bevy_prototype_lyon::{
    draw::Stroke,
    entity::Shape,
    plugin::ShapePlugin,
    prelude::{LineCap, LineJoin, ShapeBuilder, ShapeBuilderBase, StrokeOptions},
};
use petgraph::{Graph, visit::EdgeRef};

use bevy_moon::{
    MoonPlugin,
    elements::{image::ImageNode, node::Node, text::Text},
    style::{BorderColor, BoxShadow, Corners, Style},
};

mod utils;

pub use utils::*;

#[derive(Component, Clone, Debug)]
pub struct Edge {
    pub source_position: EdgePoint,
    pub target_position: EdgePoint,
    pub edge_type: EdgeType,
    pub color: Color,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            source_position: (Vec2::ZERO, EdgeAnchor::Top),
            target_position: (Vec2::ZERO, EdgeAnchor::Bottom),
            edge_type: EdgeType::Straight,
            color: Color::oklch(0.92, 0.0, 0.0),
        }
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct EdgeId {
    pub source: u32,
    pub target: u32,
}

#[derive(Resource, Debug, Default)]
pub struct FlowGraph {
    pub graph: Graph<u32, Edge>,
}

impl AsRef<Graph<u32, Edge>> for FlowGraph {
    fn as_ref(&self) -> &Graph<u32, Edge> {
        &self.graph
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut flow_graph: ResMut<FlowGraph>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2d, PanCamera::default()));

    let mesh_node_0 = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(100.0, 100.0)))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from(Color::oklcha(0.81, 0.1, 251., 0.99))),
            ),
            Transform::from_xyz(-300.0, -100.0, 0.0).with_rotation(Quat::from_rotation_z(
                // 30.0
                0.0,
            )),
        ))
        .observe(on_drag_and_drop)
        .id();

    let mesh_node_1 = commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(25.0))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from(Color::oklcha(0.87, 0.15, 154., 1.0))),
            ),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .observe(on_drag_and_drop)
        .id();

    let mesh_node_2 = commands
        .spawn((
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(50.0, 0.0),
                Vec2::new(25.0, 50.0),
            ))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from(Color::oklcha(0.62, 0.21, 259.0, 1.0))),
            ),
            Transform::from_xyz(300.0, 100.0, 0.0),
        ))
        .observe(on_drag_and_drop)
        .id();

    let mesh_node_3 = commands
        .spawn((
            Mesh2d(meshes.add(Rhombus::new(80.0, 100.0))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from(Color::oklcha(0.81, 0.11, 19.5, 1.0))),
            ),
            Transform::from_xyz(300.0, 100.0, 0.0),
        ))
        .observe(on_drag_and_drop)
        .id();

    let text_2d_node_0 = commands
        .spawn((
            Text2d::new("Text 2d"),
            TextFont::default(),
            TextColor(Color::oklch(0.14, 0.0, 0.0)),
            TextBackgroundColor(Color::oklch(0.92, 0.0, 0.0)),
            Transform::from_xyz(100.0, 100.0, 0.0),
        ))
        .observe(on_drag_and_drop)
        .id();

    let node_0 = commands
        .spawn((
            Node,
            Transform::from_xyz(-300.0, 100.0, 10.0),
            Style::from(taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(100.0),
                    height: taffy::Dimension::length(100.0),
                },
                align_items: Some(taffy::AlignItems::Center),
                justify_content: Some(taffy::JustifyContent::Center),
                border: taffy::Rect::length(10.0),
                ..default()
            })
            .corner_radii(Corners::all(14.0))
            .border_color(BorderColor::all(Color::oklch(0.92, 0.0, 0.0)))
            .box_shadow(BoxShadow::DEFAULT)
            .background(Color::WHITE),
            children![(
                Text::new("Node"),
                TextFont::default(),
                TextColor(Color::oklch(0.14, 0.0, 0.0)),
            )],
        ))
        .observe(on_drag_and_drop)
        .id();

    let node_1 = commands
        .spawn((
            Node,
            Transform::from_xyz(300.0, 0.0, 0.0),
            Style::from(taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(100.0),
                    height: taffy::Dimension::length(100.0),
                },
                align_items: Some(taffy::AlignItems::Center),
                justify_content: Some(taffy::JustifyContent::Center),
                border: taffy::Rect::length(1.0),
                ..default()
            })
            .corner_radii(Corners::all(14.0))
            .border_color(BorderColor::all(Color::oklch(0.92, 0.0, 0.0)))
            .box_shadow(BoxShadow {
                color: Color::oklcha(0.0, 0.0, 0.0, 0.05),
                offset: Vec2::new(0.0, 1.0),
                spread_radius: 1.0,
                blur_radius: 2.0,
            })
            .background(Color::WHITE),
            children![(
                Text::new("Node"),
                TextFont::default(),
                TextColor(Color::oklch(0.14, 0.0, 0.0)),
            )],
        ))
        .observe(on_drag_and_drop)
        .id();

    let node_2 = commands
        .spawn((
            Node,
            Transform::from_xyz(300.0, 300.0, 0.0),
            Style::from(taffy::Style {
                size: taffy::Size {
                    width: taffy::Dimension::length(100.0),
                    height: taffy::Dimension::length(100.0),
                },
                align_items: Some(taffy::AlignItems::Center),
                justify_content: Some(taffy::JustifyContent::Center),
                border: taffy::Rect::length(1.0),
                ..default()
            })
            .corner_radii(Corners::all(14.0))
            .border_color(BorderColor::all(Color::oklch(0.92, 0.0, 0.0)))
            .box_shadow(BoxShadow {
                color: Color::oklcha(0.0, 0.0, 0.0, 0.05),
                offset: Vec2::new(0.0, 1.0),
                spread_radius: 1.0,
                blur_radius: 2.0,
            })
            .background(Color::WHITE),
            children![(
                Text::new("Node"),
                TextFont::default(),
                TextColor(Color::oklch(0.14, 0.0, 0.0)),
            )],
        ))
        .observe(on_drag_and_drop)
        .id();

    let node_3 = commands
        .spawn((
            Node,
            Transform::from_xyz(100.0, -300.0, 0.0),
            Style::from(taffy::Style {
                size: taffy::Size::length(100.0),
                border: taffy::Rect::length(2.0),
                align_items: Some(taffy::AlignItems::Center),
                justify_content: Some(taffy::JustifyContent::Center),
                ..default()
            })
            .background(YELLOW_500)
            .border_color(BorderColor::all(YELLOW_50))
            .corner_radii(Corners::all(25.0))
            .box_shadow(BoxShadow::default()),
            children![(
                Node,
                Style::from(taffy::Style {
                    size: taffy::Size::length(50.0),
                    border: taffy::Rect::length(2.0),
                    ..default()
                })
                .background(YELLOW_600)
                .border_color(BorderColor::all(YELLOW_50))
                .corner_radii(Corners::all(25.0))
                .box_shadow(BoxShadow::default()),
            )],
        ))
        .observe(on_drag_and_drop)
        .id();

    let node_4 = commands
        .spawn((
            Node,
            Transform::from_xyz(100.0, 130.0, 0.0),
            Style::from(taffy::Style {
                size: taffy::Size::length(100.0),
                border: taffy::Rect::length(2.0),
                align_items: Some(taffy::AlignItems::Center),
                justify_content: Some(taffy::JustifyContent::End),
                ..default()
            })
            .background(STONE_500)
            .border_color(BorderColor::all(STONE_50))
            .corner_radii(Corners::all(25.0))
            .box_shadow(BoxShadow::default()),
            children![(
                Node,
                Style::from(taffy::Style {
                    size: taffy::Size::length(50.0),
                    border: taffy::Rect::length(2.0),
                    ..default()
                })
                .background(STONE_600)
                .border_color(BorderColor::all(STONE_50))
                .corner_radii(Corners::all(25.0))
                .box_shadow(BoxShadow::default()),
            )],
        ))
        .observe(on_drag_and_drop)
        .id();

    let node_5 = commands
        .spawn((
            Pickable::default(),
            Node,
            Style::from(taffy::Style {
                align_content: Some(taffy::AlignContent::FlexStart),
                size: taffy::Size::from_lengths(150.0, 150.0),
                padding: taffy::Rect::length(10.0),
                ..default()
            })
            .background(WHITE)
            .box_shadow(BoxShadow::default())
            .border_color(BorderColor::all(PINK))
            .corner_radii(Corners::all(25.0)),
            Transform::from_xyz(0.0, 0.0, 0.0),
            children![(
                Style::from(taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::percent(1.0),
                        height: taffy::Dimension::auto(),
                    },
                    ..Default::default()
                }),
                ImageNode::new(asset_server.load("images/bevy.png"))
            ),],
        ))
        .observe(on_drag_and_drop)
        .id();

    let graph = &mut flow_graph.graph;

    let mesh_node_id_0 = graph.add_node(mesh_node_0.index_u32());
    let mesh_node_id_1 = graph.add_node(mesh_node_1.index_u32());
    let mesh_node_id_2 = graph.add_node(mesh_node_2.index_u32());
    let mesh_node_id_3 = graph.add_node(mesh_node_3.index_u32());

    let text_2d_node_id_0 = graph.add_node(text_2d_node_0.index_u32());

    let node_id_0 = graph.add_node(node_0.index_u32());
    let node_id_1 = graph.add_node(node_1.index_u32());
    let node_id_2 = graph.add_node(node_2.index_u32());
    let node_id_3 = graph.add_node(node_3.index_u32());
    let node_id_4 = graph.add_node(node_4.index_u32());
    let node_id_5 = graph.add_node(node_5.index_u32());

    graph.add_edge(
        mesh_node_id_0,
        mesh_node_id_1,
        Edge {
            source_position: (
                Vec2::new(0.0, -0.5) * Vec2::new(100.0, 100.0),
                EdgeAnchor::Bottom,
            ),
            target_position: (
                Vec2::new(0.0, -0.5) * Vec2::new(25.0, 25.0),
                EdgeAnchor::Bottom,
            ),
            edge_type: EdgeType::Straight,
            ..default()
        },
    );

    graph.add_edge(
        mesh_node_id_2,
        mesh_node_id_0,
        Edge {
            source_position: (Vec2::new(0.5, 1.0) * Vec2::new(50.0, 25.0), EdgeAnchor::Top),
            // source_position: (
            //     Vec2::new(0.625, 0.5) * Vec2::new(50.0, 25.0),
            //     EdgeAnchor::Right,
            // ),
            // source_position: (
            //     Vec2::new(0.125, 0.5) * Vec2::new(50.0, 25.0),
            //     EdgeAnchor::Left,
            // ),
            // source_position: (
            //     Vec2::new(0.5, 0.0) * Vec2::new(50.0, 25.0),
            //     EdgeAnchor::Bottom,
            // ),
            target_position: (
                Vec2::new(0.5, 0.25) * Vec2::new(100.0, 100.0),
                EdgeAnchor::Right,
            ),
            edge_type: EdgeType::StraightStep,
            ..default()
        },
    );

    graph.add_edge(
        mesh_node_id_0,
        mesh_node_id_3,
        Edge {
            source_position: (
                Vec2::new(0.5, 0.25) * Vec2::new(100.0, 100.0),
                EdgeAnchor::Right,
            ),
            // target_position: (
            //     Vec2::new(0.0, 0.5) * Vec2::new(80.0, 100.0),
            //     EdgeAnchor::Top,
            // ),
            // target_position: (
            //     Vec2::new(0.5, 0.0) * Vec2::new(80.0, 100.0),
            //     EdgeAnchor::Right,
            // ),
            // target_position: (
            //     Vec2::new(-0.5, 0.0) * Vec2::new(80.0, 100.0),
            //     EdgeAnchor::Left,
            // ),
            target_position: (
                Vec2::new(0.0, -0.5) * Vec2::new(80.0, 100.0),
                EdgeAnchor::Bottom,
            ),
            edge_type: EdgeType::StraightStep,
            ..default()
        },
    );

    graph.add_edge(
        mesh_node_id_0,
        text_2d_node_id_0,
        Edge {
            source_position: (
                Vec2::new(0.0, 0.5) * Vec2::new(100.0, 100.0),
                EdgeAnchor::Top,
            ),
            target_position: (Vec2::new(0.0, 0.0), EdgeAnchor::Left),
            edge_type: EdgeType::StraightStep,
            ..default()
        },
    );

    graph.add_edge(
        node_id_0,
        mesh_node_id_0,
        Edge {
            source_position: (
                Vec2::new(0.5, 0.0) * Vec2::new(100.0, 100.0),
                EdgeAnchor::Right,
            ),
            target_position: (
                Vec2::new(-0.5, 0.0) * Vec2::new(100.0, 100.0),
                EdgeAnchor::Left,
            ),
            edge_type: EdgeType::SmoothStep,
            ..default()
        },
    );

    graph.extend_with_edges(&[
        (
            node_id_0,
            node_id_1,
            Edge {
                source_position: (
                    Vec2::new(0.0, 0.5) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Top,
                ),
                target_position: (
                    Vec2::new(-0.5, 0.0) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Left,
                ),
                edge_type: EdgeType::SmoothStep,
                ..default()
            },
        ),
        (
            node_id_1,
            node_id_2,
            Edge {
                source_position: (
                    Vec2::new(0.0, 0.5) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Top,
                ),
                target_position: (
                    Vec2::new(0.0, -0.5) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Bottom,
                ),
                edge_type: EdgeType::Curve,
                ..default()
            },
        ),
        (
            node_id_2,
            node_id_0,
            Edge {
                source_position: (
                    Vec2::new(0.0, -0.5) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Bottom,
                ),
                target_position: (
                    Vec2::new(-0.5, 0.0) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Left,
                ),
                edge_type: EdgeType::StraightStep,
                ..default()
            },
        ),
        (
            node_id_0,
            node_id_3,
            Edge {
                source_position: (
                    Vec2::new(0.5, 0.0) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Right,
                ),
                target_position: (
                    Vec2::new(-0.5, 0.0) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Left,
                ),
                edge_type: EdgeType::SmoothStep,
                color: YELLOW_500.with_alpha(0.5).into(),
            },
        ),
        (
            node_id_4,
            node_id_3,
            Edge {
                source_position: (
                    Vec2::new(-0.5, 0.25) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Left,
                ),
                target_position: (
                    Vec2::new(0.5, -0.25) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Right,
                ),
                edge_type: EdgeType::SmoothStep,
                color: STONE_500.with_alpha(0.5).into(),
            },
        ),
        (
            node_id_3,
            node_id_5,
            Edge {
                source_position: (
                    Vec2::new(0.0, -0.5) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Bottom,
                ),
                target_position: (
                    Vec2::new(0.5, -0.25) * Vec2::new(150.0, 150.0),
                    EdgeAnchor::Right,
                ),
                edge_type: EdgeType::Curve,
                color: GRAY_500.with_alpha(0.5).into(),
            },
        ),
        (
            node_id_4,
            node_id_5,
            Edge {
                source_position: (
                    Vec2::new(0.5, 0.0) * Vec2::new(100.0, 100.0),
                    EdgeAnchor::Right,
                ),
                target_position: (
                    Vec2::new(-0.5, -0.25) * Vec2::new(150.0, 150.0),
                    EdgeAnchor::Left,
                ),
                edge_type: EdgeType::SmoothStep,
                color: BLUE_500.with_alpha(0.5).into(),
            },
        ),
    ]);
}

fn draw_edges(
    mut commands: Commands,
    nodes: Query<&Transform, Or<(With<Node>, With<Mesh2d>, With<Text2d>)>>,
    changed_nodes: Query<
        Entity,
        (
            Changed<Transform>,
            Or<(With<Node>, With<Mesh2d>, With<Text2d>)>,
        ),
    >,
    mut connections: Query<(&mut Shape, &EdgeId)>,
    flow_graph: Res<FlowGraph>,
) -> Result {
    if changed_nodes.is_empty() {
        return Ok(());
    }

    let mut stroke = Stroke::default();
    stroke.options = StrokeOptions::default()
        .with_line_width(2.0)
        .with_line_join(LineJoin::Round)
        .with_line_cap(LineCap::Round);

    let graph = flow_graph.into_inner().as_ref();

    let edges = graph
        .edge_references()
        .filter_map(|edge| {
            let (Some(source), Some(target)) = (
                graph.node_weight(edge.source()).copied(),
                graph.node_weight(edge.target()).copied(),
            ) else {
                return None;
            };
            let (Some(source_transform), Some(target_transform)) = (
                Entity::from_raw_u32(source).and_then(|id| nodes.get(id).ok()),
                Entity::from_raw_u32(target).and_then(|id| nodes.get(id).ok()),
            ) else {
                return None;
            };

            let edge_id = EdgeId { source, target };

            Some((edge_id, (edge.weight(), source_transform, target_transform)))
        })
        .collect::<Vec<_>>();

    let mut changed_edges = HashMap::with_capacity(changed_nodes.count());

    for entity in changed_nodes {
        let entity_id = entity.index_u32();

        for (edge_id, (edge_weight, source_transform, target_transform)) in &edges {
            if edge_id.source != entity_id && edge_id.target != entity_id {
                continue;
            }

            changed_edges.insert(edge_id, (edge_weight, source_transform, target_transform));
        }
    }

    for (i, (&edge_id, (edge_weight, source_transform, target_transform))) in
        changed_edges.iter().enumerate()
    {
        let &&Edge {
            source_position,
            target_position,
            edge_type,
            color,
        } = *edge_weight;

        let (source_offset, source_edge_pos) = source_position;
        let (target_offset, target_edge_pos) = target_position;

        let source_pos = source_transform.translation.truncate() + source_offset;
        let target_pos = target_transform.translation.truncate() + target_offset;

        let connection: Connection = EdgePath {
            source: (source_pos, source_edge_pos),
            target: (target_pos, target_edge_pos),
            edge_type,
            ..default()
        }
        .into();

        let shape = ShapeBuilder::with(&connection)
            .stroke(Stroke { color, ..stroke })
            .build();

        if let Some((mut current_shape, _)) = connections.iter_mut().find(|(_, id)| *id == edge_id)
        {
            *current_shape = shape;
        } else {
            commands.spawn((
                shape,
                *edge_id,
                Transform::from_xyz(0.0, 0.0, -(1000.0 - i as f32) / 1000.0),
            ));
        }
    }

    Ok(())
}

fn toggle_pancam(mut query: Query<&mut PanCamera>, keys: Res<ButtonInput<KeyCode>>) {
    // Toggle Panning with Spacebar
    if keys.just_pressed(KeyCode::Space) {
        for mut pancam in &mut query {
            pancam.enabled = true;
        }
        return;
    }
    if keys.just_released(KeyCode::Space) {
        for mut pancam in &mut query {
            pancam.enabled = false;
        }
        return;
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        MeshPickingPlugin,
        PanCameraPlugin,
        ShapePlugin,
        MoonPlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, toggle_pancam)
    .add_systems(Update, draw_edges.after(TransformSystems::Propagate))
    .insert_resource(FlowGraph::default())
    .insert_resource(ClearColor(Color::oklch(0.98, 0.0, 0.0)));

    app.run();
}
