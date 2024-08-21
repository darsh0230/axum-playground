use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{
    auth_handler, // notes::{
                  //     create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
                  //     note_list_handler,
                  // },
                  // users_handler::create_user,
    health_handler::health_check_handler,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route("/api/users", post(auth_handler::user_register))
    // .route("/api/user", post(create_user))
    // .route("/api/notes", post(create_note_handler))
    // .route("/api/notes", get(note_list_handler))
    // .route(
    //     "/api/notes/:id",
    //     get(get_note_handler)
    //         .patch(edit_note_handler)
    //         .delete(delete_note_handler),
    // )
}
