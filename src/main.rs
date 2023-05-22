#[macro_use] extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![answer])
        .register("/", catchers![not_found])
}

fn get_answer() -> &'static str {
    "42"
}