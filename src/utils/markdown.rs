use dioxus::prelude::*;
use pulldown_cmark::Parser;

///
/// Copied from https://github.com/DioxusLabs/markdown to prevent breaking changes
///
#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    id: Signal<String>,
    #[props(default)]
    class: Signal<String>,

    content: ReadOnlySignal<String>,
}

/// Render some text as markdown.
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = &*props.content.read();
    let parser = Parser::new(content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    rsx! {
        div {
            id: "{&*props.id.read()}",
            class: "{&*props.class.read()}",
            dangerous_inner_html: "{html_buf}"
        }
    }
}