use poem::{endpoint::StaticFilesEndpoint, Route};

use crate::config::CONFIG;

pub fn get_app() -> Route {
    Route::new().nest(
        "/",
        StaticFilesEndpoint::new(&CONFIG.server.fe)
            .index_file("index.html")
            .fallback_to_index(),
    )
}
