use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let highlights1 = vec![
        Highlight::new(
            "Release Management",
            "Managed the release of version 1.0, ensuring all features were tested and documented."
        ),
        Highlight::new(
            "Configuration Management",
            "Developed new features for the application, including user authentication and profile management."
        ),
        Highlight::new(
            "Control Plane",
            "Start of requirements gathering."
        ),
    ];

    let highlights2 = vec![
        Highlight::new("Configuration Management", "Started study on AC3."),
        Highlight::new(
            "Release Management",
            "Managed the release of version 1.1, ensuring all features were tested and documented."
        ),
        Highlight::new("Configuration Management", "Creation of AC3 lib prototype.")
    ];

    let highlight_groups = vec![
        HighlightGroup::new("January 2025", highlights1),
        HighlightGroup::new("February 2025", highlights2)
    ];

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

#[derive(Props, PartialEq, Clone)]
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

#[derive(Props, PartialEq, Clone)]
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
