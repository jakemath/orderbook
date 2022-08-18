/*
Author: Jake Mathai
Purpose: Orderbook implementation
*/

use std::hash::{Hash, Hasher};

use priority_queue::PriorityQueue;

#[derive(Debug, Clone)]
pub struct PriceLevel {
    key: u32,
    price: f32,
    quantity: f32
}

impl Hash for PriceLevel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl PartialEq for PriceLevel {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for PriceLevel {}

pub struct Orderbook {
    key_factor: f32,
    bids: PriorityQueue<PriceLevel, i64>,
    asks: PriorityQueue<PriceLevel, i64>
}

impl Orderbook {
    pub fn new(decimals: u8) -> Orderbook {
        let base: u32 = 10;
        Orderbook { 
            key_factor: base.pow(decimals.into()) as f32,
            bids: PriorityQueue::new(), 
            asks: PriorityQueue::new() 
        }
    }

    // Side == 1 -> bid; Side == 0 -> ask
    pub fn add(&mut self, side: u8, price: f32, quantity: f32) {
        let integer_price = (price * self.key_factor).round() as u32;
        let key = integer_price as i64;
        let price_level = PriceLevel { 
            key: integer_price, 
            price, 
            quantity 
        };
        if side == 1 {
            self.bids.push(price_level, key);
        }
        else {
            self.asks.push(price_level, -key);
        }
    }

    pub fn get_bids(&self) -> &PriorityQueue<PriceLevel, i64> { &self.bids }

    pub fn get_asks(&self) -> &PriorityQueue<PriceLevel, i64> { &self.asks }

    fn get_best_bid(&self) -> Option<&PriceLevel> {
        let bid_peek = self.bids.peek();
        if bid_peek.is_none() {
            return None;
        }
        Some(bid_peek.unwrap().0)
    }

    fn get_best_ask(&self) -> Option<&PriceLevel> {
        let ask_peek = self.asks.peek();
        if ask_peek.is_none() {
            return None;
        }
        Some(ask_peek.unwrap().0)
    }

    fn get_mid_price(&self) -> Option<f32> {
        let bid_peek = self.get_best_bid();
        if bid_peek.is_none() {
            return None;
        }
        let best_bid = bid_peek.unwrap();
        let ask_peek = self.get_best_ask();
        if ask_peek.is_none() {
            return None;
        }
        let best_ask = ask_peek.unwrap();
        Some((best_bid.price + best_ask.price) * 0.5)
    }

    // Side == 1 -> simulate buy by walking asks; Side == 0 -> simulate sell by walking bids
    pub fn simulate_market_order(&self, side: u8, simulation_amount: f32) -> Option<(f32, f32)> {
        let mut book_side_clone;
        if side == 1 {
            book_side_clone = self.asks.clone();
        } 
        else {
            book_side_clone = self.bids.clone();
        }
        if book_side_clone.is_empty() {
            return None;
        }
        let mut amount_remaining: f32 = simulation_amount;
        let mut execution_price: f32 = 0.0;
        let mut worst_price_level: Option<f32> = None;
        while !book_side_clone.is_empty() {
            let price_level: &PriceLevel = book_side_clone.peek().unwrap().0;
            let price = price_level.price;
            let quantity = price_level.quantity;
            if quantity >= amount_remaining {
                execution_price += price * amount_remaining;
                amount_remaining = 0.0;
                worst_price_level = Some(price);
                break;
            }
            execution_price += price * quantity;
            amount_remaining -= quantity;
            book_side_clone.pop();
        }
        if amount_remaining != 0.0 {
            return None;
        }
        Some((execution_price / simulation_amount, worst_price_level.unwrap()))
    }

    fn get_weighted_bid(&self) -> Option<f32> {
        if self.bids.is_empty() {
            return None;
        }
        let mut bid_quantity: f32 = 0.0;
        let mut bid_sum: f32 = 0.0;
        for price_level in self.bids.iter() {
            let quantity: f32 = price_level.0.quantity;
            bid_quantity += quantity;
            bid_sum += quantity*price_level.0.price;
        }
        Some(bid_sum / bid_quantity)
    }

    fn get_weighted_ask(&self) -> Option<f32> {
        if self.asks.is_empty() {
            return None;
        }
        let mut ask_quantity: f32 = 0.0;
        let mut ask_sum: f32 = 0.0;
        for price_level in self.asks.iter() {
            let quantity: f32 = price_level.0.quantity;
            ask_quantity += quantity;
            ask_sum += quantity*price_level.0.price;
        }
        Some(ask_sum / ask_quantity)
    }
}