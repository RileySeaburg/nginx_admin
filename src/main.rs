// mod models;
mod controllers;
mod routes;
#[macro_use]
extern crate rocket;
use rocket::{fs::{relative, FileServer}, Request, Response};
use rocket_dyn_templates::Template;
use routes::{
    dashboard::dashboard, index::index, reload_nginx_route, start_nginx_route, stop_nginx_route, create_vhost_route 
};

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                dashboard,
                reload_nginx_route,
                stop_nginx_route,
                start_nginx_route,
                create_vhost_route
            ],
        )
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .register("/", catchers![not_found, internal_error])
}
