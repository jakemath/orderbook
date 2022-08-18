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
    book.add(0, 300., 50.);
    book.add(0, 350., 25.);
    let bids = book.get_bids().clone();
    for bid in bids.into_sorted_iter() {
        println!("BID {:?}", bid);
    }
    let asks = book.get_asks().clone();
    for ask in asks.into_sorted_iter() {
        println!("ASK {:?}", ask);
    }
    let sell_simulation = book.simulate_market_order(0, 30.0);
    let buy_simulation = book.simulate_market_order(1, 75.0);
    println!("BUY_SIM: {}", buy_simulation.unwrap().0);
    println!("SELL_SIM: {}", sell_simulation.unwrap().0);
    println!("BEST_BID: {}", book.get_best_bid().unwrap().price);
    println!("WEIGHTED_BID: {}", book.get_weighted_bid().unwrap());
    println!("WEIGHTED_ASK: {}", book.get_weighted_ask().unwrap());
}
