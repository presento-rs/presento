use markdown::{mdast::Node, to_mdast, ParseOptions};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Macro to iterate over all children of a node and parse, flatten and collect those
#[macro_export]
macro_rules! parse_children {
    ( $x:expr) => {{
        $x.children
            .iter()
            .map(|child| node_to_lines(child.clone()))
            .flatten()
            .collect::<Vec<Line>>()
    }};
}

/// Iterate over a multiple lines and patch their respective style
#[macro_export]
macro_rules! patch_styles {
    ($node:expr, $style:expr) => {{
        let mut spans = node_to_spans($node);
        spans.iter_mut().for_each(|span| span.patch_style($style));
        spans
    }};
}

/// Structures like headings are single lines, therefore their children are parsed as spans
pub fn node_to_spans<'a>(node: Node) -> Vec<Span<'a>> {
    let bold_style = Style::new().add_modifier(Modifier::BOLD);

    match node {
        Node::Text(text) => vec![Span::from(text.value)],
        Node::Strong(strong) => {
            let mut spans = strong
                .children
                .iter()
                .map(|child| node_to_spans(child.clone()))
                .flatten()
                .collect::<Vec<Span>>();
            spans
                .iter_mut()
                .for_each(|child| child.patch_style(bold_style));
            spans
        }
        Node::InlineCode(code) => {
            let mut span = Span::from(code.value);
            span.patch_style(Style::new().bg(Color::DarkGray));
            vec![span]
        }
        _ => vec![Span::from(format! {"{:?}", node})],
    }
}

/// Recursively parse markdown nodes into Lines that can be rendered by ratatui
pub fn node_to_lines<'a>(node: Node) -> Vec<Line<'a>> {
    match node.clone() {
        Node::Paragraph(paragraph) => vec![Line::from(
            paragraph
                .children
                .into_iter()
                .map(|child| node_to_spans(child))
                .flatten()
                .collect::<Vec<Span>>(),
        )],
        Node::Heading(h) => h
            .children
            .iter()
            .map(|child| {
                Line::from(patch_styles!(
                    child.clone(),
                    Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
                ))
            })
            .collect(),
        // ignore HTML for now
        Node::Html(_) => Vec::new(),
        Node::Text(node) => vec![Line::from(node.value.clone())],
        Node::ListItem(item) => parse_children!(item),
        Node::List(mut n) => n
            .children
            .iter_mut()
            .map(|child| node_to_lines(child.clone()))
            .flatten()
            .map(|mut list_entry| {
                if list_entry.spans.len() >= 2 && list_entry.spans[1].content == "- " {
                    // list is already indented (nested), increase indent for this list
                    let mut indent = list_entry.spans[0].content.to_string();
                    indent.push_str("  ");
                    list_entry.spans[0] = Span::from(indent);
                } else {
                    // list is not yet indented
                    list_entry.spans.insert(0, Span::from("- "));
                    list_entry.spans.insert(0, Span::from("  "));
                }
                list_entry
            })
            .collect(),
        Node::Strong(strong) => vec![Line::from(patch_styles!(
            node,
            Style::new().add_modifier(Modifier::BOLD)
        ))],
        // ToDo: Properly show links
        Node::Link(l) => parse_children!(l),
        // ToDo: handle metadata
        Node::Yaml(_) => vec![Line::from("-- Found metadata block, currently unused")],
        Node::InlineCode(code) => {
            let mut line = Line::from(code.value);
            line.patch_style(Style::new().bg(Color::DarkGray));
            vec![line]
        }
        _ => vec![Line::from(format! {"{:?}", node})],
    }
}

/// Parse slide content to lines that can be renderes by ratatui
pub fn parse<'a>(content: String) -> Vec<Line<'a>> {
    let ast = to_mdast(content.as_str(), &ParseOptions::default()).unwrap();
    ast.children()
        .unwrap()
        .into_iter()
        .map(|node| node_to_lines(node.clone()))
        .flatten()
        .collect::<Vec<Line>>()
}
