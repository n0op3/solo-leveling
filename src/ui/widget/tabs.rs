use ratatui::layout::Rect;
use ratatui::prelude::Buffer;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::symbols;
use ratatui::widgets::Block;
use ratatui::widgets::Tabs;
use ratatui::widgets::Widget;

#[derive(Debug, Default)]
pub struct TabsState<'a> {
    tabs: Vec<&'a str>,
    index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(tabs: Vec<&'a str>) -> Self {
        Self { tabs, index: 0 }
    }

    pub fn next(&mut self) {
        self.index += 1;
        if self.index == self.tabs.len() {
            self.index = 0;
        }
    }
}

impl Widget for &TabsState<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Tabs::new(self.tabs.clone())
            .block(Block::bordered())
            .style(Style::default().white())
            .highlight_style(Style::default().blue())
            .select(self.index)
            .divider(symbols::DOT)
            .render(area, buf);
    }
}
