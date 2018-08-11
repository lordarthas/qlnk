extern crate actix;
extern crate actix_web;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rand;

#[macro_use] extern crate failure;

//use actix::prelude::*;
use actix_web::{server, App, http::Method, HttpRequest, HttpResponse};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use failure::Error;
use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};

#[derive(Clone,Debug)]
struct DbConfig {
    host    : String,
    name    : String,
    user    : String,
    pwd     : String
}

#[derive(Clone,Debug)]
struct Config {
    db              : DbConfig,
    addr            : String,
    auth            : String,
    shortcode_len   : usize,
}

struct Stash {
    config  : Config,
    dbp     : r2d2::Pool<PostgresConnectionManager>,
}

pub fn run() {
    let config = readconfig();
    let dbpool = connect_db(&config);
    let listen_addr = config.addr.clone(); // Clone it before it's moved into the closure

    server::HttpServer::new(move || App::with_state(Stash{ config: config.clone(), dbp: dbpool.clone() })
        .resource("/", |r| r.f(home))
        .resource("/find_shortcode/{shortcode}", |r| r.method(Method::GET).f(find_shortcode))
        //.resource("/create", |r| r.method(Method::POST).f(create))
        .resource("/create", |r| r.f(create))
    ).bind(listen_addr).unwrap().run();
}

fn home(req: &HttpRequest<Stash>) -> String {
    format!("We're listening on {}!", req.state().config.addr) 
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

    let targeturl = String::from("https://www.cattlegrid.info/"); // Static for tests
    let shortcode: String = rand::thread_rng().sample_iter(&Alphanumeric)
        .take( req.state().config.shortcode_len ).collect();

    dbc.execute("insert into links (shortcode, targeturl) values ($1, $2)",
        &[&shortcode, &targeturl]
    ).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Created shortcode : {}", shortcode)))
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
        shortcode_len   : 8,
        addr            : String::from("localhost:3000"),
        auth            : String::from("basicauth"),
    }
}

fn connect_db(config: &Config) -> r2d2::Pool<PostgresConnectionManager> {
    let cstring = format!("postgres://{}:{}@{}/{}", config.db.user, config.db.pwd, config.db.host, config.db.name);
    let manager = PostgresConnectionManager::new(cstring, TlsMode::None).unwrap();
    r2d2::Pool::new(manager).unwrap()
}