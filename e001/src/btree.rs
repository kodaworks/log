use crate::orderbook::{OrderBook, Side};
use rust_decimal::Decimal;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct BTreeBook {
    asks: BTreeMap<Decimal, Decimal>,
    bids: BTreeMap<Decimal, Decimal>,
}

impl BTreeBook {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    fn get_map(&mut self, side: Side) -> &mut BTreeMap<Decimal, Decimal> {
        match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        }
    }
}

impl OrderBook for BTreeBook {
    #[inline]
    fn insert(&mut self, side: Side, price: Decimal, quantity: Decimal) {
        self.get_map(side).insert(price, quantity);
    }

    #[inline]
    fn delete(&mut self, side: Side, price: Decimal) {
        self.get_map(side).remove(&price);
    }

    #[inline]
    fn top(&self) -> (Option<(&Decimal, &Decimal)>, Option<(&Decimal, &Decimal)>) {
        // B-Tree is sorted in descending order
        // Get the last element for the highest bid
        let bid = self.bids.iter().next_back();
        // Get the first element for the lowest ask
        let ask = self.asks.iter().next();

        (bid, ask)
    }

    #[inline]
    fn bids(&self) -> impl Iterator<Item = (&Decimal, &Decimal)> {
        // Reverse the iterator to get the highest bid first
        self.bids.iter().rev()
    }

    #[inline]
    fn asks(&self) -> impl Iterator<Item = (&Decimal, &Decimal)> {
        // Get the lowest ask first
        self.asks.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orderbook::tests::*;

    #[test]
    fn test_btree_all() {
        test_all(BTreeBook::new);
    }
}
