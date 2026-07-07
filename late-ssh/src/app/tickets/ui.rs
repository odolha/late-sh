use chrono::{DateTime, Utc};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::common::theme;

use super::state::{FormField, SortOrder, TicketEntry, TicketModalState, TicketView};

pub(crate) fn draw_modal(frame: &mut Frame, area: Rect, state: &TicketModalState) {
    if !state.is_open() {
        return;
    }
    let popup = centered_rect(area, 90, 30);
    frame.render_widget(Clear, popup);

    match state.view {
        TicketView::List => draw_list(frame, popup, state),
        TicketView::Form => draw_form(frame, popup, state),
    }
}

// ── list view ────────────────────────────────────────────────────────────────

fn draw_list(frame: &mut Frame, area: Rect, state: &TicketModalState) {
    let title = format!(" Tickets: #{} ", state.room_name());
    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(theme::TEXT_BRIGHT()),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER()))
        .style(Style::default().bg(theme::BG_CANVAS()));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let areas = Layout::vertical([
        Constraint::Length(1), // sort bar
        Constraint::Length(1), // separator
        Constraint::Min(0),    // ticket list
        Constraint::Length(1), // footer
    ])
    .split(inner);

    // sort bar
    draw_sort_bar(frame, areas[0], state);

    // list
    if state.loading {
        frame.render_widget(
            Paragraph::new("  Loading...").style(
                Style::default()
                    .fg(theme::TEXT_DIM())
                    .bg(theme::BG_CANVAS()),
            ),
            areas[2],
        );
    } else {
        draw_ticket_list(frame, areas[2], state);
    }

    // footer
    draw_list_footer(frame, areas[3], state);
}

fn draw_sort_bar(frame: &mut Frame, area: Rect, state: &TicketModalState) {
    let sorts = [
        (SortOrder::Priority, "1:priority"),
        (SortOrder::Date, "2:date"),
        (SortOrder::Name, "3:name"),
    ];
    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::raw(" "));
    for (i, (order, label)) in sorts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        let active = state.sort == *order;
        let style = if active {
            Style::default()
                .fg(theme::SUCCESS())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::TEXT_DIM())
        };
        spans.push(Span::styled(format!("[{label}]"), style));
    }
    spans.push(Span::raw("  "));
    let closed_style = if state.show_closed {
        Style::default()
            .fg(theme::AMBER())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::TEXT_DIM())
    };
    spans.push(Span::styled("[h:history]", closed_style));
    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(theme::BG_CANVAS())),
        area,
    );
}

fn draw_ticket_list(frame: &mut Frame, area: Rect, state: &TicketModalState) {
    let visible = state.visible_tickets();
    if visible.is_empty() {
        let msg = if state.show_closed {
            "  No tickets yet."
        } else {
            "  No open tickets. Press n to submit one."
        };
        frame.render_widget(
            Paragraph::new(msg).style(
                Style::default()
                    .fg(theme::TEXT_DIM())
                    .bg(theme::BG_CANVAS()),
            ),
            area,
        );
        return;
    }

    let row_height = 2u16;
    let max_visible = (area.height / row_height) as usize;
    let start = state.selected.saturating_sub(max_visible.saturating_sub(1));
    let shown = visible.iter().skip(start).enumerate();

    let mut y = area.y;
    for (i, ticket) in shown {
        let abs_idx = start + i;
        let selected = abs_idx == state.selected;
        let row_area = Rect {
            x: area.x,
            y,
            width: area.width,
            height: row_height,
        };
        if y + row_height > area.y + area.height {
            break;
        }
        draw_ticket_row(frame, row_area, ticket, selected, state.is_staff);
        y += row_height;
    }
}

