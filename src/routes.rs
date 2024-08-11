use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::{
        health::health_check_handler,
        notes::{
            create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
            note_list_handler,
        },
        users::create_user,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route("/api/user", post(create_user))
        .route("/api/notes", post(create_note_handler))
        .route("/api/notes", get(note_list_handler))
        .route(
            "/api/notes/:id",
            get(get_note_handler)
                .patch(edit_note_handler)
                .delete(delete_note_handler),
        )
        .with_state(app_state)
}
