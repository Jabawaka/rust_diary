use std::default;

use ratatui::{
    prelude::Stylize,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph,
        calendar::{Monthly, CalendarEventStore
    }},
    Frame
};

use time::{OffsetDateTime, format_description};

use crate::app::{App, CurrScreen};


pub fn ui(frame: &mut Frame, app: &App) {
    // Split the frame into the vertical chunks we want: header, main and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(9),
            Constraint::Length(2),
        ])
    .split(frame.area());

    let header_area = chunks[0];
    let entry_area = chunks[1];
    let calendar_graph_area = chunks[2];
    let footer_area = chunks[3];

    // Render title
    let default_bordered_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let date_format = format_description::parse("[day]/[month]/[year]").unwrap();
    let today_string = app.curr_date.format(&date_format).unwrap();

    let title = Paragraph::new(Text::styled(
        format!("-- Diary {} --", today_string),
        Style::default().fg(Color::White)
    )).alignment(Alignment::Center).block(default_bordered_block.clone());

    frame.render_widget(title, header_area);

    // Render bottom bar
    let calendar_graph_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(21),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
    .split(calendar_graph_area);

    // Calendar with month of current entry
    let curr_month = Monthly::new(app.curr_date, CalendarEventStore::today(Style::new().red().bold()))
        .show_month_header(Style::new().bold())
        .show_weekdays_header(Style::new().italic());

    frame.render_widget(default_bordered_block, calendar_graph_area);

    // Render areas dependent on screen of app: entry_area and footer
    match app.curr_screen {
        CurrScreen::Main => {
            render_main(frame, entry_area, footer_area, app);
        }
        CurrScreen::Editing => {
            render_edit(frame, entry_area, footer_area, app);
        }
    }

    frame.render_widget(curr_month, calendar_graph_chunks[0]);
}

fn render_main(frame: &mut Frame, entry_area: Rect, footer: Rect, app: &App) {
    let default_bordered_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // Render entry area
    if let Some(entry) = app.get_entry_by_date(app.curr_date) {
    } else {
        let entry_text = Paragraph::new(Text::styled(
            "Press 'e' to enter today's entry",
            Style::default().fg(Color::Cyan)
        )).block(default_bordered_block);

        frame.render_widget(entry_text, entry_area);
    }
}

fn render_edit(frame: &mut Frame, entry_area: Rect, footer: Rect, app: &App) {
}

fn centred_rect(perc_x: u16, perc_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - perc_y) / 2),
            Constraint::Percentage(perc_y),
            Constraint::Percentage((100 - perc_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - perc_x) / 2),
            Constraint::Percentage(perc_x),
            Constraint::Percentage((100 - perc_x) / 2),
        ])
        .split(popup_layout[1])[1]
}