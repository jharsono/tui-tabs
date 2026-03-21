use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::{Buffer, Constraint, Layout, Rect, Widget},
    widgets::{Block, Paragraph},
};
use tui_tabs::TabNav;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}

const TABS: &[&str] = &["Dashboard", "Logs", "Settings"];

const CONTENT: &[&str] = &[
    "Welcome to the dashboard. Monitor your system status here.",
    "Application logs will appear in this panel.",
    "Configure your preferences in this section.",
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
                    KeyCode::Tab => {
                        self.selected = (self.selected + 1) % TABS.len();
                    }

                    KeyCode::Left | KeyCode::Char('h') => {
                        self.selected = self.selected.saturating_sub(1);
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.selected = (self.selected + 1).min(TABS.len() - 1);
                    }

                    _ => {}
                }
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tabs, content] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        TabNav::new(TABS, self.selected).render(tabs, buf);

        Paragraph::new(CONTENT[self.selected])
            .block(Block::bordered().title(format!(" {} ", TABS[self.selected])))
            .render(content, buf);
    }
}
