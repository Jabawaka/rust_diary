use std::time::Instant;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Stylize, Margin},
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span, Text},
    widgets::{
        calendar::{CalendarEventStore, Monthly},
        Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap
    },
    Frame
};

use time::format_description;

use crate::app::{App, CurrScreen, ZoomLevel, EditValue};


pub fn ui(frame: &mut Frame, app: &mut App) {
    // ------ SPLIT INTO NECESSARY AREAS ------
    // Split the frame into the vertical chunks we want: header, entry, graphs and footer (with calendar)
    let [header_area, main_area, graph_area, instructions_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(20),
            Constraint::Length(3),
        ])
    .areas(frame.area());

    // Split header area
    let [entry_area, calendar_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(27)
        ])
    .areas(main_area);

    // Split graph area
    let [weight_graph_area, waist_graph_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
    .areas(graph_area);

    // ------ RENDER TITLE ------
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

    // ------ RENDER GRAPHS ------
    // Weight graph
    let weight_pts = app.get_weights(app.curr_date, app.zoom);

    let mut labels = Vec::new();
    match app.zoom {
        ZoomLevel::Day => {
            let mut date = app.curr_date.prev_occurrence(app.curr_date.weekday());

            while date <= app.curr_date {
                labels.push(date.day().to_string());
                date = date.next_day().unwrap();
            }
        }
        ZoomLevel::Week => {
        }
        ZoomLevel::Month => {
        }
    }

    let x_axis = Axis::default()
        .style(Style::default())
        .labels(labels)
        .bounds([0.0, 7.0]);

    let y_axis = Axis::default()
        .style(Style::default())
        .labels(["60.0", "70.0", "80.0", "90.0", "100.0"])
        .bounds([60.0, 100.0]);

    let weight_data = Dataset::default()
        .marker(Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&weight_pts);

    let weight_chart = Chart::new(vec![weight_data])
        .block(Block::new().title("Weight [kg]"))
        .x_axis(x_axis.clone())
        .y_axis(y_axis);

    frame.render_widget(weight_chart, weight_graph_area.inner(Margin {vertical: 1, horizontal: 1}));

    // Waist graph
    let waist_pts = app.get_waists(app.curr_date, ZoomLevel::Day);

    let y_axis = Axis::default()
        .style(Style::default())
        .labels(["60.0", "70.0", "80.0", "90.0", "100.0"])
        .bounds([60.0, 100.0]);

    let waist_data = Dataset::default()
        .marker(Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&waist_pts);

    let waist_chart = Chart::new(vec![waist_data])
        .block(Block::new().title("Waist [cm]"))
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(waist_chart, waist_graph_area.inner(Margin {vertical: 1, horizontal: 1}));

    // ------ RENDER CALENDAR ------
    let mut calendar_style = CalendarEventStore::today(Style::default());

    calendar_style.add(app.curr_date, Style::new().cyan().bold());

    let curr_month = Monthly::new(app.curr_date, calendar_style)
        .show_month_header(Style::new().bold())
        .show_weekdays_header(Style::new().italic());

    frame.render_widget(curr_month, calendar_area.inner(Margin {vertical: 1, horizontal: 1}));

    // Render areas dependent on screen of app: entry and instructions
    match app.curr_screen {
        CurrScreen::Main => {
            render_main(frame, entry_area, instructions_area, app);
        }
        CurrScreen::Editing => {
            render_edit(frame, entry_area, instructions_area, app);
        }
        CurrScreen::DiscardChanges => {
            render_edit(frame, entry_area, instructions_area, app);
            render_discard(frame, entry_area);
        }
    }
}

