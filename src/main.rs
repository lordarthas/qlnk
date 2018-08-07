extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn home(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "We do not have a home page yet!")))
}

fn shortcode(req: &mut Request) -> IronResult<Response> {
// get '/:shortcode' => [shortcode => qr/[A-Za-z0-9]{8}/] => sub($self) {
//     my $link = $self->pg->db->query('select targeturl from links where shortcode = ?', $self->stash->{shortcode})->hash;
//     if (defined $link) {
//         $self->res->code(303);
//         return $self->redirect_to( $link->{targeturl} );
//     }
//     return $self->render(text => 'Notfound-shortcode');
    let ref query = req.extensions.get::<Router>().unwrap().find("shortcode").unwrap_or("X");
    Ok(Response::with((status::Ok,
        format!("Redirecting you to URL for shortcode: {} ", *query)
    )))
}

fn main() {
    let mut router = Router::new(); 
    router.get("/", home, "home");
    router.get("/:shortcode", shortcode, "shortcode");

    Iron::new(router).http("localhost:3000").unwrap();
}