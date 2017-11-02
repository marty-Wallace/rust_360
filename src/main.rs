#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rust_360;
use rust_360::{ create_post, establish_connection };

fn main() {

    let s: &str = "Post 1";
    let b = "This is the body of post 1";

    let conn = establish_connection();
    create_post(&conn, s, b);

}

