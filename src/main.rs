use rocket_dyn_templates::{context, Template};

use utils::{App, Plugin};

use plugin_user_basic::Users;

#[rocket::launch]
async fn rocket() -> _ {
    let app = App::new();
    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(Users::new())];

    // plugins.iter().for_each(|plugin| plugin.init(&app).unwrap());

    rocket::build()
        .mount("/", rocket::routes![index])
        .attach(Template::fairing())
}

#[rocket::get("/")]
async fn index() -> Template {
    Template::render("index", context! { field: "Hello World" })
}
