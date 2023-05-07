use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct UserPreferences {
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
struct User {
	//let id = Uuid::new_v7();
	id: Uuid,
	display_name: String,
	primary_email: String,
	backup_emails: Vec<String>,
	password_hash: String,
	created_at: DateTime<Utc>,
	last_login: DateTime<Utc>,
	last_active: DateTime<Utc>,
	invited_by: Uuid,
	invite_code: String,
	//sqlx::types::Json<Book>
	ui_preferences: UserPreferences,
}