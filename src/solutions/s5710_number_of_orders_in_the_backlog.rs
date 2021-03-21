use super::Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    #[allow(dead_code)]
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut buy_backlog = BinaryHeap::new();
        let mut buy_counter = HashMap::new();
        let mut sell_backlog = BinaryHeap::new();
        let mut sell_counter = HashMap::new();

        for order in orders {
            let (price, mut amount, order_type) = (order[0], order[1], order[2] != 0);
            match order_type {
                false => loop {
                    let Reverse(lowest_price) = *sell_backlog.peek().unwrap_or(&Reverse(i32::MAX));
                    let rest = *sell_counter.get(&lowest_price).unwrap_or(&i32::MIN);
                    if lowest_price > price {
                        if buy_counter.contains_key(&price) {
                            buy_counter.entry(price).and_modify(|v| *v += amount);
                        } else {
                            buy_backlog.push(price);
                            buy_counter.insert(price, amount);
                        }
                        break;
                    } else if rest > amount {
                        sell_counter
                            .entry(lowest_price)
                            .and_modify(|v| *v -= amount);
                        break;
                    } else {
                        sell_backlog.pop();
                        sell_counter.remove(&lowest_price);
                        amount -= rest;
                    }
                },
                true => loop {
                    let highest_price = *buy_backlog.peek().unwrap_or(&i32::MIN);
                    let rest = *buy_counter.get(&highest_price).unwrap_or(&i32::MIN);
                    if highest_price < price {
                        if sell_counter.contains_key(&price) {
                            sell_counter.entry(price).and_modify(|v| *v += amount);
                        } else {
                            sell_backlog.push(Reverse(price));
                            sell_counter.insert(price, amount);
                        }
                        break;
                    } else if rest > amount {
                        buy_counter
                            .entry(highest_price)
                            .and_modify(|v| *v -= amount);
                        break;
                    } else {
                        buy_backlog.pop();
                        buy_counter.remove(&highest_price);
                        amount -= rest;
                    }
                },
            }
        }

        for (_, amount) in [buy_counter, sell_counter].iter().flatten() {
            result = (result + *amount) % 1000000007;
        }

        return result;
    }
}
