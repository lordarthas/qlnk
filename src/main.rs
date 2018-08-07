extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

struct DbConfig {
    host    : String,
    name    : String,
    user    : String,
    pwd     : String
}

struct Config {
    db      : DbConfig,
    addr    : String,
    auth    : String
}

fn home(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "We do not have a home page yet!")))
}

fn find_shortcode(req: &mut Request) -> IronResult<Response> {
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

fn create_shortcode(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Create a shortcode for URL... TODO")))
}

/// Reads configuration - it's actually coded inline, but should be read
/// from an external file
fn readconfig() -> Config {
    Config {
        db : DbConfig {
            host    : String::from("localhost"),
            name    : String::from("qlnk"),
            user    : String::from("uqlnk"),
            pwd     : String::from("pqlnk")
        },
        addr: String::from("localhost:3000"),
        auth: String::from("basicauth"),
    }
}

fn main() {
    let config = readconfig();

    let mut router = Router::new(); 
    router.get("/", home, "home");
    router.get("/:shortcode", find_shortcode, "find_shortcode");
    router.get("/create", create_shortcode, "create_shortcode");

    Iron::new(router).http(config.addr).unwrap();
}