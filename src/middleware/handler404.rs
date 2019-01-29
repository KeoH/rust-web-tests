use iron::error::IronError;
use iron::{AfterMiddleware, IronResult, Request, Response};
use router::NoRoute;
use iron::status;

pub struct Handler404;

impl AfterMiddleware for Handler404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Entrando en una ruta desconocida, 404");
        if err.error.is::<NoRoute>() {
            Ok(Response::with((
                status::NotFound,
                "Esta pagina no existe, 404",
            )))
        } else {
            Err(err)
        }
    }
}
