mod scheduler;
mod sql_gen;
mod web_hook;

use poem::Route;

pub fn get_app() -> Route {
    Route::new()
        .nest_no_strip(scheduler::PATH_NAME, scheduler::scheduler_api())
        .nest_no_strip(web_hook::PATH_NAME, web_hook::web_hook_api())
        .nest(sql_gen::PATH_NAME, sql_gen::sqlgen_api())
}
