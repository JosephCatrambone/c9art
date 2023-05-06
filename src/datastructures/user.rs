use serde::{Deserialize, Serialize};
use std::time::Instant;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Sync)]
struct UserPreferences {
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow, Sync)]
struct User {
	//let id = Uuid::new_v7();
	id: Uuid,
	display_name: String,
	primary_email: String,
	backup_emails: Vec<String>,
	password_hash: String,
	created_at: Instant,
	last_login: Instant,
	last_active: Instant,
	invited_by: Uuid,
	invite_code: String,
	//sqlx::types::Json<Book>
	ui_preferences: UserPreferences,
}