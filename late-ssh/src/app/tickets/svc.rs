use late_core::{db::Db, models::ticket::Ticket};
use tokio::sync::mpsc;
use uuid::Uuid;

use super::state::TicketEvent;

#[derive(Clone)]
pub struct TicketService {
    db: Db,
    pub daily_limit: usize,
}

impl TicketService {
    pub fn new(db: Db, daily_limit: usize) -> Self {
        Self { db, daily_limit }
    }

    /// Load all tickets (and known categories) for the room into the modal.
    pub(crate) fn load_task(
        &self,
        room_id: Uuid,
        include_closed: bool,
        tx: mpsc::Sender<TicketEvent>,
    ) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let result: anyhow::Result<_> = async {
                let client = db.get().await?;
                let tickets = Ticket::list_for_room(&client, room_id, include_closed).await?;
                let categories = Ticket::list_categories(&client, room_id).await?;
                Ok((tickets, categories))
            }
            .await;
            match result {
                Ok((tickets, categories)) => {
                    let _ = tx
                        .send(TicketEvent::Loaded {
                            tickets,
                            categories,
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx.send(TicketEvent::Error(e.to_string())).await;
                }
            }
        });
    }

    /// Submit a new ticket with daily rate-limit check.
    pub(crate) fn create_task(
        &self,
        room_id: Uuid,
        submitter_id: Uuid,
        title: String,
        description: String,
        categories: Vec<String>,
        tx: mpsc::Sender<TicketEvent>,
    ) {
        let db = self.db.clone();
        let daily_limit = self.daily_limit;
        tokio::spawn(async move {
            let result: anyhow::Result<_> = async {
                let client = db.get().await?;
                if daily_limit > 0 {
                    let today = Ticket::count_today_for_user(&client, submitter_id).await?;
                    if today >= daily_limit as i64 {
                        return Err(anyhow::anyhow!(
                            "Daily ticket limit reached ({daily_limit}/day). Try again tomorrow."
                        ));
                    }
                }
                let ticket = Ticket::insert(
                    &client,
                    room_id,
                    submitter_id,
                    title,
                    description,
                    categories,
                )
                .await?;
                let categories = Ticket::list_categories(&client, room_id).await?;
                Ok((ticket, categories))
            }
            .await;
            match result {
                Ok((ticket, categories)) => {
                    let _ = tx.send(TicketEvent::Created { ticket, categories }).await;
                }
                Err(e) => {
                    let _ = tx.send(TicketEvent::Error(e.to_string())).await;
                }
            }
        });
    }

    /// Update an existing ticket. Staff can edit any ticket; non-staff only their own.
    pub(crate) fn update_task(
        &self,
        id: Uuid,
        submitter_id: Uuid,
        is_staff: bool,
        title: String,
        description: String,
        categories: Vec<String>,
        priority: Option<String>,
        status: String,
        tx: mpsc::Sender<TicketEvent>,
    ) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let result: anyhow::Result<bool> = async {
                let client = db.get().await?;
                if is_staff {
                    Ticket::update_mod(
                        &client,
                        id,
                        title,
                        description,
                        categories,
                        priority,
                        status,
                    )
                    .await
                } else {
                    Ticket::update_by_submitter(
                        &client,
                        id,
                        submitter_id,
                        title,
                        description,
                        categories,
                    )
                    .await
                }
            }
            .await;
            match result {
                Ok(true) => {
                    let _ = tx.send(TicketEvent::Updated(id)).await;
                }
                Ok(false) => {
                    let _ = tx
                        .send(TicketEvent::Error("Ticket not found or not yours".into()))
                        .await;
                }
                Err(e) => {
                    let _ = tx.send(TicketEvent::Error(e.to_string())).await;
                }
            }
        });
    }

    /// Cycle priority on a ticket (mod-only). Advances through the priority ladder.
    pub(crate) fn set_priority_task(
        &self,
        id: Uuid,
        priority: Option<String>,
        tx: mpsc::Sender<TicketEvent>,
    ) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let result: anyhow::Result<_> = async {
                let client = db.get().await?;
                Ticket::set_priority(&client, id, priority.as_deref()).await
            }
            .await;
            match result {
                Ok(_) => {
                    let _ = tx.send(TicketEvent::PriorityChanged(id, priority)).await;
                }
                Err(e) => {
                    let _ = tx.send(TicketEvent::Error(e.to_string())).await;
                }
            }
        });
    }

    /// Toggle status between 'open' and 'closed' (mod-only).
    pub(crate) fn set_status_task(&self, id: Uuid, status: String, tx: mpsc::Sender<TicketEvent>) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let result: anyhow::Result<_> = async {
                let client = db.get().await?;
                Ticket::set_status(&client, id, &status).await
            }
            .await;
            match result {
                Ok(_) => {
                    let _ = tx.send(TicketEvent::StatusChanged(id, status)).await;
                }
                Err(e) => {
                    let _ = tx.send(TicketEvent::Error(e.to_string())).await;
                }
            }
        });
    }

    /// Enable or disable ticket system for a room (mod-only). Returns a human-readable result.
    pub(crate) fn set_room_tickets_enabled_task(
        &self,
        room_id: Uuid,
        enabled: bool,
        result_tx: tokio::sync::mpsc::Sender<String>,
    ) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let result: anyhow::Result<_> = async {
                let client = db.get().await?;
                late_core::models::chat_room::ChatRoom::set_tickets_enabled(
                    &client, room_id, enabled,
                )
                .await
            }
            .await;
            let msg = match result {
                Ok(_) if enabled => "Tickets enabled for this room".to_string(),
                Ok(_) => "Tickets disabled for this room".to_string(),
                Err(e) => format!("Error: {e}"),
            };
            let _ = result_tx.send(msg).await;
        });
    }
}
