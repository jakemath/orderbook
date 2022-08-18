/*
Author: Jake Mathai
Purpose: Orderbook driver program
*/

mod orderbook;

fn main() {
    let mut book = orderbook::Orderbook::new(5);
    book.add(1, 100., 10.);
    book.add(1, 50., 1000.);
    book.add(1, 200., 10.);
    let bids = book.get_bids().clone();
    for bid in bids.into_sorted_iter() {
        println!("{:?}", bid);
    }
    let sim = book.simulate_market_order(0, 30.0);
    if sim.is_none() {
        return
    }
    println!("SIM: {}", sim.unwrap().0);
}
