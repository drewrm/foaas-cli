extern crate hyper;
extern crate getopts;
extern crate foaas_cli;

use std::io;
use std::env;
use hyper::Client;
use hyper::header::*;
use hyper::mime::{Mime};
use hyper::mime::TopLevel::Text;
use hyper::mime::SubLevel::Plain;
use getopts::Options;
use foaas_cli::foaas;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "command", "Type of fuck off", "COMMAND");
    opts.optopt("f", "from", "The giver of the fuck off", "FROM");
    opts.optopt("n", "name", "The recipient of the fuck off", "TO");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let command = matches.opt_str("c").unwrap();
    let from = matches.opt_str("f");
    let name = matches.opt_str("n");

    let url =  foaas::get_path(command, from, name);
    let accept = QualityItem::new(Mime(Text, Plain, vec![]), q(1.000));

    let mut client = Client::new();
    let mut res = client.get(&*url)
        .header(ContentType::json())
        .header(Accept(vec![accept]))
        .send().unwrap();

    io::copy(&mut res, &mut io::stdout()).unwrap();
    println!("");
}
