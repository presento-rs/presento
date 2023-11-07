use std::borrow::Cow;

use markdown::{mdast::Node, to_mdast, ParseOptions};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};

/// Macro to iterate over all children of a node and parse, flatten and collect those
#[macro_export]
macro_rules! parse_children {
    ( $x:expr) => {{
        $x.children
            .iter()
            .map(|child| node_to_line(child.clone()))
            .flatten()
            .collect::<Vec<Line>>()
    }};
}

/// Iterate over a multiple lines and patch their respective style
#[macro_export]
macro_rules! patch_styles {
    ($node:expr, $style:expr) => {{
        let mut lines = parse_children! {$node};
        lines.iter_mut().for_each(|line| line.patch_style($style));
        lines
    }};
}

/// Recursively parse markdown nodes into Lines that can be rendered by ratatui
pub fn node_to_line<'a>(node: Node) -> Vec<Line<'a>> {
    match node.clone() {
        Node::Paragraph(paragraph) => parse_children!(paragraph),
        Node::Heading(h) => {
            // collect headlines
            let mut lines = parse_children!(h);
            // color headlines read for testing
            lines.iter_mut().for_each(|line| {
                line.patch_style(Style::new().fg(Color::Red).add_modifier(Modifier::BOLD))
            });
            lines
        }
        // ignore HTML for now
        Node::Html(_) => Vec::new(),
        Node::Text(node) => vec![Line::from(node.value.clone())],
        Node::ListItem(item) => parse_children!(item),
        Node::List(mut n) => n
            .children
            .iter_mut()
            .map(|child| node_to_line(child.clone()))
            .flatten()
            .map(|mut list_entry| {
                let mut indented = "  - ".to_owned();
                indented.push_str(&list_entry.spans[0].content.to_string());
                list_entry.spans[0].content = Cow::from(indented);
                list_entry
            })
            .collect(),
        Node::Strong(strong) => patch_styles!(strong, Style::new().add_modifier(Modifier::BOLD)),
        // ignore links urls for now
        Node::Link(l) => parse_children!(l),
        // ToDo: handle metadata
        Node::Yaml(_) => vec![Line::from("-- Found metadata block, currently unused")],
        _ => vec![Line::from(format! {"{:?}", node})],
    }
}

/// Parse slide content to lines that can be renderes by ratatui
pub fn parse<'a>(content: String) -> Vec<Line<'a>> {
    let ast = to_mdast(content.as_str(), &ParseOptions::default()).unwrap();
    ast.children()
        .unwrap()
        .into_iter()
        .map(|node| node_to_line(node.clone()))
        .flatten()
        .collect::<Vec<Line>>()
}
