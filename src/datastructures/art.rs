use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
struct Art {
	id: Uuid,
	uploader: Uuid, // Deliberately not using 'author' here because there may be many contributors to a piece.
	created_at: DateTime<Utc>,
}