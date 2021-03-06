use rocket;
use http::resources::users::handlers::*;
use http::resources::articles::handlers::*;
use http::errors::handlers::*;
use settings::Settings;
use rocket::Rocket;
use db::init_db;

pub struct ApiResult<R, E>(pub Result<R, E>);

pub fn main(settings: Settings) {
    rocket(settings).launch();
}

pub fn rocket(settings: Settings) -> Rocket {
    rocket::ignite()
        .manage(init_db(&settings.database))
        .manage(settings)
        .mount("/api/users", routes![current_user_handler, register_user_handler, login_user_handler, update_user_handler])
        .mount("/api/articles", routes![create_article_handler])
        .catch(catchers![
            not_found,
            unauthenticated,
            unauthorized,
            bad_request,
            unprocessable_entity,
            internal_server_error
        ])
}