fn draw_ticket_row(
    frame: &mut Frame,
    area: Rect,
    ticket: &TicketEntry,
    selected: bool,
    _is_staff: bool,
) {
    let (sel_char, title_style, bg) = if selected {
        (
            "▶",
            Style::default()
                .fg(theme::TEXT_BRIGHT())
                .add_modifier(Modifier::BOLD),
            theme::BG_CANVAS(),
        )
    } else {
        ("  ", Style::default().fg(theme::TEXT()), theme::BG_CANVAS())
    };

    let closed_dim = if !ticket.is_open() {
        Modifier::DIM
    } else {
        Modifier::empty()
    };

    // line 1: selector + title + priority badge + status
    let priority_style = priority_color(ticket.priority.as_deref());
    let priority_label = if ticket.priority.is_some() {
        format!("[{}]", ticket.priority_label())
    } else {
        String::new()
    };

    let mut line1_spans = vec![
        Span::styled(
            format!("{sel_char} "),
            Style::default().fg(theme::SUCCESS()),
        ),
        Span::styled(&ticket.title, title_style.add_modifier(closed_dim)),
    ];
    if !priority_label.is_empty() {
        line1_spans.push(Span::raw("  "));
        line1_spans.push(Span::styled(priority_label, priority_style));
    }
    if !ticket.is_open() {
        line1_spans.push(Span::raw("  "));
        line1_spans.push(Span::styled(
            "[closed]",
            Style::default().fg(theme::TEXT_DIM()),
        ));
    }

    // line 2: submitter + date + categories
    let date_str = format_date(ticket.created);
    let cats = if ticket.categories.is_empty() {
        String::new()
    } else {
        format!("  {}", ticket.categories.join(" · "))
    };
    let line2_spans = vec![
        Span::raw("   "),
        Span::styled(
            format!("@{}", ticket.submitter_username),
            Style::default().fg(theme::TEXT_DIM()),
        ),
        Span::raw(format!("  {date_str}")),
        Span::styled(cats, Style::default().fg(theme::TEXT_DIM())),
    ];

    let [row1, row2] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(area)[..]
    else {
        return;
    };

    frame.render_widget(
        Paragraph::new(Line::from(line1_spans)).style(Style::default().bg(bg)),
        row1,
    );
    frame.render_widget(
        Paragraph::new(Line::from(line2_spans)).style(Style::default().bg(bg)),
        row2,
    );
}

fn draw_list_footer(frame: &mut Frame, area: Rect, state: &TicketModalState) {
    let mut spans = vec![
        Span::styled("j/k", Style::default().fg(theme::AMBER())),
        Span::styled(" nav  ", Style::default().fg(theme::TEXT_DIM())),
        Span::styled("n", Style::default().fg(theme::SUCCESS())),
        Span::styled(" new  ", Style::default().fg(theme::TEXT_DIM())),
        Span::styled("e", Style::default().fg(theme::AMBER())),
        Span::styled(" edit  ", Style::default().fg(theme::TEXT_DIM())),
        Span::styled("Esc", Style::default().fg(theme::ERROR())),
        Span::styled(" close", Style::default().fg(theme::TEXT_DIM())),
    ];
    if state.is_staff {
        spans.push(Span::styled(
            "  [mod] ",
            Style::default().fg(theme::TEXT_DIM()),
        ));
        spans.push(Span::styled("p", Style::default().fg(theme::AMBER())));
        spans.push(Span::styled(
            " priority  ",
            Style::default().fg(theme::TEXT_DIM()),
        ));
        spans.push(Span::styled("c", Style::default().fg(theme::AMBER())));
        spans.push(Span::styled(
            " close/reopen",
            Style::default().fg(theme::TEXT_DIM()),
        ));
    }
    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(theme::BG_CANVAS())),
        area,
    );
}

// ── form view ────────────────────────────────────────────────────────────────

fn draw_form(frame: &mut Frame, area: Rect, state: &TicketModalState) {
    let mode_label = match state.form_mode {
        super::state::FormMode::New => "New Ticket",
        super::state::FormMode::Edit => "Edit Ticket",
    };
    let title = format!(" {} ", mode_label);
    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(theme::TEXT_BRIGHT()),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER()))
        .style(Style::default().bg(theme::BG_CANVAS()));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let priority_row = if state.is_staff {
        Constraint::Length(3)
    } else {
        Constraint::Length(0)
    };
    let areas = Layout::vertical([
        Constraint::Length(1), // hint
        Constraint::Length(3), // title
        Constraint::Length(8), // description
        Constraint::Length(3), // categories
        priority_row,          // priority (staff only)
    ])
    .split(inner);

    // hint
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Enter", Style::default().fg(theme::SUCCESS())),
            Span::styled(" save  ", Style::default().fg(theme::TEXT_DIM())),
            Span::styled("Tab", Style::default().fg(theme::AMBER())),
            Span::styled(" next field  ", Style::default().fg(theme::TEXT_DIM())),
            Span::styled("Esc", Style::default().fg(theme::ERROR())),
            Span::styled(" back", Style::default().fg(theme::TEXT_DIM())),
        ]))
        .style(Style::default().bg(theme::BG_CANVAS())),
        areas[0],
    );

    // title field
    draw_textarea_field(
        frame,
        areas[1],
        &format!(
            "Title {}/{}",
            state.title_input.lines().join("").len(),
            super::state::TITLE_MAX
        ),
        &state.title_input,
        state.form_focus == FormField::Title,
    );

    // description field
    draw_textarea_field(
        frame,
        areas[2],
        &format!("Description (Alt+Enter: newline)"),
        &state.desc_input,
        state.form_focus == FormField::Description,
    );

    // categories field
    draw_categories_field(frame, areas[3], state);

    // priority field (staff only)
    if state.is_staff && areas[4].height > 0 {
        draw_priority_field(frame, areas[4], state);
    }
}

