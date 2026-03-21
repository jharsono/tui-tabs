//! A tab navigation widget for [Ratatui](https://ratatui.rs) with bordered boxes and rounded corners.
//!
//! Each tab renders as an individual bordered box. The active tab opens into the content
//! below via rounded junction corners while inactive tabs maintain a continuous baseline.
//!
//! # Example
//!
//! ```rust
//! use ratatui::style::{Color, Style};
//! use tui_tabs::TabNav;
//!
//! let widget = TabNav::new(&["Files", "Search", "Settings"], 0)
//!     .highlight_style(Style::new().fg(Color::Cyan))
//!     .border_style(Style::new().fg(Color::DarkGray));
//! ```

use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    symbols,
    widgets::Widget,
};

const DEFAULT_INDICATOR: &str = "▸";

/// Tab navigation rendered as individually bordered boxes.
///
/// Adjacent tabs sit flush (no gap). The active tab's bottom opens into the
/// content below via rounded junction corners. Inactive tabs use `┴` junctions
/// so the bottom line stays continuous. A horizontal baseline spans the full width.
///
/// Requires exactly 3 rows of height (top border, label, bottom/baseline).
#[must_use]
pub struct TabNav<'a> {
    tabs: &'a [&'a str],
    selected: usize,
    style: Style,
    highlight_style: Style,
    highlight_bold: bool,
    border_style: Style,
    indicator: Option<&'a str>,
    border_set: symbols::border::Set<'a>,
}

impl<'a> TabNav<'a> {
    /// Creates a new `TabNav` with the given tab labels and selected index.
    ///
    /// All styles default to `Style::new()` (unstyled). The active tab is bold
    /// by default. Override with [`highlight_bold`](Self::highlight_bold).
    pub fn new(tabs: &'a [&'a str], selected: usize) -> Self {
        Self {
            tabs,
            selected,
            style: Style::new(),
            highlight_style: Style::new(),
            highlight_bold: true,
            border_style: Style::new(),
            indicator: Some(DEFAULT_INDICATOR),
            border_set: symbols::border::ROUNDED,
        }
    }

    /// Style for inactive tab labels.
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Style for the active tab label. Bold is applied on top unless disabled
    /// via [`highlight_bold`](Self::highlight_bold).
    pub fn highlight_style(mut self, style: impl Into<Style>) -> Self {
        self.highlight_style = style.into();
        self
    }

    /// Whether to auto-apply bold to the active tab. Default: `true`.
    pub fn highlight_bold(mut self, bold: bool) -> Self {
        self.highlight_bold = bold;
        self
    }

    /// Style for borders and the baseline.
    pub fn border_style(mut self, style: impl Into<Style>) -> Self {
        self.border_style = style.into();
        self
    }

    /// Symbol shown left of the active tab label. Default: `Some("▸")`.
    /// Pass `None` to disable.
    pub fn indicator(mut self, indicator: Option<&'a str>) -> Self {
        self.indicator = indicator;
        self
    }

    /// Border character set. Default: [`symbols::border::ROUNDED`].
    /// Pass [`symbols::border::PLAIN`] for square corners.
    pub fn border_set(mut self, set: symbols::border::Set<'a>) -> Self {
        self.border_set = set;
        self
    }
}

/// `│  ▸ Label  │` → border(1) + pad(3) + label + pad(3) + border(1)
fn tab_width(label: &str) -> u16 {
    label.len() as u16 + 8
}

impl Widget for TabNav<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 3 || area.width == 0 || self.tabs.is_empty() {
            return;
        }

        let border = &self.border_set;
        let bs = self.border_style;

        let top_y = area.y;
        let mid_y = area.y + 1;
        let bot_y = area.y + 2;

        draw_baseline(area.x, area.right(), bot_y, border, bs, buf);

        let positions = compute_tab_positions(self.tabs, area.x, area.right());

        for (i, (label, &(tx, tw))) in self.tabs.iter().zip(&positions).enumerate() {
            let active = i == self.selected;

            let left_x = tx;
            let right_x = tx + tw - 1;

            draw_top_border(left_x, right_x, top_y, border, bs, buf);
            draw_side_borders(left_x, right_x, mid_y, border, bs, buf);

            let text_style = if active {
                let mut s = self.highlight_style;
                if self.highlight_bold {
                    s = s.add_modifier(Modifier::BOLD);
                }
                s
            } else {
                self.style
            };

            draw_label(left_x, right_x, mid_y, label, text_style, buf);

            if active {
                if let Some(sym) = self.indicator {
                    let indicator_x = left_x + 2;
                    buf[(indicator_x, mid_y)]
                        .set_symbol(sym)
                        .set_style(text_style);
                }

                draw_active_bottom(left_x, right_x, bot_y, border, bs, buf);
            } else {
                draw_inactive_bottom(left_x, right_x, bot_y, bs, buf);
            }
        }
    }
}

fn draw_baseline(
    start: u16,
    end: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    for x in start..end {
        buf[(x, y)]
            .set_symbol(border.horizontal_top)
            .set_style(style);
    }
}

fn compute_tab_positions(tabs: &[&str], start: u16, end: u16) -> Vec<(u16, u16)> {
    let mut positions = Vec::with_capacity(tabs.len());
    let mut x = start;

    for label in tabs {
        let w = tab_width(label);
        if x + w > end {
            break;
        }
        positions.push((x, w));
        x += w;
    }

    positions
}

