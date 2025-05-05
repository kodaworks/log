use crate::orderbook::{OrderBook, Side};
use hashbrown::HashMap;
use rust_decimal::Decimal;

#[derive(Clone)]
pub struct HashMapBook {
    asks: HashMap<Decimal, Decimal>,
    bids: HashMap<Decimal, Decimal>,
}

impl HashMapBook {
    pub fn new() -> Self {
        Self {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn get_map(&mut self, side: Side) -> &mut HashMap<Decimal, Decimal> {
        match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        }
    }
}

impl OrderBook for HashMapBook {
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
        let bid = self.bids.iter().max_by_key(|(price, _)| *price);
        let ask = self.asks.iter().min_by_key(|(price, _)| *price);

        (bid, ask)
    }

    #[inline]
    fn bids(&self) -> impl Iterator<Item = (&Decimal, &Decimal)> {
        let mut bids: Vec<_> = self.bids.iter().collect();
        bids.sort_unstable_by(|a, b| b.0.cmp(a.0)); // descending
        bids.into_iter()
    }

    #[inline]
    fn asks(&self) -> impl Iterator<Item = (&Decimal, &Decimal)> {
        let mut asks: Vec<_> = self.asks.iter().collect();
        asks.sort_unstable_by(|a, b| a.0.cmp(b.0)); // ascending
        asks.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orderbook::tests::*;

    #[test]
    fn test_hashmap_all() {
        test_all(HashMapBook::new);
    }
}
