//! Contemporary art-led menus; existing navigation remains authoritative.
use super::*;
pub(super) const PAPER: Color = Color::srgb(0.94, 0.93, 0.90);
pub(super) const ACCENT: Color = Color::srgb(0.96, 0.40, 0.22);
#[derive(Component)]
pub(super) struct MenuChoice;
fn rect(page: Page, index: usize, count: usize) -> [f32; 4] {
    if page == Page::Main {
        [5., 29. + index as f32 * 12., 39., 10.]
    } else {
        let width = 90. / count.max(1) as f32;
        [5. + index as f32 * width, 78., width - 2., 12.]
    }
}
fn node([x, y, w, h]: [f32; 4]) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: percent(x),
        top: percent(y),
        width: percent(w),
        height: percent(h),
        ..default()
    }
}
pub(super) fn render(
    commands: &mut Commands,
    canvas: Entity,
    page: Page,
    title: &str,
    body: &str,
    loading: bool,
) {
    // Native continuous gradients: no hard bars or banded stacks of rectangles.
    let shade = |alpha| Color::srgba(0.012, 0.012, 0.02, alpha);
    let mut gradients = vec![
        LinearGradient {
            angle: LinearGradient::TO_BOTTOM,
            stops: vec![
                ColorStop::new(shade(0.6), percent(0.)),
                ColorStop::new(shade(0.), percent(38.)),
            ],
            ..default()
        },
        LinearGradient {
            angle: LinearGradient::TO_BOTTOM,
            stops: vec![
                ColorStop::new(shade(0.), percent(40.)),
                ColorStop::new(shade(0.88), percent(100.)),
            ],
            ..default()
        },
    ];
    if page == Page::Main {
        gradients.push(LinearGradient {
            angle: LinearGradient::TO_LEFT,
            stops: vec![
                ColorStop::new(shade(0.), percent(0.)),
                ColorStop::new(shade(0.12), percent(40.)),
                ColorStop::new(shade(0.86), percent(100.)),
            ],
            ..default()
        });
    }
    let scrim = commands
        .spawn((
            node([0., 0., 100., 100.]),
            BackgroundGradient(gradients.into_iter().map(Gradient::Linear).collect()),
        ))
        .id();
    commands.entity(canvas).add_child(scrim);
    let heading = text(
        commands,
        title,
        if page == Page::Main || page == Page::Splash {
            72.
        } else {
            48.
        },
        Node {
            position_type: PositionType::Absolute,
            left: percent(5.),
            top: percent(7.),
            ..default()
        },
        PAPER,
    );
    commands.entity(canvas).add_child(heading);
    let eyebrow = text(
        commands,
        if matches!(page, Page::Main | Page::Splash) {
            "MOTORCYCLE COMBAT RACING"
        } else {
            "ROAD / DASH"
        },
        16.,
        Node {
            position_type: PositionType::Absolute,
            left: percent(5.),
            top: percent(3.5),
            ..default()
        },
        ACCENT,
    );
    commands.entity(canvas).add_child(eyebrow);
    let info = text(
        commands,
        body,
        22.,
        Node {
            position_type: PositionType::Absolute,
            left: percent(5.),
            top: percent(if page == Page::Main { 85. } else { 56. }),
            width: percent(if page == Page::Main { 48. } else { 85. }),
            ..default()
        },
        PAPER.with_alpha(0.78),
    );
    commands.entity(canvas).add_child(info);
    let rows = entries(page);
    for (index, entry) in rows.iter().enumerate() {
        let mut n = node(rect(page, index, rows.len()));
        n.align_items = AlignItems::Center;
        n.justify_content = JustifyContent::FlexStart;
        n.border = if page == Page::Main {
            UiRect::left(px(3.))
        } else {
            UiRect::top(px(1.))
        };
        n.padding = UiRect::axes(px(22.), px(8.));
        let button = commands
            .spawn((
                n,
                BackgroundColor(Color::NONE),
                BorderColor::all(Color::srgba(1., 1., 1., 0.12)),
            ))
            .id();
        if let Some(action) = entry.action.filter(|_| !loading) {
            commands
                .entity(button)
                .insert((Button, MenuButton { index, action }, MenuChoice));
        }
        let label = text(
            commands,
            &entry.text,
            if page == Page::Main { 34. } else { 28. },
            Node::default(),
            if entry.action.is_some() && !loading {
                PAPER
            } else {
                PAPER.with_alpha(0.30)
            },
        );
        commands.entity(button).add_child(label);
        commands.entity(canvas).add_child(button);
    }
    let hint = text(
        commands,
        "ENTER  Select       ESC  Back / Quit       F2  Settings",
        16.,
        Node {
            position_type: PositionType::Absolute,
            left: percent(5.),
            bottom: percent(3.),
            ..default()
        },
        PAPER.with_alpha(0.6),
    );
    commands.entity(canvas).add_child(hint);
}
pub(super) fn button(
    commands: &mut Commands,
    canvas: Entity,
    index: usize,
    entry: &Entry,
    area: [f32; 4],
    loading: bool,
) {
    let mut n = node(area);
    n.align_items = AlignItems::Center;
    n.padding = UiRect::axes(px(18.), px(8.));
    n.border = UiRect::left(px(3.));
    let button = commands
        .spawn((
            n,
            BackgroundColor(Color::srgba(0.015, 0.012, 0.02, 0.94)),
            BorderColor::all(Color::NONE),
        ))
        .id();
    if let Some(action) = entry.action.filter(|_| !loading) {
        commands
            .entity(button)
            .insert((Button, MenuButton { index, action }));
    }
    let label = text(
        commands,
        &entry.text,
        if entry.text.len() > 38 { 18. } else { 24. },
        Node::default(),
        if entry.action.is_some() {
            PAPER
        } else {
            PAPER.with_alpha(0.45)
        },
    );
    commands.entity(button).add_child(label);
    commands.entity(canvas).add_child(button);
}
pub(super) fn render_online(
    commands: &mut Commands,
    canvas: Entity,
    heading: &str,
    body: &str,
    rows: &[Entry],
) {
    let panel = commands
        .spawn((
            node([3., 3., 94., 94.]),
            BackgroundColor(Color::srgba(0.014, 0.018, 0.025, 0.88)),
        ))
        .id();
    commands.entity(canvas).add_child(panel);
    for (value, size, area, color) in [
        (
            "ROAD / DASH   /   MULTIPLAYER",
            16.,
            [6., 6., 87., 4.],
            ACCENT,
        ),
        (heading, 48., [6., 12., 87., 10.], PAPER),
        (body, 23., [6., 28., 47., 37.], PAPER),
    ] {
        let label = text(commands, value, size, node(area), color);
        commands.entity(canvas).add_child(label);
    }
    for (i, entry) in rows.iter().enumerate() {
        button(
            commands,
            canvas,
            i,
            entry,
            [58., 28. + i as f32 * 13., 35., 10.],
            false,
        );
    }
    let hint = text(
        commands,
        "ARROWS / D-PAD  Navigate     ENTER / A  Select     ESC / B  Back",
        16.,
        node([6., 88., 87., 5.]),
        PAPER.with_alpha(0.65),
    );
    commands.entity(canvas).add_child(hint);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn available_actions_fit_safe_area_without_overlapping() {
        for page in [
            Page::Main,
            Page::Street,
            Page::Courses,
            Page::Results,
            Page::Quit,
        ] {
            let rows = entries(page);
            let mut areas: Vec<[f32; 4]> = Vec::new();
            for i in 0..rows.len() {
                let [x, y, w, h] = rect(page, i, rows.len());
                assert!(x >= 5. && y >= 20. && x + w <= 95. && y + h <= 90.);
                for [a, b, c, d] in &areas {
                    assert!(x + w <= *a || x >= a + c || y + h <= *b || y >= b + d);
                }
                areas.push([x, y, w, h]);
            }
        }
    }
}