fn draw_top_border(
    left: u16,
    right: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, y)].set_symbol(border.top_left).set_style(style);

    for x in (left + 1)..right {
        buf[(x, y)]
            .set_symbol(border.horizontal_top)
            .set_style(style);
    }

    buf[(right, y)]
        .set_symbol(border.top_right)
        .set_style(style);
}

fn draw_side_borders(
    left: u16,
    right: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, y)]
        .set_symbol(border.vertical_left)
        .set_style(style);

    buf[(right, y)]
        .set_symbol(border.vertical_right)
        .set_style(style);
}

fn draw_label(left: u16, right: u16, y: u16, label: &str, style: Style, buf: &mut Buffer) {
    let label_x = left + 4;

    for (j, ch) in label.chars().enumerate() {
        let cx = label_x + j as u16;
        if cx >= right {
            break;
        }
        buf[(cx, y)].set_char(ch).set_style(style);
    }
}

fn draw_active_bottom(
    left: u16,
    right: u16,
    y: u16,
    border: &symbols::border::Set,
    style: Style,
    buf: &mut Buffer,
) {
    buf[(left, y)]
        .set_symbol(border.bottom_right)
        .set_style(style);

    for x in (left + 1)..right {
        buf[(x, y)].set_symbol(" ").set_style(style);
    }

    buf[(right, y)]
        .set_symbol(border.bottom_left)
        .set_style(style);
}

fn draw_inactive_bottom(left: u16, right: u16, y: u16, style: Style, buf: &mut Buffer) {
    buf[(left, y)].set_symbol("┴").set_style(style);
    buf[(right, y)].set_symbol("┴").set_style(style);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui_core::buffer::Buffer;
    use ratatui_core::layout::Rect;

    fn render(tabs: &[&str], selected: usize, width: u16) -> Buffer {
        let area = Rect::new(0, 0, width, 3);
        let mut buf = Buffer::empty(area);
        TabNav::new(tabs, selected).render(area, &mut buf);
        buf
    }

    #[test]
    fn empty_tabs_renders_nothing() {
        let area = Rect::new(0, 0, 40, 3);
        let mut buf = Buffer::empty(area);
        let expected = buf.clone();
        TabNav::new(&[], 0).render(area, &mut buf);
        assert_eq!(buf, expected);
    }

    #[test]
    fn insufficient_height_renders_nothing() {
        let area = Rect::new(0, 0, 40, 2);
        let mut buf = Buffer::empty(area);
        let expected = buf.clone();
        TabNav::new(&["Tab"], 0).render(area, &mut buf);
        assert_eq!(buf, expected);
    }

    #[test]
    fn single_tab_renders_three_rows() {
        let buf = render(&["Hi"], 0, 30);
        let top_line = line_str(&buf, 0);
        let mid_line = line_str(&buf, 1);
        let bot_line = line_str(&buf, 2);

        assert!(top_line.starts_with("╭"));
        assert!(top_line.contains("╮"));
        assert!(mid_line.contains("Hi"));
        assert!(mid_line.contains("▸"));
        // Active tab opens: bottom corners are ╯ and ╰
        assert!(bot_line.starts_with("╯"));
    }

    #[test]
    fn inactive_tab_has_junction_corners() {
        let buf = render(&["A", "B"], 1, 30);
        let bot_line = line_str(&buf, 2);
        // First tab (inactive) should have ┴ at its left edge
        assert!(bot_line.starts_with("┴"));
    }

    #[test]
    fn indicator_appears_on_active_tab() {
        let buf = render(&["Tab"], 0, 20);
        let mid_line = line_str(&buf, 1);
        assert!(mid_line.contains("▸"));
    }

    #[test]
    fn no_indicator_when_disabled() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        TabNav::new(&["Tab"], 0)
            .indicator(None)
            .render(area, &mut buf);
        let mid_line = line_str(&buf, 1);
        assert!(!mid_line.contains("▸"));
    }

    #[test]
    fn overflow_tabs_are_omitted() {
        // Each tab "Long" = 4 + 8 = 12 wide. Width 20 fits only 1.
        let buf = render(&["Long", "Overflow"], 0, 20);
        let mid_line = line_str(&buf, 1);
        assert!(mid_line.contains("Long"));
        assert!(!mid_line.contains("Overflow"));
    }

    #[test]
    fn square_borders() {
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        TabNav::new(&["Tab"], 0)
            .border_set(symbols::border::PLAIN)
            .render(area, &mut buf);
        let top_line = line_str(&buf, 0);
        assert!(top_line.starts_with("┌"));
    }

    #[test]
    fn tab_width_calculation() {
        assert_eq!(tab_width("Hi"), 10);
        assert_eq!(tab_width(""), 8);
        assert_eq!(tab_width("Nodes"), 13);
    }

    #[test]
    fn two_active_tabs_layout() {
        let buf = render(&["A", "B"], 0, 30);
        let mid_line = line_str(&buf, 1);
        // Both labels present
        assert!(mid_line.contains("A"));
        assert!(mid_line.contains("B"));
    }

    fn line_str(buf: &Buffer, y: u16) -> String {
        let area = buf.area();
        (area.x..area.right())
            .map(|x| buf[(x, y)].symbol().to_string())
            .collect()
    }
}
