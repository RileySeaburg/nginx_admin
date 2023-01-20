use rocket_dyn_templates::{context, Template};

#[get("/domains")]
pub fn dashboard() -> Template {
    Template::render(
        "pages/domains",
        context! {
            page_title: "Domains",
            error_title: "Error",
        },
    )
}
