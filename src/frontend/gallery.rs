//! Art-led 4:3 portrait pages with live text and the shared menu edge owner.
use super::*;
const INK: Color = Color::srgb(0.02, 0.012, 0.025);
const PAPER: Color = Color::srgb(1.0, 0.92, 0.69);
const GOLD: Color = Color::srgb(1.0, 0.74, 0.12);

fn back_button(commands: &mut Commands, canvas: Entity, index: usize, caption: &str) {
    let button = commands
        .spawn((
            Button,
            MenuButton {
                index,
                action: Action::Back,
            },
            Node {
                position_type: PositionType::Absolute,
                right: percent(2),
                bottom: percent(1.2),
                width: percent(22),
                height: percent(5.2),
                border: UiRect::all(px(2)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(INK),
            BorderColor::all(GOLD),
        ))
        .id();
    let label = text(commands, caption, 22.0, Node::default(), PAPER);
    commands.entity(button).add_child(label);
    commands.entity(canvas).add_child(button);
}

pub(super) fn render(commands: &mut Commands, canvas: Entity, portraits: &[Handle<Image>]) {
    let wash = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                ..default()
            },
            BackgroundColor(Color::srgba(0.015, 0.0, 0.025, 0.72)),
        ))
        .id();
    commands.entity(canvas).add_child(wash);
    let title = text(
        commands,
        "CHOOSE YOUR RIDER",
        42.0,
        Node {
            position_type: PositionType::Absolute,
            left: percent(3),
            top: percent(1),
            ..default()
        },
        GOLD,
    );
    commands.entity(canvas).add_child(title);
    let grid = commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            left: percent(3),
            top: percent(8),
            width: percent(94),
            height: percent(83),
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.0); 4],
            grid_template_rows: vec![GridTrack::flex(1.0); 4],
            column_gap: px(8),
            row_gap: px(8),
            ..default()
        },))
        .id();
    for (index, rider) in ROSTER.iter().enumerate() {
        let card = commands
            .spawn((
                Button,
                MenuButton {
                    index,
                    action: Action::SelectPortrait(index),
                },
                Node {
                    min_width: px(0),
                    min_height: px(0),
                    border: UiRect::all(px(3)),
                    overflow: Overflow::clip(),
                    ..default()
                },
                BackgroundColor(INK),
                BorderColor::all(Color::srgb(0.29, 0.22, 0.25)),
            ))
            .id();
        let portrait = commands
            .spawn((
                ImageNode::new(portraits[index].clone()),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(6),
                    width: percent(88),
                    height: percent(100),
                    ..default()
                },
            ))
            .id();
        let tag = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: percent(0),
                    width: percent(100),
                    min_height: percent(20),
                    padding: UiRect::axes(px(8), px(4)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.015, 0.0, 0.025, 0.92)),
            ))
            .id();
        let label = text(commands, rider.name, 22.0, Node::default(), PAPER);
        commands.entity(tag).add_child(label);
        commands.entity(card).add_children(&[portrait, tag]);
        commands.entity(grid).add_child(card);
    }
    commands.entity(canvas).add_child(grid);
    let hint = text(
        commands,
        "CLICK A PORTRAIT / ENTER TO MEET THE RIDER",
        17.0,
        Node {
            position_type: PositionType::Absolute,
            left: percent(3),
            bottom: percent(2.2),
            ..default()
        },
        PAPER,
    );
    commands.entity(canvas).add_child(hint);
    back_button(commands, canvas, ROSTER.len(), "BACK");
}

/// The portrait itself fills the entire 4:3 page; all copy stays in the lower strip.
pub(super) fn detail(commands: &mut Commands, canvas: Entity, selected: usize) {
    let rider = &ROSTER[selected];
    let band = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: percent(0),
                width: percent(100),
                height: percent(26),
                border: UiRect::top(px(3)),
                padding: UiRect::axes(px(28), px(12)),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                ..default()
            },
            BackgroundColor(Color::srgba(0.015, 0.0, 0.025, 0.94)),
            BorderColor::all(Color::srgb(0.45, 0.07, 0.04)),
        ))
        .id();
    let name = text(
        commands,
        rider.name.to_uppercase(),
        42.0,
        Node::default(),
        GOLD,
    );
    let bio = text(
        commands,
        rider.bio,
        24.0,
        Node {
            width: percent(100),
            ..default()
        },
        PAPER,
    );
    commands.entity(band).add_children(&[name, bio]);
    commands.entity(canvas).add_child(band);
    let hint = text(
        commands,
        format!("{:02} / 16    ESC / EAST: BACK TO RIDERS", selected + 1),
        16.0,
        Node {
            position_type: PositionType::Absolute,
            left: percent(2),
            bottom: percent(2.2),
            ..default()
        },
        PAPER,
    );
    commands.entity(canvas).add_child(hint);
    back_button(commands, canvas, 0, "BACK TO RIDERS");
}
