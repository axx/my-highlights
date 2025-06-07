use dioxus::prelude::*;
use serde::Deserialize;
use indexmap::IndexMap;
use serde_json;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let data = String::from(
        r##"[
        {
            "period": "January 2025",
            "category": "Release Management",
            "description": "Managed the release of version 1.0, ensuring all features were tested and documented."
        },
        {
            "period": "January 2025",
            "category": "Configuration Management",
            "description": "Developed new features for the application, including user authentication and profile management."
        },
        {
            "period": "January 2025",
            "category": "Control Plane",
            "description": "Start of requirements gathering."
        },
        {
            "period": "February 2025",
            "category": "Configuration Management",
            "description": "Started study on AC3."
        },
        {
            "period": "February 2025",
            "category": "Release Management",
            "description": "Managed the release of version 1.1, ensuring all features were tested and documented."
        },
        {
            "period": "February 2025",
            "category": "Configuration Management",
            "description": "Creation of AC3 lib prototype."
        }
    ]"##
    );

    let highlights: Vec<StoredHighlight> = serde_json::from_str(&data).unwrap_or_else(|_| {
        panic!("Failed to parse highlights data");
    });
    let highlight_groups: Vec<HighlightGroup> = highlights
        .into_iter()
        .fold(IndexMap::new(), |mut acc, highlight| {
            acc.entry(highlight.period.clone())
                .or_insert_with(Vec::new)
                .push(Highlight::new(&highlight.category, &highlight.description));
            acc
        })
        .into_iter()
        .map(|(period, highlights)| HighlightGroup::new(&period, highlights))
        .collect();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Highlights { highlight_groups }
    }
}

#[component]
fn Highlights(highlight_groups: Vec<HighlightGroup>) -> Element {
    rsx! {
        div { id: "highlights",
            h1 { class: "highlights-title", "My Highlights" }
            ul { class: "highlight-groups",
                for group in highlight_groups {
                    li { class: "highlight-group",
                        HighlightPeriod { period: group.period }
                        HighlightEntries { highlights: group.highlights }
                    }
                }
            }
        }
    }
}

#[component]
fn HighlightEntries(highlights: Vec<Highlight>) -> Element {
    rsx! {
        div { class: "highlight-entries",
            for highlight in highlights {
                HighlightEntry { highlight }
            }
        }
    }
}

#[component]
fn HighlightEntry(highlight: Highlight) -> Element {
    rsx! {
        div { class: "highlight-entry {highlight.css_class}",
            div { class: "highlight-category", "{highlight.category}" }
            div { class: "highlight-description", "{highlight.description}" }
        }
    }
}

#[component]
fn HighlightPeriod(period: String) -> Element {
    rsx! {
        div { class: "highlight-period", "{period}" }
    }
}

#[derive(Props, PartialEq, Clone, Debug, Deserialize)]
struct Highlight {
    category: String,
    description: String,
    css_class: String,
}

impl Highlight {
    fn new(category: &str, description: &str) -> Self {
        Highlight {
            category: category.to_string(),
            description: description.to_string(),
            css_class: category.to_string().to_lowercase().replace(" ", "-"),
        }
    }
}

type Period = String;

#[derive(Props, PartialEq, Clone, Debug, Deserialize)]
struct HighlightGroup {
    period: Period,
    highlights: Vec<Highlight>,
}
impl HighlightGroup {
    fn new(period: &str, highlights: Vec<Highlight>) -> Self {
        HighlightGroup {
            period: period.to_string(),
            highlights,
        }
    }
}

#[derive(Props, PartialEq, Clone, Deserialize, Debug)]
struct StoredHighlight {
    category: String,
    description: String,
    period: Period,
}
