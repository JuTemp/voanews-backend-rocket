mod app;
mod util;

use rocket::{catch, fs::FileServer, get, response::Redirect, catchers, routes};

// #[get("/")]
fn root() -> FileServer {
    FileServer::from("./staticHtml/")
}

#[get("/<id>")]
fn redirect_desc(id: String) -> Redirect {
    Redirect::to(format!("/#/desc?id={}", id))
}

#[catch(default)]
async fn not_found() -> &'static str {
    "catcher"
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 26890)))
        // .mount("/static/", FileServer::from(crate::MP4_PATH))
        .mount("/", root())
        .mount("/", routes![redirect_desc])
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
