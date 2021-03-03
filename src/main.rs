#[macro_use] extern crate rocket;

// This is a route
#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::ignite()
        .mount("/", routes![hello])
        .launch()
        .await();
}
