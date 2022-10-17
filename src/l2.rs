/*
Author: Jake Mathai
Purpose: L2 orderbook
*/

use std::collections::BTreeMap;

/*
Bids and asks trees map scaled price to scaled quantity.
Methods iterate bids in descending order and asks in ascending order of price keys
*/
pub struct Orderbook {
    pub bids: BTreeMap<u64, u64>,
    pub asks: BTreeMap<u64, u64>,
    pub price_factor: f64,
    pub quantity_factor: f64
}

const MAX_DECIMALS: u8 = 8;
const DEFAULT_DECIMALS: u8 = 6;

impl Orderbook {
    pub fn new(price_decimals: Option<u8>, quantity_decimals: Option<u8>) -> Orderbook {
        Orderbook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            price_factor: f64::powf(
                10.0, 
                (
                    match price_decimals {
                        Some(x) => {
                            if x > MAX_DECIMALS {
                                panic!("Too many decimals");
                            }
                            x
                        },
                        None => DEFAULT_DECIMALS
                    }
                ).into()
            ),
            quantity_factor: f64::powf(
                10.0, 
                (
                    match quantity_decimals {
                        Some(x) => {
                            if x > MAX_DECIMALS {
                                panic!("Too many decimals");
                            }
                            x
                        },
                        None => DEFAULT_DECIMALS
                    }
                ).into()
            ),
        }
    }

    /*
    Process orderbook update. If is_snapshot, resets the bids and asks to empty.
    Bids and asks should be formatted as (price, quantity)
    */
    pub fn process(&mut self, bids: Vec<(f64, f64)>, asks: Vec<(f64, f64)>, is_snapshot: bool) {
        if is_snapshot {
            self.bids.clear();
            self.asks.clear();
        }
        for bid in bids.iter() {
            if bid.1 > 0.0 {
                let scaled_price = (bid.0 * self.price_factor) as u64;
                let scaled_quantity = (bid.1 * self.quantity_factor) as u64;
                self.bids.insert(scaled_price, scaled_quantity);
            }
        }
        for ask in asks.iter() {
            if ask.1 > 0.0 {
                let scaled_price = (ask.0 * self.price_factor) as u64;
                let scaled_quantity = (ask.1 * self.quantity_factor) as u64;
                self.asks.insert(scaled_price, scaled_quantity);
            }
        }
    }

    pub fn get_best_bid(&self) -> Option<(u64, u64)> {
        match self.bids.iter().rev().next() {
            Some((price, quantity)) => Some((*price, *quantity)),
            None => None
        }
    }

    pub fn get_best_ask(&self) -> Option<(u64, u64)> {
        match self.asks.iter().next() {
            Some((price, quantity)) => Some((*price, *quantity)),
            None => None
        }
    }

    pub fn get_weighted_mid_price(&self) -> Option<f64> {
        let best_bid_option = self.get_best_bid();
        if best_bid_option.is_none() {
            return None;
        }
        let best_bid = best_bid_option.unwrap();
        let best_ask_option = self.get_best_ask();
        if best_ask_option.is_none() {
            return None;
        }
        let best_ask = best_ask_option.unwrap();
        Some(((best_bid.0 * best_bid.1 + best_ask.0 * best_ask.1) as f64) / ((best_bid.1 + best_ask.1) as f64))
    }

    pub fn get_weighted_bid(&self) -> Option<f64> {
        if self.bids.is_empty() {
            return None;
        }
        let mut numerator: u64 = 0;
        let mut total_quantity: u64 = 0;
        for (price, quantity) in self.bids.iter() {
            numerator += price * quantity;
            total_quantity += quantity;
        }
        Some((numerator as f64) / (total_quantity as f64))
    }

    pub fn get_weighted_ask(&self) -> Option<f64> {
        if self.asks.is_empty() {
            return None;
        }
        let mut numerator: u64 = 0;
        let mut total_quantity: u64 = 0;
        for (price, quantity) in self.asks.iter() {
            numerator += price * quantity;
            total_quantity += quantity;
        }
        Some((numerator as f64) / (total_quantity as f64))
    }

    pub fn get_total_bid_quantity(&self) -> f64 {
        let mut total_quantity: u64 = 0;
        for (_, quantity) in self.bids.iter() {
            total_quantity += quantity;
        }
        (total_quantity as f64) / self.quantity_factor
    }

    pub fn get_total_ask_quantity(&self) -> f64 {
        let mut total_quantity: u64 = 0;
        for (_, quantity) in self.asks.iter() {
            total_quantity += quantity;
        }
        (total_quantity as f64) / self.quantity_factor
    }

    pub fn simulate_taker_buy(&self, quantity: f64) -> Option<f64> {
        let scaled_quantity = (quantity * self.quantity_factor) as u64;
        let mut amount_remaining = scaled_quantity;
        let mut price_numerator: u64 = 0;
        for (ask_price, ask_quantity) in self.asks.iter() {
            if ask_quantity > &amount_remaining {
                price_numerator += amount_remaining * ask_price;
                amount_remaining = 0;
                break;
            }
            price_numerator += ask_quantity * ask_price;
            amount_remaining -= ask_quantity;
        }
        match amount_remaining {
            0 => None,
            _ => Some((price_numerator as f64) / (self.quantity_factor * quantity))
        }
    }

    pub fn simulate_taker_sell(&self, quantity: f64) -> Option<f64> {
        let scaled_quantity = (quantity * self.quantity_factor) as u64;
        let mut amount_remaining = scaled_quantity;
        let mut price_numerator: u64 = 0;
        for (ask_price, ask_quantity) in self.bids.iter().rev() {
            if ask_quantity > &amount_remaining {
                price_numerator += amount_remaining * ask_price;
                amount_remaining = 0;
                break;
            }
            price_numerator += ask_quantity * ask_price;
            amount_remaining -= ask_quantity;
        }
        match amount_remaining {
            0 => None,
            _ => Some((price_numerator as f64) / (self.quantity_factor * quantity))
        }
    }
}