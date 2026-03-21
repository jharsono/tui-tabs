use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::{Buffer, Constraint, Layout, Rect, Stylize, Widget},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use tui_tabs::TabNav;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}

const TABS: &[&str] = &[
    "Overview",
    "Nodes",
    "Network",
    "Content",
    "Inference",
    "Config",
    "Logs",
];

#[derive(Default)]
struct App {
    selected: usize,
}

impl App {
    fn run(mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::BackTab => {
                        self.selected = (self.selected + TABS.len() - 1) % TABS.len();
                    }
                    KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                        self.selected = (self.selected + 1) % TABS.len();
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.selected = (self.selected + TABS.len() - 1) % TABS.len();
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bg = Color::Rgb(20, 20, 40);
        let highlight = Color::LightBlue;
        let dim = Color::DarkGray;
        let border = Color::Rgb(60, 60, 100);

        Block::new().style(Style::new().bg(bg)).render(area, buf);

        let [title, tabs, content] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(area);

        "tui-tabs"
            .bold()
            .fg(highlight)
            .into_centered_line()
            .render(title, buf);

        TabNav::new(TABS, self.selected)
            .style(Style::new().fg(dim).bg(bg))
            .highlight_style(Style::new().fg(highlight).bg(bg))
            .border_style(Style::new().fg(border).bg(bg))
            .render(tabs, buf);

        Paragraph::new(format!("Selected: {}", TABS[self.selected]))
            .block(
                Block::bordered()
                    .title(format!(" {} ", TABS[self.selected]))
                    .border_style(Style::new().fg(border))
                    .style(Style::new().fg(Color::White).bg(bg)),
            )
            .render(content, buf);
    }
}