fn render_main(frame: &mut Frame, entry_area: Rect, instructions_area: Rect, app: &App) {
    // ------ ENTRY AREA ------
    let entry_text;
    let default_bordered_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let text_style = Style::default().fg(Color::White);

    // Render entry area
    if let Some(entry) = app.get_entry_by_date(app.curr_date) {
        let weight_str;
        if let Some(weight_kg) = entry.weight_kg {
            weight_str = weight_kg.to_string();
        } else {
            weight_str = String::from("--");
        }

        let waist_str;
        if let Some(waist_cm) = entry.waist_cm {
            waist_str = waist_cm.to_string();
        } else {
            waist_str = String::from("--");
        }

        entry_text = Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::styled(weight_str, text_style),
                Span::styled(" kg, ", text_style),
                Span::styled(waist_str, text_style),
                Span::styled(" cm", text_style)
            ]).alignment(Alignment::Right),
            Line::from(Span::raw("")),
            Line::from(Span::styled(entry.content, text_style))
        ]))
        .wrap(Wrap {trim: false})
    } else {
        entry_text = Paragraph::new(Text::styled(
            "Press 'e' to enter today's entry",
            Style::default().fg(Color::Cyan)
        ));
    }

    let text_area = entry_area.inner(Margin {
        vertical: 1,
        horizontal: 3}
    );

    frame.render_widget(default_bordered_block.clone(), entry_area);
    frame.render_widget(entry_text, text_area);

    // ------ INSTRUCTIONS AREA ------
    let instructions = Paragraph::new(Span::styled("'hjkl', '◄▲▼►' - Navigate date  |  'e' - Enter edit mode  |  '<>' - Graph zoom", text_style))
        .alignment(Alignment::Center)
        .block(default_bordered_block);

    frame.render_widget(instructions, instructions_area);
}

fn render_edit(frame: &mut Frame, entry_area: Rect, instructions_area: Rect, app: &App) {
    // ------ ENTRY AREA ------
    let default_bordered_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let text_style = Style::default().fg(Color::White);

    // Entry text
    let entry_str;
    if app.edit_value == EditValue::Content {
        entry_str = app.edit_content.to_edit_string();
    } else {
        entry_str = app.edit_content.to_final_string();
    }

    // Weight text
    let mut weight_str;
    let kg_str = String::from(" kg, ");
    if app.edit_value == EditValue::Weight {
        weight_str = app.edit_weight.to_edit_string();
    } else {
        weight_str = app.edit_weight.to_final_string();
    }
    weight_str.push_str(&kg_str);

    // Waist text
    let mut waist_str;
    let cm_str = String::from(" cm");
    if app.edit_value == EditValue::Waist {
        waist_str = app.edit_waist.to_edit_string();
    } else {
        waist_str = app.edit_waist.to_final_string();
    }
    waist_str.push_str(&cm_str);

    let entry_para = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::styled(weight_str, text_style),
            Span::styled(waist_str, text_style)
        ]).alignment(Alignment::Right),
        Line::from(Span::raw("")),
        Line::from(Span::styled(entry_str, text_style))
    ]))
    .wrap(Wrap {trim: false});

    let text_area = entry_area.inner(Margin {
        vertical: 1,
        horizontal: 3}
    );

    frame.render_widget(default_bordered_block.clone(), entry_area);
    frame.render_widget(entry_para, text_area);

    // ------ INSTRUCTIONS AREA ------
    let instructions = Paragraph::new(Span::styled("Type to enter values  |  Tab - Cycle through fields  |  Enter - Save and exit edit mode", text_style))
        .alignment(Alignment::Center)
        .block(default_bordered_block);

    frame.render_widget(instructions, instructions_area);
}

fn render_discard(frame: &mut Frame, entry_area: Rect) {
    let dialogue_rect = centered_rect(50, 30, entry_area);
    let text_style = Style::default().fg(Color::White);

    let discard_msg = Paragraph::new(vec![
        Line::from(Span::raw("")),
        Line::from(Span::styled("Do you want to discard all changes?", text_style)).alignment(Alignment::Center),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Y/N", text_style)).alignment(Alignment::Center)
    ]).block(Block::bordered());

    frame.render_widget(discard_msg, dialogue_rect);
}

fn centered_rect(perc_x: u16, perc_y: u16, r: Rect) -> Rect {
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