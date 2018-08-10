//! # URL shortener in Rust
//! 
//! ## TODO
//! 
//! * Make the basic work as a drop-in replacement for the Perl equivalent. I'll need to use iron, postgres and json crates
//! * Read configuration from a (toml?) files
//! 

extern crate qlnk;

fn main() {
    qlnk::run();
    // let config = readconfig();

    // let dbc = connect_db(&config);

    // let mut router = Router::new(); 
    // router.get("/", home, "home");
    // router.get("/:shortcode", find_shortcode, "find_shortcode");
    // router.get("/create", create_shortcode, "create_shortcode");

    // Iron::new(router).http(config.addr).unwrap();
}