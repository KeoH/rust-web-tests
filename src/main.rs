extern crate iron;
extern crate router;
extern crate time;

use iron::{Iron, Request, Response, IronResult, Chain};
use router::{Router};
use iron::status;

mod middleware;

use middleware::response_time::ResponseTime;
use middleware::handler404::Handler404;

fn main() {

    let mut router = Router::new();

    router.get("/", index_view, "index");
    router.get("/profile/:id", profile_view, "profile");

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    chain.link_after(Handler404);

    Iron::new(chain).http("localhost:3000").unwrap();
    println!("Running on port 3000");
}

fn index_view(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn profile_view(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}
