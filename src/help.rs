use crate::keyconfig::KeyConfig;
use std::cmp;

use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

const TEXT: &str = include_str!("../KEYBINDINGS.md");

pub struct Help {
    pub title: String,
    pub scroll: u16,
    pub text_height: usize,
    pub keyconfig: KeyConfig,
}

impl Help {
    pub fn new() -> Self {
        Self {
            title: "Help".to_string(),
            scroll: 0,
            text_height: TEXT.lines().count(),
            keyconfig: KeyConfig::default(),
        }
    }
}

impl Default for Help {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text: Vec<Spans> = vec![
            Spans::from("Keybindings:"),
            Spans::from(""),
            Spans::from("    Esc:                                 - Exit current action"),
            Spans::from(""),
            Spans::from("    ]: Next view                         - Go to next view"),
            Spans::from(""),
            Spans::from("    [: Previous view                     - Go to previous view"),
            Spans::from(""),
            Spans::from("    ?: help                              - Help menu"),
            Spans::from(""),
            Spans::from(""),
            Spans::from("Keybindings for task report:"),
            Spans::from(""),
            Spans::from("    /: task {string}                     - Filter task report"),
            Spans::from(""),
            Spans::from("    a: task add {string}                 - Add new task"),
            Spans::from(""),
            Spans::from("    d: task {selected} done              - Mark task as done"),
            Spans::from(""),
            Spans::from("    e: task {selected} edit              - Open selected task in editor"),
            Spans::from(""),
            Spans::from("    j: {selected+=1}                     - Move down in task report"),
            Spans::from(""),
            Spans::from("    k: {selected-=1}                     - Move up in task report"),
            Spans::from(""),
            Spans::from("    J: {selected+=pageheight}            - Move page down in task report"),
            Spans::from(""),
            Spans::from("    K: {selected-=pageheight}            - Move page up in task report"),
            Spans::from(""),
            Spans::from("    gg: {selected=last}                  - Go to top"),
            Spans::from(""),
            Spans::from("    G: {selected=last}                   - Go to bottom"),
            Spans::from(""),
            Spans::from("    l: task log {string}                 - Log new task"),
            Spans::from(""),
            Spans::from("    m: task {selected} modify {string}   - Modify selected task"),
            Spans::from(""),
            Spans::from("    q: exit                              - Quit"),
            Spans::from(""),
            Spans::from("    s: task {selected} start/stop        - Toggle start and stop"),
            Spans::from(""),
            Spans::from("    u: task undo                         - Undo"),
            Spans::from(""),
            Spans::from("    x: task delete {selected}            - Delete"),
            Spans::from(""),
            Spans::from("    z: toggle task info                  - Toggle task info view"),
            Spans::from(""),
            Spans::from("    A: task {selected} annotate {string} - Annotate current task"),
            Spans::from(""),
            Spans::from("    !: {string}                          - Custom shell command"),
            Spans::from(""),
            Spans::from("    c: context switcher menu             - Open context switcher menu"),
            Spans::from(""),
            Spans::from(""),
            Spans::from("Keybindings for context switcher:"),
            Spans::from(""),
            Spans::from("    j: {selected+=1}                     - Move down in context menu"),
            Spans::from(""),
            Spans::from("    k: {selected-=1}                     - Move up in context menu"),
            Spans::from(""),
            Spans::from(""),
            Spans::from("Keybindings for calendar:"),
            Spans::from(""),
            Spans::from("    j: {selected+=1}                     - Move forward a year in calendar"),
            Spans::from(""),
            Spans::from("    k: {selected-=1}                     - Move back a year in calendar"),
            Spans::from(""),
            Spans::from("    J: {selected+=10}                    - Move forward a decade in calendar"),
            Spans::from(""),
            Spans::from("    K: {selected-=10}                    - Move back a decade in calendar"),
        ];

        Paragraph::new(text)
            .block(
                Block::default()
                    .title(Span::styled(&self.title, Style::default().add_modifier(Modifier::BOLD)))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Left)
            .scroll((self.scroll, 0))
            .render(area, buf);
    }
}
