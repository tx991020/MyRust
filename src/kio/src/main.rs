#[macro_use]
extern crate nickel;
extern crate tera;

use std::env;
use std::path::Path;
use tera::{Tera, Context};
use nickel::{Nickel, HttpRouter, Middleware, MiddlewareResult, Request, Response, JsonBody, FormBody};


fn main() {
    let mut server = Nickel::new();

    let root_path = env::current_dir().unwrap();
    let template_dir = root_path.join(Path::new("templates/*.tera"));
    println!("{:?}", template_dir.to_str().unwrap());
    let template_engine = Tera::new(template_dir.to_str().unwrap());

    let mut ctx = Context::new();
    let foo = "";
    let hello = "world";
    let vector = vec![1, 3, 6];
    ctx.insert("foo", &foo);
    ctx.insert("hello", &hello);
    ctx.insert("vector", &vector);


    println!("{}", template_engine.unwrap().render("index.tera", &ctx).unwrap());


}