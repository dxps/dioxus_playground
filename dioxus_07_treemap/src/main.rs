use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        TreeMap {}
    }
}

#[component]
pub fn TreeMap() -> Element {
    let width = 600.0;
    let height = 400.0;

    let items = build_treemap(&DATA, 0.0, 0.0, width, height);

    rsx! {
        div { style: "font-family: sans-serif; padding: 16px;",
            h2 { "Treemap Example (Dioxus 0.7)" }

            svg {
                width: "{width}",
                height: "{height}",
                style: "border: 1px solid #ccc;",

                for item in items {
                    Rectangle { item }
                }
            }
        }
    }
}

#[component]
fn Rectangle(item: TreemapItem) -> Element {
    return rsx! {
        g {
            rect {
                x: "{item.x}",
                y: "{item.y}",
                width: "{item.w}",
                height: "{item.h}",
                fill: "{item.node.color}",
                stroke: "white",
                stroke_width: "2",
            }
            text {
                x: "{item.x + 6.0}",
                y: "{item.y + 18.0}",
                font_size: "16",
                fill: "white",
                "{item.node.name}"
            }
        }
    };
}

// Simple data model for each treemap cell
#[derive(Clone, Copy, PartialEq)]
struct DataNode {
    name: &'static str,
    value: f32,
    color: &'static str,
}

#[derive(Clone, PartialEq)]
struct TreemapItem {
    node: DataNode,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

// Example data for the treemap
static DATA: [DataNode; 5] = [
    DataNode {
        name: "A",
        value: 6.0,
        color: "#1f77b4",
    },
    DataNode {
        name: "B",
        value: 4.0,
        color: "#ff7f0e",
    },
    DataNode {
        name: "C",
        value: 3.0,
        color: "#2ca02c",
    },
    DataNode {
        name: "D",
        value: 2.0,
        color: "#d62728",
    },
    DataNode {
        name: "E",
        value: 1.0,
        color: "#9467bd",
    },
];

fn build_treemap(data: &[DataNode], x: f32, y: f32, width: f32, height: f32) -> Vec<TreemapItem> {
    let total: f32 = data.iter().map(|n| n.value).sum();
    let mut result = Vec::new();

    if total <= 0.0 {
        return result;
    }

    let mut offset = 0.0;

    // Horizontal “slice-and-dice” layout:
    // each item gets a width proportional to value, full height.
    for node in data {
        let fraction = node.value / total;
        let w = width * fraction;

        result.push(TreemapItem {
            node: *node,
            x: x + offset,
            y,
            w,
            h: height,
        });

        offset += w;
    }

    result
}
