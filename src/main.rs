mod app;
mod util;
mod my_trait;

use app::main::desc;
use rocket::{catch, fs::FileServer, get, response::Redirect, catchers, routes};

use crate::app::main::today;

// #[get("/")]
fn root() -> FileServer {
    FileServer::from("./staticHtml/")
}

#[get("/<id>")]
fn redirect_desc(id: i32) -> Redirect {
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
        .mount("/", routes![redirect_desc, today, desc])
        .register("/", catchers![not_found])
        .launch()
        .await?;
    Ok(())
}
