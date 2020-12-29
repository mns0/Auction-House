#![allow(dead_code)]
#![allow(non_snake_case)]

pub use auction_type::AuctionType;

use clap::{App, Arg};
use std::env;
use std::process::exit;

mod auction_type;

fn main() {
    //Parsing the options
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("Auction-Type")
                .short("at")
                .takes_value(true)
                .help("Select Auction Type: First or Second Price or All Pay"),
        )
        .arg(
            Arg::with_name("Num-Bidders")
                .short("nb")
                .takes_value(true)
                .help("Number of Bidders 1..n"),
        )
        .arg(
            Arg::with_name("Item")
                .short("i")
                .takes_value(true)
                .help("Item name"),
        )
        .arg(
            Arg::with_name("Bid-Floor")
                .short("bf")
                .takes_value(true)
                .help("Bid Floor"),
        )
        .get_matches();

    let at: AuctionType;
    let num_bidders: u16;
    let bid_floor: f32;

    if let Some(auction_type) = matches.value_of("Auction-Type") {
        at = auction_type.parse().unwrap();
    } else {
        println!("Didn't match the Auction type\n");
        println!("Auction Types Avaliable:\n");
        println!("FirstPrice\n");
        println!("SecondPrice\n");
        println!("AllPay\n");
        exit(1);
    }
    if let Some(nb) = matches.value_of("Num-Bidders") {
        num_bidders = nb.parse::<u16>().unwrap();
    }
    if let Some(bf) = matches.value_of("Bid-Floor") {
        bid_floor = bf.parse::<f32>().unwrap();
    }
    if let Some(bf) = matches.value_of("Bid-Floor") {
        bid_floor = bf.parse::<f32>().unwrap();
    }

    println!("{:#?}", (at, num_bidders, bid_floor));

    // match matches.subcommand() {
    //     ("at", Some(_matches)) => {
    //         eprintln!("unimplemented");
    //         exit(1);
    //     }
    //     _ => unreachable!(),
    // }
}
