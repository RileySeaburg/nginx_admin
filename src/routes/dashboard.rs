use rocket_dyn_templates::{context, Template};

#[get("/dashboard")]
pub fn dashboard() -> Template {
    Template::render(
        "pages/dashboard",
        context! {
            page_title: "Dashboard",
            error_title: "Error",
        },
    )
}
