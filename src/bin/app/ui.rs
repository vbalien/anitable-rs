use tui::layout::{Constraint, Direction, Layout, Alignment};
use tui::style::{Color, Style, Modifier};
use tui::widgets::{Block, Borders, Tabs, Widget, Text, Paragraph, Table, Row};
use tui::backend::Backend;
use tui::Terminal;
use std::io;
use crate::app::app::App;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let header = ["시각", "제목", "장르"];
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);
        let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
        let normal_style = Style::default().fg(Color::White);

        let padding = 5;
        let offset = chunks[2]
            .height
            .checked_sub(padding)
            .and_then(|height| app.selected.checked_sub(height as usize))
            .unwrap_or(0);
        let rows = app.items.iter().skip(offset).enumerate().map(|(i, item)| {
            let time = format!("{}:{}", &item.time[..2], &item.time[2..]);
            let data = vec![time, item.subject.clone(), item.genre.clone()];
            if Some(i) == app.selected.checked_sub(offset) {
                Row::StyledData(data.into_iter(), selected_style)
            } else {
                Row::StyledData(data.into_iter(), normal_style)
            }
        });

        Block::default()
            .style(Style::default().bg(Color::White))
            .render(&mut f, size);

        Paragraph::new([Text::raw("좌,우: 요일이동 | r: 새로고침 | q: 종료")].iter()).block(
            Block::default()
                .title("단축키")
                .borders(Borders::ALL)
            )
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, chunks[0]);
        Tabs::default()
            .block(Block::default().borders(Borders::ALL).title("요일"))
            .titles(&app.tabs.titles)
            .select(app.tabs.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow))
            .render(&mut f, chunks[1]);

        Table::new(header.iter(), rows)
            .block(
                Block::default()
                    .title("애니목록")
                    .borders(Borders::ALL)
            )
            .widths(&[
                Constraint::Length(7),
                Constraint::Percentage(100),
                Constraint::Length(15),
            ])
            .render(&mut f, chunks[2]);
    })
}