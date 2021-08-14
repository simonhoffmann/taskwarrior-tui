// Based on https://gist.github.com/diwic/5c20a283ca3a03752e1a27b0f3ebfa30
// See https://old.reddit.com/r/rust/comments/4xneq5/the_calendar_example_challenge_ii_why_eddyb_all/

use std::fmt;

const COL_WIDTH: usize = 21;

use chrono::{Datelike, Duration, Local, Month, NaiveDate, NaiveDateTime, TimeZone};

use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, Widget},
};

use std::cmp::min;

#[derive(Debug, Clone)]
pub struct Projects<'a> {
    pub block: Option<Block<'a>>,
    pub year: i32,
    pub month: u32,
    pub style: Style,
    pub months_per_row: usize,
    pub date_style: Vec<(NaiveDate, Style)>,
    pub today_style: Style,
    pub title_background_color: Color,
}

impl<'a> Default for Projects<'a> {
    fn default() -> Projects<'a> {
        let year = Local::today().year();
        let month = Local::today().month();
        Projects {
            block: None,
            style: Default::default(),
            months_per_row: 0,
            year,
            month,
            date_style: vec![],
            today_style: Default::default(),
            title_background_color: Color::Reset,
        }
    }
}

impl<'a> Projects<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = year;
        if self.year < 0 {
            self.year = 0;
        }
        self
    }

    pub fn month(mut self, month: u32) -> Self {
        self.month = month;
        self
    }

    pub fn date_style(mut self, date_style: Vec<(NaiveDate, Style)>) -> Self {
        self.date_style = date_style;
        self
    }

    pub fn today_style(mut self, today_style: Style) -> Self {
        self.today_style = today_style;
        self
    }

    pub fn months_per_row(mut self, months_per_row: usize) -> Self {
        self.months_per_row = months_per_row;
        self
    }
}

impl<'a> Widget for Projects<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        let area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
    }
}
