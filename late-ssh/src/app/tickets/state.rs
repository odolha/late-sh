use chrono::{DateTime, Utc};
use late_core::models::ticket::{Ticket, TicketRow};
use ratatui_textarea::{TextArea, WrapMode};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::app::common::{
    composer::{new_themed_textarea, set_themed_textarea_cursor_visible},
    primitives::Banner,
};

// ── constants ────────────────────────────────────────────────────────────────

pub(crate) const TITLE_MAX: usize = 100;
pub(crate) const DESC_MAX: usize = 2000;
pub(crate) const CATEGORIES_MAX: usize = 200;

const PRIORITY_CYCLE: &[Option<&str>] = &[
    None,
    Some("very_low"),
    Some("low"),
    Some("medium"),
    Some("high"),
    Some("very_high"),
    Some("urgent"),
];

// ── events from async tasks ──────────────────────────────────────────────────

pub(crate) enum TicketEvent {
    Loaded {
        tickets: Vec<TicketRow>,
        categories: Vec<String>,
    },
    Created {
        ticket: Ticket,
        categories: Vec<String>,
    },
    Updated(Uuid),
    PriorityChanged(Uuid, Option<String>),
    StatusChanged(Uuid, String),
    Error(String),
}

// ── views ────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TicketView {
    List,
    Form,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FormMode {
    New,
    Edit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SortOrder {
    Priority,
    Date,
    Name,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FormField {
    Title,
    Description,
    Categories,
    Priority,
}

// ── cached ticket entry ──────────────────────────────────────────────────────

pub(crate) struct TicketEntry {
    pub id: Uuid,
    pub room_id: Uuid,
    pub submitter_id: Uuid,
    pub submitter_username: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub priority: Option<String>,
    pub status: String,
    pub created: DateTime<Utc>,
}

impl TicketEntry {
    fn from_row(row: TicketRow) -> Self {
        Self {
            id: row.id,
            room_id: row.room_id,
            submitter_id: row.submitter_id,
            submitter_username: row.submitter_username,
            title: row.title,
            description: row.description,
            categories: row.categories,
            priority: row.priority,
            status: row.status,
            created: row.created,
        }
    }

    pub fn is_open(&self) -> bool {
        self.status == "open"
    }

    pub fn priority_sort_key(&self) -> u8 {
        match self.priority.as_deref() {
            Some("urgent") => 6,
            Some("very_high") => 5,
            Some("high") => 4,
            Some("medium") => 3,
            Some("low") => 2,
            Some("very_low") => 1,
            _ => 0,
        }
    }

    pub fn priority_label(&self) -> &str {
        match self.priority.as_deref() {
            Some("urgent") => "urgent",
            Some("very_high") => "very high",
            Some("high") => "high",
            Some("medium") => "medium",
            Some("low") => "low",
            Some("very_low") => "very low",
            Some(_) | None => "—",
        }
    }
}

// ── main state ───────────────────────────────────────────────────────────────

pub(crate) struct TicketModalState {
    // identity
    room_id: Option<Uuid>,
    room_name: String,
    user_id: Uuid,
    pub is_staff: bool,

    // list view
    pub view: TicketView,
    pub sort: SortOrder,
    pub show_closed: bool,
    pub tickets: Vec<TicketEntry>,
    pub selected: usize,
    pub loading: bool,
    pub error: Option<String>,

    // form view
    pub form_mode: FormMode,
    edit_ticket_id: Option<Uuid>,
    pub form_focus: FormField,
    pub title_input: TextArea<'static>,
    pub desc_input: TextArea<'static>,
    pub categories_raw: String,
    pub priority_index: usize,

    // category autocomplete
    pub known_categories: Vec<String>,
    pub autocomplete_visible: bool,
    pub autocomplete_matches: Vec<String>,

    // async channel
    tx: mpsc::Sender<TicketEvent>,
    rx: mpsc::Receiver<TicketEvent>,
}

impl TicketModalState {
    pub(crate) fn new(user_id: Uuid) -> Self {
        let (tx, rx) = mpsc::channel(32);
        Self {
            room_id: None,
            room_name: String::new(),
            user_id,
            is_staff: false,
            view: TicketView::List,
            sort: SortOrder::Priority,
            show_closed: false,
            tickets: Vec::new(),
            selected: 0,
            loading: false,
            error: None,
            form_mode: FormMode::New,
            edit_ticket_id: None,
            form_focus: FormField::Title,
            title_input: new_title_input(),
            desc_input: new_desc_input(),
            categories_raw: String::new(),
            priority_index: 0,
            known_categories: Vec::new(),
            autocomplete_visible: false,
            autocomplete_matches: Vec::new(),
            tx,
            rx,
        }
    }

    pub(crate) fn tx(&self) -> mpsc::Sender<TicketEvent> {
        self.tx.clone()
    }

    pub(crate) fn is_open(&self) -> bool {
        self.room_id.is_some()
    }

    pub(crate) fn room_id(&self) -> Option<Uuid> {
        self.room_id
    }

    pub(crate) fn room_name(&self) -> &str {
        &self.room_name
    }

    pub(crate) fn edit_ticket_id(&self) -> Option<Uuid> {
        self.edit_ticket_id
    }

    /// Open modal in list view, kick off background ticket load.
    pub(crate) fn open(&mut self, room_id: Uuid, room_name: impl Into<String>, is_staff: bool) {
        self.room_id = Some(room_id);
        self.room_name = room_name.into();
        self.is_staff = is_staff;
        self.view = TicketView::List;
        self.selected = 0;
        self.show_closed = false;
        self.sort = SortOrder::Priority;
        self.tickets.clear();
        self.error = None;
        self.loading = true;
        self.known_categories.clear();
        self.autocomplete_visible = false;
        self.autocomplete_matches.clear();
    }

    pub(crate) fn close(&mut self) {
        self.room_id = None;
        self.loading = false;
        self.error = None;
    }

    /// Open the new-ticket form.
    pub(crate) fn open_new_form(&mut self) {
        self.view = TicketView::Form;
        self.form_mode = FormMode::New;
        self.edit_ticket_id = None;
        self.form_focus = FormField::Title;
        self.title_input = new_title_input();
        self.desc_input = new_desc_input();
        self.categories_raw = String::new();
        self.priority_index = 0;
        self.autocomplete_visible = false;
        self.autocomplete_matches.clear();
        self.sync_form_cursor();
    }

    /// Open the edit form pre-populated with the selected ticket's data.
    pub(crate) fn open_edit_form(&mut self) {
        let Some(ticket) = self.selected_ticket() else {
            return;
        };
        let id = ticket.id;
        let title = ticket.title.clone();
        let desc = ticket.description.clone();
        let cats = ticket.categories.join(", ");
        let priority_index = self.priority_index_for(ticket.priority.as_deref());
        self.view = TicketView::Form;
        self.form_mode = FormMode::Edit;
        self.edit_ticket_id = Some(id);
        self.form_focus = FormField::Title;
        self.title_input = new_title_input();
        self.title_input.insert_str(&title);
        self.desc_input = new_desc_input();
        self.desc_input.insert_str(&desc);
        self.categories_raw = cats;
        self.priority_index = priority_index;
        self.autocomplete_visible = false;
        self.autocomplete_matches.clear();
        self.sync_form_cursor();
    }

    pub(crate) fn back_to_list(&mut self) {
        self.view = TicketView::List;
        self.autocomplete_visible = false;
        self.autocomplete_matches.clear();
        self.error = None;
    }

    // ── navigation ──────────────────────────────────────────────────────────

    pub(crate) fn visible_tickets(&self) -> Vec<&TicketEntry> {
        let mut visible: Vec<&TicketEntry> = self
            .tickets
            .iter()
            .filter(|t| self.show_closed || t.is_open())
            .collect();
        match self.sort {
            SortOrder::Priority => {
                visible.sort_by(|a, b| {
                    b.priority_sort_key()
                        .cmp(&a.priority_sort_key())
                        .then(b.created.cmp(&a.created))
                });
            }
            SortOrder::Date => {
                visible.sort_by(|a, b| b.created.cmp(&a.created));
            }
            SortOrder::Name => {
                visible.sort_by(|a, b| a.title.cmp(&b.title));
            }
        }
        visible
    }

    pub(crate) fn selected_ticket(&self) -> Option<&TicketEntry> {
        self.visible_tickets().into_iter().nth(self.selected)
    }

    pub(crate) fn move_selection(&mut self, delta: isize) {
        let count = self.visible_ticket_count();
        if count == 0 {
            return;
        }
        self.selected = (self.selected as isize + delta).rem_euclid(count as isize) as usize;
    }

    pub(crate) fn visible_ticket_count(&self) -> usize {
        self.visible_tickets().len()
    }

    pub(crate) fn set_sort(&mut self, sort: SortOrder) {
        self.sort = sort;
        self.selected = 0;
    }

    pub(crate) fn toggle_show_closed(&mut self) {
        self.show_closed = !self.show_closed;
        self.selected = 0;
    }

    // ── form ────────────────────────────────────────────────────────────────

    pub(crate) fn move_form_focus(&mut self, delta: isize) {
        let fields: &[FormField] = if self.is_staff {
            &[
                FormField::Title,
                FormField::Description,
                FormField::Categories,
                FormField::Priority,
            ]
        } else {
            &[
                FormField::Title,
                FormField::Description,
                FormField::Categories,
            ]
        };
        let pos = fields
            .iter()
            .position(|f| *f == self.form_focus)
            .unwrap_or(0);
        let next = (pos as isize + delta).rem_euclid(fields.len() as isize) as usize;
        self.form_focus = fields[next];
        self.sync_form_cursor();
        if self.form_focus != FormField::Categories {
            self.autocomplete_visible = false;
        }
    }

    pub(crate) fn cycle_priority(&mut self, delta: isize) {
        self.priority_index = (self.priority_index as isize + delta)
            .rem_euclid(PRIORITY_CYCLE.len() as isize) as usize;
    }

    pub(crate) fn current_priority(&self) -> Option<String> {
        PRIORITY_CYCLE[self.priority_index].map(|s| s.to_string())
    }

    pub(crate) fn current_priority_label(&self) -> &str {
        match PRIORITY_CYCLE[self.priority_index] {
            Some("urgent") => "urgent",
            Some("very_high") => "very high",
            Some("high") => "high",
            Some("medium") => "medium",
            Some("low") => "low",
            Some("very_low") => "very low",
            _ => "— none",
        }
    }

    pub(crate) fn categories_input_push(&mut self, c: char) {
        if self.categories_raw.len() < CATEGORIES_MAX {
            self.categories_raw.push(c);
            self.update_autocomplete();
        }
    }

    pub(crate) fn categories_input_pop(&mut self) {
        self.categories_raw.pop();
        self.update_autocomplete();
    }

    pub(crate) fn categories_input_clear_word(&mut self) {
        let trimmed = self
            .categories_raw
            .trim_end_matches(|c: char| c != ',' && c != ' ');
        self.categories_raw = trimmed.trim_end().to_string();
        self.update_autocomplete();
    }

    /// Insert an autocomplete suggestion (replaces last tag being typed).
    pub(crate) fn accept_autocomplete(&mut self, suggestion: String) {
        let before_last = self
            .categories_raw
            .rsplit_once(',')
            .map(|(before, _)| format!("{before}, "))
            .unwrap_or_default();
        self.categories_raw = format!("{before_last}{suggestion}, ");
        self.autocomplete_visible = false;
        self.autocomplete_matches.clear();
    }

    fn update_autocomplete(&mut self) {
        let last_tag = self
            .categories_raw
            .split(',')
            .last()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        if last_tag.is_empty() {
            self.autocomplete_visible = false;
            self.autocomplete_matches.clear();
            return;
        }
        self.autocomplete_matches = self
            .known_categories
            .iter()
            .filter(|cat| cat.to_ascii_lowercase().starts_with(&last_tag) && *cat != &last_tag)
            .cloned()
            .collect();
        self.autocomplete_visible = !self.autocomplete_matches.is_empty();
    }

    /// Parse `categories_raw` into a deduplicated, lowercased tag list.
    pub(crate) fn parsed_categories(&self) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        self.categories_raw
            .split(',')
            .map(|s| s.trim().to_ascii_lowercase())
            .filter(|s| !s.is_empty() && seen.insert(s.clone()))
            .collect()
    }

    /// Validate and build the form submit payload. Returns Err with a user message.
    pub(crate) fn form_submit_data(&self) -> Result<FormSubmit, String> {
        let title = self.title_input.lines().join("").trim().to_string();
        if title.is_empty() {
            return Err("Title is required".into());
        }
        if title.len() > TITLE_MAX {
            return Err(format!("Title too long ({}/{})", title.len(), TITLE_MAX));
        }
        let description = self.desc_input.lines().join("\n").trim_end().to_string();
        if description.is_empty() {
            return Err("Description is required".into());
        }
        if description.len() > DESC_MAX {
            return Err(format!(
                "Description too long ({}/{})",
                description.len(),
                DESC_MAX
            ));
        }
        let categories = self.parsed_categories();
        let priority = if self.is_staff {
            self.current_priority()
        } else {
            None
        };
        let status = if self.is_staff && self.form_mode == FormMode::Edit {
            // status unchanged by default in edit; caller sets it explicitly
            "open".to_string()
        } else {
            "open".to_string()
        };
        Ok(FormSubmit {
            title,
            description,
            categories,
            priority,
            status,
        })
    }

    // ── mod quick actions in list view ──────────────────────────────────────

    /// Advance priority on selected ticket by one step (mod-only).
    pub(crate) fn next_priority_for_selected(&self) -> Option<String> {
        let ticket = self.selected_ticket()?;
        let idx = self.priority_index_for(ticket.priority.as_deref());
        let next_idx = (idx + 1) % PRIORITY_CYCLE.len();
        PRIORITY_CYCLE[next_idx].map(|s| s.to_string())
    }

    /// Toggle status of selected ticket (mod-only).
    pub(crate) fn toggle_status_for_selected(&self) -> Option<(Uuid, String)> {
        let ticket = self.selected_ticket()?;
        let new_status = if ticket.is_open() { "closed" } else { "open" };
        Some((ticket.id, new_status.to_string()))
    }

    // ── event drain (called from App::tick) ─────────────────────────────────

    /// Drain async results; returns an optional banner.
    pub(crate) fn tick(&mut self) -> Option<Banner> {
        let mut banner = None;
        while let Ok(event) = self.rx.try_recv() {
            match event {
                TicketEvent::Loaded {
                    tickets,
                    categories,
                } => {
                    self.tickets = tickets.into_iter().map(TicketEntry::from_row).collect();
                    self.known_categories = categories;
                    self.loading = false;
                    self.selected = self.selected.min(self.tickets.len().saturating_sub(1));
                }
                TicketEvent::Created { ticket, categories } => {
                    self.known_categories = categories;
                    let entry = TicketEntry {
                        id: ticket.id,
                        room_id: ticket.room_id,
                        submitter_id: ticket.submitter_id,
                        submitter_username: String::new(),
                        title: ticket.title,
                        description: ticket.description,
                        categories: ticket.categories,
                        priority: ticket.priority,
                        status: ticket.status,
                        created: ticket.created,
                    };
                    self.tickets.push(entry);
                    self.back_to_list();
                    banner = Some(Banner::success("Ticket submitted"));
                }
                TicketEvent::Updated(id) => {
                    let _ = id;
                    self.back_to_list();
                    banner = Some(Banner::success("Ticket updated"));
                }
                TicketEvent::PriorityChanged(id, priority) => {
                    if let Some(t) = self.tickets.iter_mut().find(|t| t.id == id) {
                        t.priority = priority;
                    }
                    if let Some(new_idx) = self.visible_tickets().iter().position(|t| t.id == id) {
                        self.selected = new_idx;
                    }
                }
                TicketEvent::StatusChanged(id, status) => {
                    if let Some(t) = self.tickets.iter_mut().find(|t| t.id == id) {
                        t.status = status;
                    }
                    self.selected = self
                        .selected
                        .min(self.visible_ticket_count().saturating_sub(1));
                }
                TicketEvent::Error(msg) => {
                    self.loading = false;
                    banner = Some(Banner::error(&msg));
                }
            }
        }
        banner
    }

    // ── helpers ─────────────────────────────────────────────────────────────

    fn sync_form_cursor(&mut self) {
        set_themed_textarea_cursor_visible(
            &mut self.title_input,
            self.form_focus == FormField::Title,
        );
        set_themed_textarea_cursor_visible(
            &mut self.desc_input,
            self.form_focus == FormField::Description,
        );
    }

    fn priority_index_for(&self, priority: Option<&str>) -> usize {
        PRIORITY_CYCLE
            .iter()
            .position(|p| *p == priority)
            .unwrap_or(0)
    }
}

// ── form submit payload ──────────────────────────────────────────────────────

pub(crate) struct FormSubmit {
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub priority: Option<String>,
    pub status: String,
}

// ── textarea constructors ────────────────────────────────────────────────────

fn new_title_input() -> TextArea<'static> {
    new_themed_textarea("Title (max 100 chars)", WrapMode::None, false)
}

fn new_desc_input() -> TextArea<'static> {
    new_themed_textarea(
        "Description (optional, Alt+Enter for newline)",
        WrapMode::Word,
        false,
    )
}
