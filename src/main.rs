#[macro_use] extern crate rocket;
// use rocket::http::{Status, Header, ContentType};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api/v1/answer")]
fn answer() -> &'static str {
    return get_answer();
}

// 404 catcher
#[catch(404)]
fn not_found() -> &'static str {
    "Not found!"
}

// responder example
#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct AllOkResponse {
    text: String
}

// needed for Render deploys
// this one returns the correct status code
#[get("/all_ok")]
fn all_ok() -> AllOkResponse {
    return AllOkResponse {
        text: "OK".to_string()
    };
}

// Plain status code example
// #[get("/all_ok")]
// fn all_ok() -> Status {
//     return Status::Ok;
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![answer])
        .register("/", catchers![not_found])
        .mount("/", routes![all_ok])
}

fn get_answer() -> &'static str {
    return "42";
}