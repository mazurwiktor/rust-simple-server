extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use iron::status;
use iron::{Iron, Request, Response, IronResult};
use iron::mime::Mime;

use mount::Mount;
use router::Router;
use staticfile::Static;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn json_test(req: &mut Request) -> IronResult<Response> {
    println!("Running json_test handler, URL path: {}", req.url.path().join("/"));
    let content_type = "application/json".parse::<Mime>().unwrap();
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();

    Ok(Response::with((content_type, status::Ok, serialized)))
}

fn main() {
    let addres: &'static str = "127.0.0.1:3000";
    let mut router = Router::new();
    router
        .get("/json", json_test, "json");

    let mut mount = Mount::new();
    mount
        .mount("/", Static::new(Path::new("static")))
        .mount("/api", router);

    println!("Server started at http://{}/", &addres);
    Iron::new(mount).http(addres).unwrap();
}