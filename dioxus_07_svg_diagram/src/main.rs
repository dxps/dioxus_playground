use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
struct Node {
    id: u32,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    label: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Edge {
    from: u32,
    to: u32,
    label: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct Diagram {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Handle {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl Handle {
    fn cursor(self) -> &'static str {
        match self {
            Handle::N | Handle::S => "ns-resize",
            Handle::E | Handle::W => "ew-resize",
            Handle::NE | Handle::SW => "nesw-resize",
            Handle::NW | Handle::SE => "nwse-resize",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Interaction {
    Drag {
        id: u32,
        ox: f64,
        oy: f64,
    },
    Resize {
        id: u32,
        handle: Handle,
        sx: f64,
        sy: f64,
        start: Node,
    },
}

#[derive(Clone, Debug, PartialEq)]
struct Guides {
    lines: Vec<GuideLine>,
}

#[derive(Clone, Debug, PartialEq)]
struct GuideLine {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let mut diagram = use_signal(|| Diagram {
        nodes: vec![
            Node {
                id: 1,
                x: 80.0,
                y: 80.0,
                w: 170.0,
                h: 80.0,
                label: "Start".into(),
            },
            Node {
                id: 2,
                x: 360.0,
                y: 220.0,
                w: 220.0,
                h: 90.0,
                label: "Process".into(),
            },
            Node {
                id: 3,
                x: 360.0,
                y: 80.0,
                w: 220.0,
                h: 90.0,
                label: "Align target".into(),
            },
        ],
        edges: vec![
            Edge {
                from: 1,
                to: 2,
                label: Some("go".into()),
            },
            Edge {
                from: 1,
                to: 3,
                label: Some("up".into()),
            },
        ],
    });

    let mut interaction = use_signal(|| Option::<Interaction>::None);
    let mut show_guides = use_signal(|| Guides { lines: vec![] });
    let mut show_grid = use_signal(|| false);
    let mut orthogonal_edges = use_signal(|| false);

    // Viewbox
    let view_w = 900.0;
    let view_h = 600.0;

    // Snapping config
    const GRID: f64 = 10.0;
    const GRID_THRESH: f64 = 5.0; // distance to snap to grid
    const ALIGN_THRESH: f64 = 6.0; // distance to snap to other nodes
    const MIN_W: f64 = 60.0;
    const MIN_H: f64 = 40.0;

    const CSS: &str = r#"
        .handles { opacity: 0; pointer-events: none; }
        .node:hover .handles { opacity: 1; pointer-events: all; }
        "#;

    rsx! {
        div { style: "font-family: system-ui; padding: 12px;",

            h3 { "Dioxus 0.7 SVG diagram (drag, 8-handle resize, snapping, guides)" }

            h4 {
                label {
                    input {
                        r#type: "checkbox",
                        checked: "{show_grid()}",
                        onchange: move |_| show_grid.set(!show_grid()),
                    }
                    " Show grid"
                }
            }

            h4 {
                label {
                    input {
                        r#type: "checkbox",
                        checked: "{orthogonal_edges()}",
                        onchange: move |_| orthogonal_edges.set(!orthogonal_edges()),
                    }
                    " Orthogonal edges"
                }
            }

            svg {
                width: "{view_w}",
                height: "{view_h}",
                view_box: "0 0 {view_w} {view_h}",
                style: "border: 1px solid #ccc; background: white; touch-action: none; user-select: none;",

                // Pointer move: handles drag + resize and snapping + guides.
                onpointermove: move |evt| {
                    let (mx, my) = element_xy(&evt);

                    let Some(int) = interaction() else {
                        // No interaction: clear guides (optional).
                        if !show_guides().lines.is_empty() {
                            show_guides.set(Guides { lines: vec![] });
                        }
                        return;
                    };

                    // Snapshot other nodes for snapping/alignment (exclude active node).
                    let (active_id, others) = {
                        let d = diagram();
                        let active_id = match &int {
                            Interaction::Drag { id, .. } => *id,
                            Interaction::Resize { id, .. } => *id,
                        };
                        let others = d

                            .nodes

                            // Compute proposed rect from start + delta + handle.

                            .iter()
                            .filter(|n| n.id != active_id)
                            .cloned()
                            .collect::<Vec<_>>();
                        (active_id, others)
                    };
                    match int {
                        Interaction::Drag { id, ox, oy } => {
                            let proposed = {
                                let d = diagram();
                                let mut n = d.nodes.iter().find(|n| n.id == id).cloned().unwrap();
                                n.x = mx - ox;
                                n.y = my - oy;
                                n
                            };
                            let (snapped, g) = snap_move(
                                proposed,
                                &others,
                                GRID,
                                GRID_THRESH,
                                ALIGN_THRESH,
                            );
                            show_guides.set(g);
                            diagram
                                .with_mut(|d| {
                                    if let Some(n) = d.nodes.iter_mut().find(|n| n.id == active_id) {
                                        n.x = snapped.x;
                                        n.y = snapped.y;
                                    }
                                });
                        }
                        Interaction::Resize { id, handle, sx, sy, start } => {
                            let dx = mx - sx;
                            let dy = my - sy;
                            let mut proposed = start.clone();
                            apply_resize_delta(&mut proposed, handle, dx, dy, MIN_W, MIN_H);
                            let (snapped, g) = snap_resize(
                                proposed,
                                &start,
                                handle,
                                &others,
                                GRID,
                                GRID_THRESH,
                                ALIGN_THRESH,
                                MIN_W,
                                MIN_H,
                            );
                            show_guides.set(g);
                            diagram
                                .with_mut(|d| {
                                    if let Some(n) = d.nodes.iter_mut().find(|n| n.id == active_id) {
                                        *n = snapped;
                                    }
                                });
                        }
                    }
                },

                // Finish interaction
                onpointerup: move |_| {
                    interaction.set(None);
                    show_guides.set(Guides { lines: vec![] });
                },
                onpointercancel: move |_| {
                    interaction.set(None);
                    show_guides.set(Guides { lines: vec![] });
                },

                style { "{CSS}" }

                defs {
                    marker {
                        id: "arrow",
                        view_box: "0 0 10 10",
                        ref_x: "9",
                        ref_y: "5",
                        marker_width: "8",
                        marker_height: "8",
                        orient: "auto",
                        path { d: "M 0 0 L 10 5 L 0 10 z", fill: "#333" }
                    }
                }

                // Optional: faint grid (helps visualize snapping)
                if show_grid() {
                    Grid { w: view_w, h: view_h, step: GRID }
                }

                // Guides (alignment lines), on top of grid, under edges/nodes
                for (i , gl) in show_guides().lines.iter().enumerate() {
                    line {
                        key: "{i}",
                        x1: "{gl.x1}",
                        y1: "{gl.y1}",
                        x2: "{gl.x2}",
                        y2: "{gl.y2}",
                        stroke: "#2b7cff",
                        stroke_width: "1.5",
                        stroke_dasharray: "5 4",
                        pointer_events: "none",
                    }
                }

                // Edges (under nodes)
                for e in diagram().edges.clone() {
                    EdgeView {
                        diagram: diagram(),
                        edge: e,
                        orthogonal: orthogonal_edges(),
                    }
                }

                // Nodes (over edges)
                for n in diagram().nodes.clone() {
                    NodeView {
                        key: "{n.id}",
                        node: n,
                        on_drag_start: move |(id, ox, oy)| interaction.set(Some(Interaction::Drag { id, ox, oy })),
                        on_resize_start: move |(id, handle, sx, sy, start)| {
                            interaction
                                .set(
                                    Some(Interaction::Resize {
                                        id,
                                        handle,
                                        sx,
                                        sy,
                                        start,
                                    }),
                                )
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn Grid(w: f64, h: f64, step: f64) -> Element {
    // Lightweight grid: vertical + horizontal lines
    let mut xs = vec![];
    let mut x = 0.0;
    while x <= w {
        xs.push(x);
        x += step;
    }
    let mut ys = vec![];
    let mut y = 0.0;
    while y <= h {
        ys.push(y);
        y += step;
    }

    rsx! {
        g { pointer_events: "none",
            for (i , x) in xs.iter().enumerate() {
                line {
                    key: "vx-{i}",
                    x1: "{x}",
                    y1: "0",
                    x2: "{x}",
                    y2: "{h}",
                    stroke: "#eee",
                    stroke_width: "1",
                }
            }
            for (i , y) in ys.iter().enumerate() {
                line {
                    key: "hy-{i}",
                    x1: "0",
                    y1: "{y}",
                    x2: "{w}",
                    y2: "{y}",
                    stroke: "#eee",
                    stroke_width: "1",
                }
            }
        }
    }
}

#[component]
fn NodeView(
    node: Node,
    on_drag_start: EventHandler<(u32, f64, f64)>,
    on_resize_start: EventHandler<(u32, Handle, f64, f64, Node)>,
) -> Element {
    const VIS: f64 = 10.0;
    const HIT: f64 = 28.0;

    let cx = node.x + node.w / 2.0;
    let cy = node.y + node.h / 2.0;

    let handle_positions: [(Handle, f64, f64); 8] = [
        (Handle::N, cx, node.y),
        (Handle::S, cx, node.y + node.h),
        (Handle::E, node.x + node.w, cy),
        (Handle::W, node.x, cy),
        (Handle::NE, node.x + node.w, node.y),
        (Handle::NW, node.x, node.y),
        (Handle::SE, node.x + node.w, node.y + node.h),
        (Handle::SW, node.x, node.y + node.h),
    ];

    let drag_node = node.clone();

    rsx! {
        g { class: "node", pointer_events: "bounding-box",

            rect {
                x: "{node.x}",
                y: "{node.y}",
                width: "{node.w}",
                height: "{node.h}",
                rx: "10",
                ry: "10",
                fill: "#f7f7f7",
                stroke: "#333",
                stroke_width: "1.5",

                onpointerdown: move |evt| {
                    evt.stop_propagation();
                    let (ex, ey) = element_xy(&evt);
                    on_drag_start.call((drag_node.id, ex - drag_node.x, ey - drag_node.y));
                },
            }

            text {
                x: "{node.x + node.w / 2.0}",
                y: "{node.y + node.h / 2.0}",
                text_anchor: "middle",
                dominant_baseline: "middle",
                font_size: "16",
                fill: "#111",
                pointer_events: "none",
                "{node.label}"
            }

            // Handles are ALWAYS rendered; CSS controls visibility
            g { class: "handles",
                {
                    handle_positions

                        .into_iter()
                        .map(|(h, hx, hy)| {
                            let start_node = node.clone();
                            let key_suffix = format!("{:?}", h);
                            rsx! {
                                g { key: "handle-{start_node.id}-{key_suffix}",
                                    rect {
                                        x: "{hx - HIT / 2.0}",
                                        y: "{hy - HIT / 2.0}",
                                        width: "{HIT}",
                                        height: "{HIT}",
                                        fill: "transparent",
                                        cursor: "{h.cursor()}",
                                        onpointerdown: move |evt| {
                                            evt.stop_propagation();
                                            let (mx, my) = element_xy(&evt);
                                            on_resize_start.call((start_node.id, h, mx, my, start_node.clone()));
                                        },
                                    }


                                    rect {
                                        x: "{hx - VIS / 2.0}",
                                        y: "{hy - VIS / 2.0}",
                                        width: "{VIS}",
                                        height: "{VIS}",
                                        fill: "#666",
                                        stroke: "#fff",
                                        stroke_width: "1",
                                        pointer_events: "none",
                                        rx: "2",
                                        ry: "2",
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}

#[component]
fn EdgeView(diagram: Diagram, edge: Edge, orthogonal: bool) -> Element {
    let nodes: HashMap<u32, Node> = diagram.nodes.into_iter().map(|n| (n.id, n)).collect();
    let Some(a) = nodes.get(&edge.from) else {
        return rsx! {};
    };
    let Some(b) = nodes.get(&edge.to) else {
        return rsx! {};
    };

    // Route: right-center of A to left-center of B
    // let ax = a.x + a.w;
    // let ay = a.y + a.h / 2.0;
    // let bx = b.x;
    // let by = b.y + b.h / 2.0;

    // Route: direction-awarer.
    let acx = a.x + a.w / 2.0;
    let acy = a.y + a.h / 2.0;
    let bcx = b.x + b.w / 2.0;
    let bcy = b.y + b.h / 2.0;

    let dx = bcx - acx;
    let dy = bcy - acy;

    let (ax, ay, bx, by) = if dy.abs() > dx.abs() {
        if dy > 0.0 {
            (acx, a.y + a.h, bcx, b.y)
        } else {
            (acx, a.y, bcx, b.y + b.h)
        }
    } else {
        if dx > 0.0 {
            (a.x + a.w, acy, b.x, bcy)
        } else {
            (a.x, acy, b.x + b.w, bcy)
        }
    };

    let mx = (ax + bx) / 2.0;
    let my = (ay + by) / 2.0;

    // For orthogonal routing.
    let mid_x = (ax + bx) / 2.0;
    let mid_y = (ay + by) / 2.0;

    let (ex, ey) = if (ay - by).abs() > (ax - bx).abs() {
        // vertical routing: down/up then sideways
        (ax, mid_y)
    } else {
        // horizontal routing: sideways then up/down
        (mid_x, ay)
    };

    rsx! {
        g {
            if orthogonal {
                path {
                    d: format!("M {} {} L {} {} L {} {}", ax, ay, ex, ey, bx, by),
                    fill: "none",
                    stroke: "#333",
                    stroke_width: "2",
                    marker_end: "url(#arrow)",
                }
            } else {
                line {
                    x1: "{ax}",
                    y1: "{ay}",
                    x2: "{bx}",
                    y2: "{by}",
                    stroke: "#333",
                    stroke_width: "2",
                    marker_end: "url(#arrow)",
                }
            }

            if let Some(label) = edge.label {
                text {
                    x: "{mid_x}",
                    y: "{mid_y - 8.0}",
                    text_anchor: "middle",
                    font_size: "13",
                    fill: "#111",
                    stroke: "rgba(255,255,255,0.7)",
                    stroke_width: "6",
                    paint_order: "stroke",
                    pointer_events: "none",
                    "{label}"
                }
            }
        }
    }
}

fn apply_resize_delta(n: &mut Node, handle: Handle, dx: f64, dy: f64, min_w: f64, min_h: f64) {
    // Work in edges to make snapping easier.
    let mut left = n.x;
    let mut right = n.x + n.w;
    let mut top = n.y;
    let mut bottom = n.y + n.h;

    match handle {
        Handle::E => right += dx,
        Handle::W => left += dx,
        Handle::S => bottom += dy,
        Handle::N => top += dy,
        Handle::NE => {
            right += dx;
            top += dy;
        }
        Handle::NW => {
            left += dx;
            top += dy;
        }
        Handle::SE => {
            right += dx;
            bottom += dy;
        }
        Handle::SW => {
            left += dx;
            bottom += dy;
        }
    }

    // Enforce mins by adjusting the moved edge back.
    if right - left < min_w {
        match handle {
            Handle::W | Handle::NW | Handle::SW => left = right - min_w,
            _ => right = left + min_w,
        }
    }
    if bottom - top < min_h {
        match handle {
            Handle::N | Handle::NE | Handle::NW => top = bottom - min_h,
            _ => bottom = top + min_h,
        }
    }

    n.x = left;
    n.y = top;
    n.w = right - left;
    n.h = bottom - top;
}

fn snap_move(
    mut proposed: Node,
    others: &[Node],
    grid: f64,
    grid_thresh: f64,
    align_thresh: f64,
) -> (Node, Guides) {
    let mut g = Guides { lines: vec![] };

    // Grid snap for x/y
    let (sx, gx) = snap_to_grid(proposed.x, grid, grid_thresh);
    let (sy, gy) = snap_to_grid(proposed.y, grid, grid_thresh);
    if gx {
        g.lines.push(GuideLine {
            x1: sx,
            y1: 0.0,
            x2: sx,
            y2: 10_000.0,
        });
    }
    if gy {
        g.lines.push(GuideLine {
            x1: 0.0,
            y1: sy,
            x2: 10_000.0,
            y2: sy,
        });
    }
    proposed.x = sx;
    proposed.y = sy;

    // Alignment snap (left/right/center; top/bottom/center)
    let (snapped_x, x_guides) = snap_x_family(
        proposed.x,
        proposed.w,
        proposed.y,
        proposed.h,
        others,
        align_thresh,
    );
    let (snapped_y, y_guides) = snap_y_family(
        proposed.y,
        proposed.h,
        proposed.x,
        proposed.w,
        others,
        align_thresh,
    );

    proposed.x = snapped_x;
    proposed.y = snapped_y;
    g.lines.extend(x_guides);
    g.lines.extend(y_guides);

    (proposed, g)
}

fn snap_resize(
    mut proposed: Node,
    start: &Node,
    handle: Handle,
    others: &[Node],
    grid: f64,
    grid_thresh: f64,
    align_thresh: f64,
    min_w: f64,
    min_h: f64,
) -> (Node, Guides) {
    let mut g = Guides { lines: vec![] };

    // Edges of proposed
    let mut left = proposed.x;
    let mut right = proposed.x + proposed.w;
    let mut top = proposed.y;
    let mut bottom = proposed.y + proposed.h;

    // Determine which edges are "active" for snapping based on handle
    let snap_left = matches!(handle, Handle::W | Handle::NW | Handle::SW);
    let snap_right = matches!(handle, Handle::E | Handle::NE | Handle::SE);
    let snap_top = matches!(handle, Handle::N | Handle::NE | Handle::NW);
    let snap_bottom = matches!(handle, Handle::S | Handle::SE | Handle::SW);

    // Grid snap on active edges
    if snap_left {
        let (v, ok) = snap_to_grid(left, grid, grid_thresh);
        if ok {
            g.lines.push(GuideLine {
                x1: v,
                y1: 0.0,
                x2: v,
                y2: 10_000.0,
            });
        }
        left = v;
    }
    if snap_right {
        let (v, ok) = snap_to_grid(right, grid, grid_thresh);
        if ok {
            g.lines.push(GuideLine {
                x1: v,
                y1: 0.0,
                x2: v,
                y2: 10_000.0,
            });
        }
        right = v;
    }
    if snap_top {
        let (v, ok) = snap_to_grid(top, grid, grid_thresh);
        if ok {
            g.lines.push(GuideLine {
                x1: 0.0,
                y1: v,
                x2: 10_000.0,
                y2: v,
            });
        }
        top = v;
    }
    if snap_bottom {
        let (v, ok) = snap_to_grid(bottom, grid, grid_thresh);
        if ok {
            g.lines.push(GuideLine {
                x1: 0.0,
                y1: v,
                x2: 10_000.0,
                y2: v,
            });
        }
        bottom = v;
    }

    // Alignment snap to other nodes on active edges/centers
    // X: snap left/right/center depending on which edge is active.
    if snap_left || snap_right {
        let center = (left + right) / 2.0;
        let (new_left, new_right, x_guides) = snap_x_edges(
            left,
            right,
            center,
            top,
            bottom,
            others,
            align_thresh,
            snap_left,
            snap_right,
        );
        left = new_left;
        right = new_right;
        g.lines.extend(x_guides);
    }

    // Y: snap top/bottom/center
    if snap_top || snap_bottom {
        let center = (top + bottom) / 2.0;
        let (new_top, new_bottom, y_guides) = snap_y_edges(
            top,
            bottom,
            center,
            left,
            right,
            others,
            align_thresh,
            snap_top,
            snap_bottom,
        );
        top = new_top;
        bottom = new_bottom;
        g.lines.extend(y_guides);
    }

    // Enforce mins (and keep the opposite edge anchored if needed)
    if right - left < min_w {
        if snap_left && !snap_right {
            left = right - min_w;
        } else {
            right = left + min_w;
        }
    }
    if bottom - top < min_h {
        if snap_top && !snap_bottom {
            top = bottom - min_h;
        } else {
            bottom = top + min_h;
        }
    }

    // If handle is single-edge (e.g., E), keep the other edge anchored to start for stability
    // (prevents drifting when both grid+align snap happen).
    // This is optional; it improves feel.
    match handle {
        Handle::E => {
            left = start.x;
            right = left + (right - left).max(min_w);
        }
        Handle::W => {
            right = start.x + start.w;
            left = right - (right - left).max(min_w);
        }
        Handle::S => {
            top = start.y;
            bottom = top + (bottom - top).max(min_h);
        }
        Handle::N => {
            bottom = start.y + start.h;
            top = bottom - (bottom - top).max(min_h);
        }
        _ => {}
    }

    proposed.x = left;
    proposed.y = top;
    proposed.w = right - left;
    proposed.h = bottom - top;

    (proposed, g)
}

fn snap_to_grid(v: f64, step: f64, thresh: f64) -> (f64, bool) {
    let nearest = (v / step).round() * step;
    if (nearest - v).abs() <= thresh {
        (nearest, true)
    } else {
        (v, false)
    }
}

fn snap_x_family(
    x: f64,
    w: f64,
    y: f64,
    h: f64,
    others: &[Node],
    thresh: f64,
) -> (f64, Vec<GuideLine>) {
    let mut guides = vec![];

    let left = x;
    let right = x + w;
    let center = x + w / 2.0;

    let mut best_dx: Option<f64> = None;
    let mut best_line_x: Option<f64> = None;

    for o in others {
        let o_left = o.x;
        let o_right = o.x + o.w;
        let o_center = o.x + o.w / 2.0;

        for (a, b) in [
            (left, o_left),
            (left, o_center),
            (left, o_right),
            (center, o_left),
            (center, o_center),
            (center, o_right),
            (right, o_left),
            (right, o_center),
            (right, o_right),
        ] {
            let dx = b - a;
            if dx.abs() <= thresh {
                if best_dx.map(|bdx| dx.abs() < bdx.abs()).unwrap_or(true) {
                    best_dx = Some(dx);
                    best_line_x = Some(b);
                }
            }
        }
    }

    let snapped_x = if let Some(dx) = best_dx { x + dx } else { x };
    if let Some(line_x) = best_line_x {
        guides.push(GuideLine {
            x1: line_x,
            y1: 0.0,
            x2: line_x,
            y2: 10_000.0,
        });
    }

    (snapped_x, guides)
}

fn snap_y_family(
    y: f64,
    h: f64,
    x: f64,
    w: f64,
    others: &[Node],
    thresh: f64,
) -> (f64, Vec<GuideLine>) {
    let mut guides = vec![];

    let top = y;
    let bottom = y + h;
    let center = y + h / 2.0;

    let mut best_dy: Option<f64> = None;
    let mut best_line_y: Option<f64> = None;

    for o in others {
        let o_top = o.y;
        let o_bottom = o.y + o.h;
        let o_center = o.y + o.h / 2.0;

        for (a, b) in [
            (top, o_top),
            (top, o_center),
            (top, o_bottom),
            (center, o_top),
            (center, o_center),
            (center, o_bottom),
            (bottom, o_top),
            (bottom, o_center),
            (bottom, o_bottom),
        ] {
            let dy = b - a;
            if dy.abs() <= thresh {
                if best_dy.map(|bdy| dy.abs() < bdy.abs()).unwrap_or(true) {
                    best_dy = Some(dy);
                    best_line_y = Some(b);
                }
            }
        }
    }

    let snapped_y = if let Some(dy) = best_dy { y + dy } else { y };
    if let Some(line_y) = best_line_y {
        guides.push(GuideLine {
            x1: 0.0,
            y1: line_y,
            x2: 10_000.0,
            y2: line_y,
        });
    }

    (snapped_y, guides)
}

fn snap_x_edges(
    left: f64,
    right: f64,
    center: f64,
    top: f64,
    bottom: f64,
    others: &[Node],
    thresh: f64,
    snap_left: bool,
    snap_right: bool,
) -> (f64, f64, Vec<GuideLine>) {
    let mut guides = vec![];

    let candidates_self = {
        let mut v = vec![];
        if snap_left {
            v.push(left);
        }
        if snap_right {
            v.push(right);
        }
        // allow center snapping for nicer feel when resizing corners too
        v.push(center);
        v
    };

    let mut best_shift: Option<f64> = None;
    let mut best_line_x: Option<f64> = None;

    for o in others {
        let o_left = o.x;
        let o_right = o.x + o.w;
        let o_center = o.x + o.w / 2.0;

        for s in &candidates_self {
            for target in [o_left, o_center, o_right] {
                let shift = target - *s;
                if shift.abs() <= thresh {
                    if best_shift.map(|bs| shift.abs() < bs.abs()).unwrap_or(true) {
                        best_shift = Some(shift);
                        best_line_x = Some(target);
                    }
                }
            }
        }
    }

    let mut new_left = left;
    let mut new_right = right;

    if let Some(shift) = best_shift {
        // Apply shift to whichever edge is active; if both are active (corner resize),
        // shift the whole rect horizontally (preserves width) by applying to both.
        if snap_left && snap_right {
            new_left += shift;
            new_right += shift;
        } else if snap_left {
            new_left += shift;
        } else if snap_right {
            new_right += shift;
        } else {
            // center-only case: shift both
            new_left += shift;
            new_right += shift;
        }
    }

    if let Some(line_x) = best_line_x {
        guides.push(GuideLine {
            x1: line_x,
            y1: 0.0,
            x2: line_x,
            y2: 10_000.0,
        });
        // Additionally, add a short segment across the active rect for clarity
        guides.push(GuideLine {
            x1: line_x,
            y1: top,
            x2: line_x,
            y2: bottom,
        });
    }

    (new_left, new_right, guides)
}

fn snap_y_edges(
    top: f64,
    bottom: f64,
    center: f64,
    left: f64,
    right: f64,
    others: &[Node],
    thresh: f64,
    snap_top: bool,
    snap_bottom: bool,
) -> (f64, f64, Vec<GuideLine>) {
    let mut guides = vec![];

    let candidates_self = {
        let mut v = vec![];
        if snap_top {
            v.push(top);
        }
        if snap_bottom {
            v.push(bottom);
        }
        v.push(center);
        v
    };

    let mut best_shift: Option<f64> = None;
    let mut best_line_y: Option<f64> = None;

    for o in others {
        let o_top = o.y;
        let o_bottom = o.y + o.h;
        let o_center = o.y + o.h / 2.0;

        for s in &candidates_self {
            for target in [o_top, o_center, o_bottom] {
                let shift = target - *s;
                if shift.abs() <= thresh {
                    if best_shift.map(|bs| shift.abs() < bs.abs()).unwrap_or(true) {
                        best_shift = Some(shift);
                        best_line_y = Some(target);
                    }
                }
            }
        }
    }

    let mut new_top = top;
    let mut new_bottom = bottom;

    if let Some(shift) = best_shift {
        if snap_top && snap_bottom {
            new_top += shift;
            new_bottom += shift;
        } else if snap_top {
            new_top += shift;
        } else if snap_bottom {
            new_bottom += shift;
        } else {
            new_top += shift;
            new_bottom += shift;
        }
    }

    if let Some(line_y) = best_line_y {
        guides.push(GuideLine {
            x1: 0.0,
            y1: line_y,
            x2: 10_000.0,
            y2: line_y,
        });
        guides.push(GuideLine {
            x1: left,
            y1: line_y,
            x2: right,
            y2: line_y,
        });
    }

    (new_top, new_bottom, guides)
}

fn element_xy(evt: &Event<PointerData>) -> (f64, f64) {
    // Dioxus 0.7 PointerData: element_coordinates() (relative to the event target)
    let p = evt.data().element_coordinates();
    (p.x, p.y)
}
