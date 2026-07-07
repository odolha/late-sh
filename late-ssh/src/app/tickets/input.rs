use crate::app::{
    common::{
        primitives::Banner,
        textarea_input::{EditOutcome, handle_multiline_edit, handle_single_line_edit},
    },
    input::ParsedInput,
    state::App,
};

use super::state::{FormField, SortOrder, TicketView};

pub(crate) fn handle_input(app: &mut App, event: ParsedInput) {
    match app.ticket_modal_state.view {
        TicketView::List => handle_list_input(app, event),
        TicketView::Form => handle_form_input(app, event),
    }
}

pub(crate) fn handle_escape(app: &mut App) {
    match app.ticket_modal_state.view {
        TicketView::Form => {
            app.ticket_modal_state.back_to_list();
        }
        TicketView::List => {
            app.ticket_modal_state.close();
            app.show_ticket_modal = false;
        }
    }
}

// ── list view ────────────────────────────────────────────────────────────────

fn handle_list_input(app: &mut App, event: ParsedInput) {
    match event {
        ParsedInput::Byte(0x1B) => {
            handle_escape(app);
        }
        // navigation
        ParsedInput::Byte(b'j') | ParsedInput::Char('j') | ParsedInput::Arrow(b'B') => {
            app.ticket_modal_state.move_selection(1);
        }
        ParsedInput::Byte(b'k') | ParsedInput::Char('k') | ParsedInput::Arrow(b'A') => {
            app.ticket_modal_state.move_selection(-1);
        }
        // new ticket
        ParsedInput::Byte(b'n') | ParsedInput::Char('n') => {
            app.ticket_modal_state.open_new_form();
        }
        // history toggle
        ParsedInput::Byte(b'h') | ParsedInput::Char('h') => {
            app.ticket_modal_state.toggle_show_closed();
        }
        // sort
        ParsedInput::Byte(b'1') | ParsedInput::Char('1') => {
            app.ticket_modal_state.set_sort(SortOrder::Priority);
        }
        ParsedInput::Byte(b'2') | ParsedInput::Char('2') => {
            app.ticket_modal_state.set_sort(SortOrder::Date);
        }
        ParsedInput::Byte(b'3') | ParsedInput::Char('3') => {
            app.ticket_modal_state.set_sort(SortOrder::Name);
        }
        // edit (own ticket or mod)
        ParsedInput::Byte(b'e') | ParsedInput::Char('e') => {
            if can_edit_selected(app) {
                app.ticket_modal_state.open_edit_form();
            }
        }
        // priority cycle (mod only)
        ParsedInput::Byte(b'p') | ParsedInput::Char('p') => {
            if app.ticket_modal_state.is_staff {
                if let Some(ticket_id) = app.ticket_modal_state.selected_ticket().map(|t| t.id) {
                    let next_priority = app.ticket_modal_state.next_priority_for_selected();
                    let tx = app.ticket_modal_state.tx();
                    app.ticket_service
                        .set_priority_task(ticket_id, next_priority, tx);
                }
            }
        }
        // close/reopen (mod only)
        ParsedInput::Byte(b'c') | ParsedInput::Char('c') => {
            if app.ticket_modal_state.is_staff {
                if let Some((ticket_id, new_status)) =
                    app.ticket_modal_state.toggle_status_for_selected()
                {
                    let tx = app.ticket_modal_state.tx();
                    app.ticket_service
                        .set_status_task(ticket_id, new_status, tx);
                }
            }
        }
        _ => {}
    }
}

fn can_edit_selected(app: &App) -> bool {
    let Some(ticket) = app.ticket_modal_state.selected_ticket() else {
        return false;
    };
    app.ticket_modal_state.is_staff || ticket.submitter_id == app.user_id
}

// ── form view ────────────────────────────────────────────────────────────────

