use anyhow::Result;
use chrono::{DateTime, Utc};
use tokio_postgres::Client;
use uuid::Uuid;

crate::model! {
    table = "tickets";
    params = TicketParams;
    struct Ticket {
        @data
        pub room_id: Uuid,
        pub submitter_id: Uuid,
        pub title: String,
        pub description: String,
        pub categories: Vec<String>,
        pub priority: Option<String>,
        pub status: String,
    }
}

/// Denormalised ticket row with submitter username, used for list display.
pub struct TicketRow {
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
    pub updated: DateTime<Utc>,
}

impl Ticket {
    /// All tickets for a room, joined with submitter username.
    /// When `include_closed` is false, only `status='open'` rows are returned.
    /// Results ordered newest-first (client sorts by priority/name as needed).
    pub async fn list_for_room(
        client: &Client,
        room_id: Uuid,
        include_closed: bool,
    ) -> Result<Vec<TicketRow>> {
        let rows = client
            .query(
                "SELECT t.id, t.room_id, t.submitter_id, u.username AS submitter_username,
                        t.title, t.description, t.categories, t.priority, t.status,
                        t.created, t.updated
                 FROM tickets t
                 JOIN users u ON u.id = t.submitter_id
                 WHERE t.room_id = $1
                   AND ($2 OR t.status = 'open')
                 ORDER BY t.created DESC",
                &[&room_id, &include_closed],
            )
            .await?;
        Ok(rows.into_iter().map(TicketRow::from_row).collect())
    }

    /// Count tickets created by this user today (UTC day). Used for rate limiting.
    pub async fn count_today_for_user(client: &Client, submitter_id: Uuid) -> Result<i64> {
        let row = client
            .query_one(
                "SELECT COUNT(*) FROM tickets
                 WHERE submitter_id = $1
                   AND created >= date_trunc('day', current_timestamp AT TIME ZONE 'UTC')",
                &[&submitter_id],
            )
            .await?;
        Ok(row.get(0))
    }

    /// Distinct category tags used in any ticket in a room (for autocomplete).
    pub async fn list_categories(client: &Client, room_id: Uuid) -> Result<Vec<String>> {
        let rows = client
            .query(
                "SELECT DISTINCT unnest(categories) AS cat
                 FROM tickets WHERE room_id = $1 ORDER BY cat",
                &[&room_id],
            )
            .await?;
        Ok(rows.into_iter().map(|r| r.get::<_, String>(0)).collect())
    }

    /// Create a new ticket. Returns the created row (without submitter_username join).
    pub async fn insert(
        client: &Client,
        room_id: Uuid,
        submitter_id: Uuid,
        title: impl Into<String>,
        description: impl Into<String>,
        categories: Vec<String>,
    ) -> Result<Self> {
        let row = client
            .query_one(
                "INSERT INTO tickets (room_id, submitter_id, title, description, categories)
                 VALUES ($1, $2, $3, $4, $5) RETURNING *",
                &[
                    &room_id,
                    &submitter_id,
                    &title.into(),
                    &description.into(),
                    &categories,
                ],
            )
            .await?;
        Ok(Self::from(row))
    }

    /// Update title/description/categories as the original submitter (non-mod path).
    pub async fn update_by_submitter(
        client: &Client,
        id: Uuid,
        submitter_id: Uuid,
        title: impl Into<String>,
        description: impl Into<String>,
        categories: Vec<String>,
    ) -> Result<bool> {
        let n = client
            .execute(
                "UPDATE tickets
                 SET title=$3, description=$4, categories=$5, updated=current_timestamp
                 WHERE id=$1 AND submitter_id=$2",
                &[
                    &id,
                    &submitter_id,
                    &title.into(),
                    &description.into(),
                    &categories,
                ],
            )
            .await?;
        Ok(n > 0)
    }

    /// Full update including priority and status (mod-only path).
    pub async fn update_mod(
        client: &Client,
        id: Uuid,
        title: impl Into<String>,
        description: impl Into<String>,
        categories: Vec<String>,
        priority: Option<String>,
        status: impl Into<String>,
    ) -> Result<bool> {
        let n = client
            .execute(
                "UPDATE tickets
                 SET title=$2, description=$3, categories=$4, priority=$5, status=$6,
                     updated=current_timestamp
                 WHERE id=$1",
                &[
                    &id,
                    &title.into(),
                    &description.into(),
                    &categories,
                    &priority,
                    &status.into(),
                ],
            )
            .await?;
        Ok(n > 0)
    }

    /// Set priority only (mod-only). `None` clears the priority field.
    pub async fn set_priority(
        client: &Client,
        id: Uuid,
        priority: Option<&str>,
    ) -> Result<bool> {
        let n = client
            .execute(
                "UPDATE tickets SET priority=$2, updated=current_timestamp WHERE id=$1",
                &[&id, &priority],
            )
            .await?;
        Ok(n > 0)
    }

    /// Toggle between 'open' and 'closed' (mod-only).
    pub async fn set_status(
        client: &Client,
        id: Uuid,
        status: &str,
    ) -> Result<bool> {
        let n = client
            .execute(
                "UPDATE tickets SET status=$2, updated=current_timestamp WHERE id=$1",
                &[&id, &status],
            )
            .await?;
        Ok(n > 0)
    }
}

impl TicketRow {
    pub(crate) fn from_row(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            room_id: row.get("room_id"),
            submitter_id: row.get("submitter_id"),
            submitter_username: row.get("submitter_username"),
            title: row.get("title"),
            description: row.get("description"),
            categories: row.get("categories"),
            priority: row.get("priority"),
            status: row.get("status"),
            created: row.get("created"),
            updated: row.get("updated"),
        }
    }

    /// Sort key for priority-descending order (urgent = 6, none = 0).
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

    pub fn is_open(&self) -> bool {
        self.status == "open"
    }
}
