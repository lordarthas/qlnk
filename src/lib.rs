extern crate actix;
extern crate actix_web;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

//use actix::prelude::*;
use actix_web::{server, App, http::Method, HttpRequest, HttpResponse};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use failure::Error;

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

struct Stash {
    dbp: r2d2::Pool<PostgresConnectionManager>,
}

pub struct Qlnk {
    config  : Config
}

lazy_static! {
    static ref CONFIG   : Config = readconfig();
    // "std::cell::RefCell<postgres::InnerConnection>` cannot be shared between threads safely"
    //static ref DBC      : postgres::Connection = connect_db(); 
}

pub fn run() {
    let dbpool = connect_db();

    server::HttpServer::new(move || App::with_state(Stash{ dbp: dbpool.clone() })
        .resource("/", |r| r.f(home))
        .resource("/find_shortcode/{shortcode}", |r| r.method(Method::GET).f(find_shortcode))
        //.resource("/create", |r| r.method(Method::POST).f(create))
        .resource("/create", |r| r.f(create))
    ).bind("127.0.0.1:3000").unwrap().run();
}

fn home(_req: &HttpRequest<Stash>) -> &'static str {
    "We do not have a home page yet!"
}

fn find_shortcode(req: &HttpRequest<Stash>) -> Result<HttpResponse, Error> {
// get '/:shortcode' => [shortcode => qr/[A-Za-z0-9]{8}/] => sub($self) {
//     my $link = $self->pg->db->query('select targeturl from links where shortcode = ?', $self->stash->{shortcode})->hash;
//     if (defined $link) {
//         $self->res->code(303);
//         return $self->redirect_to( $link->{targeturl} );
//     }
//     return $self->render(text => 'Notfound-shortcode');

    let shortcode : String = req.match_info().query("shortcode")?;
    let body = format!("Redirecting you to URL for shortcode: {} ", shortcode);

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(body))
}

fn create(req: &HttpRequest<Stash>) -> Result<HttpResponse, Error> {
    let dbc = req.state().dbp.get().unwrap();
    dbc.execute("insert into links (shortcode, targeturl) values ($1, $2)", &[&"a".to_string(), &"b".to_string()]).unwrap();
//     let targeturl = String::from("https://www.cattlegrid.info/"); // Static for tests

// // let shortcode: String = rand::thread_rng().gen_ascii_chars().take(len).collect();

//     Ok(Response::with((status::Ok, "Create a shortcode for URL... TODO")))

//     // conn.execute("CREATE TABLE person (
//     //                 id              SERIAL PRIMARY KEY,
//     //                 name            VARCHAR NOT NULL,
//     //                 data            BYTEA
//     //               )", &[]).unwrap();
//     // let me = Person {
//     //     id: 0,
//     //     name: "Steven".to_string(),
//     //     data: None,
//     // };
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Creating shortcode"))
}

/// Reads configuration - it's actually coded inline, but should be read
/// from an external file
fn readconfig() -> Config {
    Config {
        db : DbConfig {
            host    : String::from("localhost"),
            name    : String::from("qlnk"),
            user    : String::from("qlnk"),
            pwd     : String::from("pqlnk")
        },
        addr: String::from("localhost:3000"),
        auth: String::from("basicauth"),
    }
}

fn connect_db() -> r2d2::Pool<PostgresConnectionManager> {
    let cstring = format!("postgres://{}:{}@{}/{}", CONFIG.db.user, CONFIG.db.pwd, CONFIG.db.host, CONFIG.db.name);
    let manager = PostgresConnectionManager::new(cstring, TlsMode::None).unwrap();
    r2d2::Pool::new(manager).unwrap()
}