fn handle_form_input(app: &mut App, event: ParsedInput) {
    // Categories field handles Tab specially (autocomplete accept)
    if app.ticket_modal_state.form_focus == FormField::Categories {
        match event {
            ParsedInput::Byte(0x1B) => {
                app.ticket_modal_state.back_to_list();
                return;
            }
            ParsedInput::Byte(b'\t') if app.ticket_modal_state.autocomplete_visible => {
                if let Some(sug) = app.ticket_modal_state.autocomplete_matches.first().cloned() {
                    app.ticket_modal_state.accept_autocomplete(sug);
                }
                return;
            }
            ParsedInput::BackTab => {
                app.ticket_modal_state.move_form_focus(-1);
                return;
            }
            ParsedInput::Byte(b'\t') | ParsedInput::Arrow(b'B') => {
                app.ticket_modal_state.move_form_focus(1);
                return;
            }
            _ => {
                handle_categories_input(app, event);
                return;
            }
        }
    }
    match event {
        ParsedInput::Byte(0x1B) => {
            app.ticket_modal_state.back_to_list();
        }
        ParsedInput::Byte(b'\t') | ParsedInput::Arrow(b'B') => {
            app.ticket_modal_state.move_form_focus(1);
        }
        ParsedInput::BackTab | ParsedInput::Arrow(b'A') => {
            app.ticket_modal_state.move_form_focus(-1);
        }
        // Priority field navigation
        ParsedInput::Arrow(b'C') | ParsedInput::Byte(b'l') | ParsedInput::Char('l')
            if app.ticket_modal_state.form_focus == FormField::Priority =>
        {
            app.ticket_modal_state.cycle_priority(1);
        }
        ParsedInput::Arrow(b'D') | ParsedInput::Byte(b'h') | ParsedInput::Char('h')
            if app.ticket_modal_state.form_focus == FormField::Priority =>
        {
            app.ticket_modal_state.cycle_priority(-1);
        }
        ParsedInput::Byte(b'\r' | b'\n') | ParsedInput::Char('\r' | '\n')
            if app.ticket_modal_state.form_focus == FormField::Priority =>
        {
            submit_form(app);
        }
        // Priority field: ignore all other input
        _ if app.ticket_modal_state.form_focus == FormField::Priority => {}
        // Title / Description text fields
        event => {
            let (outcome, is_desc) = match app.ticket_modal_state.form_focus {
                FormField::Title => {
                    let max = super::state::TITLE_MAX;
                    let outcome = handle_single_line_edit(
                        &mut app.ticket_modal_state.title_input,
                        &event,
                        max,
                    );
                    (outcome, false)
                }
                FormField::Description => {
                    let max = super::state::DESC_MAX;
                    let outcome =
                        handle_multiline_edit(&mut app.ticket_modal_state.desc_input, &event, max);
                    (outcome, true)
                }
                _ => return,
            };
            let _ = is_desc;
            match outcome {
                EditOutcome::Submit => submit_form(app),
                EditOutcome::Cancel => app.ticket_modal_state.back_to_list(),
                EditOutcome::Handled | EditOutcome::Ignored => {}
            }
        }
    }
}

fn handle_categories_input(app: &mut App, event: ParsedInput) {
    match event {
        ParsedInput::Byte(0x7f | 0x08) => {
            app.ticket_modal_state.categories_input_pop();
        }
        ParsedInput::Byte(0x17) => {
            app.ticket_modal_state.categories_input_clear_word();
        }
        ParsedInput::Byte(b'\r' | b'\n') | ParsedInput::Char('\r' | '\n') => {
            submit_form(app);
        }
        ParsedInput::Char(c) if !c.is_control() => {
            app.ticket_modal_state.categories_input_push(c);
        }
        ParsedInput::Byte(b) if b.is_ascii_graphic() || b == b' ' => {
            app.ticket_modal_state.categories_input_push(b as char);
        }
        _ => {}
    }
}

fn submit_form(app: &mut App) {
    let submit = match app.ticket_modal_state.form_submit_data() {
        Ok(s) => s,
        Err(msg) => {
            app.banner = Some(Banner::error(&msg));
            return;
        }
    };
    let Some(room_id) = app.ticket_modal_state.room_id() else {
        return;
    };
    let tx = app.ticket_modal_state.tx();
    match app.ticket_modal_state.form_mode {
        super::state::FormMode::New => {
            app.ticket_service.create_task(
                room_id,
                app.user_id,
                submit.title,
                submit.description,
                submit.categories,
                tx,
            );
        }
        super::state::FormMode::Edit => {
            let Some(id) = app.ticket_modal_state.edit_ticket_id() else {
                return;
            };
            app.ticket_service.update_task(
                id,
                app.user_id,
                app.ticket_modal_state.is_staff,
                submit.title,
                submit.description,
                submit.categories,
                submit.priority,
                submit.status,
                tx,
            );
        }
    }
    app.banner = Some(Banner::success("Saving..."));
}
