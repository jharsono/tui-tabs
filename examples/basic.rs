use ratatui::{
    prelude::{Constraint, Layout, Widget},
    widgets::{Block, Paragraph},
};
use tui_tabs::TabNav;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| {
        terminal.draw(|frame| {
            let area = frame.area();
            let [tabs, content] =
                Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

            TabNav::new(&["Files", "Search", "Settings"], 0).render(tabs, frame.buffer_mut());

            Paragraph::new("Select a tab to get started.")
                .block(Block::bordered().title(" Files "))
                .render(content, frame.buffer_mut());
        })?;

        ratatui::crossterm::event::read()?;
        Ok::<(), std::io::Error>(())
    })?;
    Ok(())
}
