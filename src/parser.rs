use minimad::{parser::parse, Composite, CompositeStyle, Compound};
use ratatui::prelude::*;
use ratatui::{
    text::{Line, Span, Text},
    widgets::Paragraph,
};

pub fn compounds_to_line<'a>(comps: Vec<Compound<'a>>) -> Line<'a> {
    // ToDo: add style attrbiutes
    let text: Vec<Span> = comps
        .iter()
        .map(|comp| {
            let style = Style::new();
            if comp.bold {
                style.bold();
            };
            if comp.italic {
                style.italic();
            };
            Span::styled(comp.src, style)
        })
        .collect();
    Line::from(text)
}

pub fn parse_normal<'a>(comp: &Composite<'a>) -> Line<'a> {
    match comp.style {
        CompositeStyle::Paragraph => compounds_to_line(comp.compounds.clone()),
        _ => Line::from("Other compound"),
    }
}

pub fn line_to_widget<'a>(line: &minimad::Line<'a>) -> Line<'a> {
    match line {
        minimad::Line::Normal(normal) => parse_normal(normal),
        _ => Line::from("Currently unsupported :c"),
    }
}

pub fn into_paragraph<'a>(content: &'a String) -> Paragraph<'a> {
    let c = content.as_str();
    let text: Vec<Line<'a>> = parse(c, Default::default())
        .lines
        .iter()
        .map(|line| line_to_widget(line))
        .collect();
    Paragraph::new(text)
}
