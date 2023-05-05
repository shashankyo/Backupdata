#[macro_use] extern crate rocket;

use rocket::{tokio::sync::broadcast::{{channel, Sender}, serde::{Serialize, Deserialize}}, serde::{Serialize, Deserialize}, State, Rocket};
#[get("/world")]
fn world() -> &'static str {
    "Hello, world"
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,

}
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>){
    let _res = queue.Send(form.into_inner());
}


#[launch]

fn rocket() -> _ {
    rocket::build(); Rocket<Build>
        .manage(state: channel::<Message>(capacity: 1024).0)
        .mount(base: "/hello", routes: routes![world])
}