fn draw_textarea_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    textarea: &ratatui_textarea::TextArea<'static>,
    focused: bool,
) {
    let border = if focused {
        theme::BORDER_ACTIVE()
    } else {
        theme::BORDER()
    };
    let block = Block::default()
        .title(Span::styled(
            format!(" {label} "),
            Style::default().fg(if focused {
                theme::TEXT_BRIGHT()
            } else {
                theme::TEXT_DIM()
            }),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border))
        .style(Style::default().bg(theme::BG_CANVAS()));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(textarea, inner);
}

fn draw_categories_field(frame: &mut Frame, area: Rect, state: &TicketModalState) {
    let focused = state.form_focus == FormField::Categories;
    let border = if focused {
        theme::BORDER_ACTIVE()
    } else {
        theme::BORDER()
    };
    let block = Block::default()
        .title(Span::styled(
            " Categories (comma-separated tags) ",
            Style::default().fg(if focused {
                theme::TEXT_BRIGHT()
            } else {
                theme::TEXT_DIM()
            }),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border))
        .style(Style::default().bg(theme::BG_CANVAS()));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Raw input text with cursor
    let cursor_str = if focused { "█" } else { "" };
    let display = format!("{}{}", state.categories_raw, cursor_str);
    frame.render_widget(
        Paragraph::new(display).style(Style::default().fg(theme::TEXT()).bg(theme::BG_CANVAS())),
        inner,
    );

    // Autocomplete suggestions overlay
    if focused && state.autocomplete_visible && !state.autocomplete_matches.is_empty() {
        let max_shown = state.autocomplete_matches.len().min(4);
        let ac_area = Rect {
            x: inner.x,
            y: inner.y + 1,
            width: inner.width.min(30),
            height: max_shown as u16,
        };
        if ac_area.y + ac_area.height <= area.y + area.height + 5 {
            frame.render_widget(Clear, ac_area);
            let lines: Vec<Line> = state
                .autocomplete_matches
                .iter()
                .take(max_shown)
                .enumerate()
                .map(|(i, cat)| {
                    let style = if i == 0 {
                        Style::default().fg(theme::BG_CANVAS()).bg(theme::SUCCESS())
                    } else {
                        Style::default().fg(theme::TEXT()).bg(theme::BG_HIGHLIGHT())
                    };
                    Line::from(Span::styled(format!(" {cat} "), style))
                })
                .collect();
            frame.render_widget(
                Paragraph::new(lines).style(Style::default().bg(theme::BG_HIGHLIGHT())),
                ac_area,
            );
        }
    }
}

fn draw_priority_field(frame: &mut Frame, area: Rect, state: &TicketModalState) {
    let focused = state.form_focus == FormField::Priority;
    let border = if focused {
        theme::BORDER_ACTIVE()
    } else {
        theme::BORDER()
    };
    let block = Block::default()
        .title(Span::styled(
            " Priority (←/→) ",
            Style::default().fg(if focused {
                theme::TEXT_BRIGHT()
            } else {
                theme::TEXT_DIM()
            }),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border))
        .style(Style::default().bg(theme::BG_CANVAS()));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let label = state.current_priority_label();
    let style = priority_color(state.current_priority().as_deref());
    frame.render_widget(
        Paragraph::new(format!(" {label}"))
            .style(Style::default().bg(theme::BG_CANVAS()))
            .style(style),
        inner,
    );
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn priority_color(priority: Option<&str>) -> Style {
    match priority {
        Some("urgent") => Style::default()
            .fg(theme::ERROR())
            .add_modifier(Modifier::BOLD),
        Some("very_high") => Style::default().fg(theme::ERROR()),
        Some("high") => Style::default().fg(theme::AMBER()),
        Some("medium") => Style::default().fg(theme::SUCCESS()),
        Some("low") | Some("very_low") => Style::default().fg(theme::TEXT_DIM()),
        _ => Style::default().fg(theme::TEXT_DIM()),
    }
}

fn format_date(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d").to_string()
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    }
}
