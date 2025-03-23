mod scheduler;

use poem::Route;

pub fn get_app() -> Route {
    Route::new().nest_no_strip(scheduler::PATH_NAME, scheduler::scheduler_api())
}
