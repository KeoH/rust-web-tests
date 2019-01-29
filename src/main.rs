extern crate iron;
extern crate router;
extern crate time;

use iron::{typemap, Iron, Request, Response, IronResult, AfterMiddleware, Chain, BeforeMiddleware};
use iron::error::{IronError};
use router::{Router, NoRoute};
use iron::status;
use time::precise_time_ns;

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

struct Handler404;

impl AfterMiddleware for Handler404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Entrando en una ruta desconocida, 404");
        if err.error.is::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Esta pagina no existe, 404")))
        } else {
            Err(err)
        }
    }
}

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}