use crate::orderbook::{OrderBook, Side};
use hashbrown::HashMap;
use rust_decimal::Decimal;
use std::collections::BTreeMap;
use std::ptr::NonNull;

#[derive(Clone)]
pub struct HybridBook {
    asks: BTreeMap<Decimal, NonNull<Decimal>>,
    bids: BTreeMap<Decimal, NonNull<Decimal>>,
    askmap: HashMap<Decimal, NonNull<Decimal>>,
    bidmap: HashMap<Decimal, NonNull<Decimal>>,
    topbid: Option<(Decimal, NonNull<Decimal>)>,
    topask: Option<(Decimal, NonNull<Decimal>)>,
}

impl HybridBook {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            askmap: HashMap::new(),
            bidmap: HashMap::new(),
            topbid: None,
            topask: None,
        }
    }
}

impl OrderBook for HybridBook {
    #[inline]
    fn insert(&mut self, side: Side, price: Decimal, quantity: Decimal) {
        let map = match side {
            Side::Bid => &mut self.bidmap,
            Side::Ask => &mut self.askmap,
        };

        if let Some(ptr) = map.get(&price) {
            unsafe {
                *ptr.as_ptr() = quantity;
            }
            return;
        }

        let (tree, top) = match side {
            Side::Bid => (&mut self.bids, &mut self.topbid),
            Side::Ask => (&mut self.asks, &mut self.topask),
        };

        let boxed = Box::new(quantity);
        let ptr = NonNull::new(Box::into_raw(boxed)).unwrap();

        map.insert(price, ptr);
        tree.insert(price, ptr);

        match top {
            Some((top_price, _))
                if (side == Side::Bid && price > *top_price)
                    || (side == Side::Ask && price < *top_price) =>
            {
                *top = Some((price, ptr));
            }
            None => {
                *top = Some((price, ptr));
            }
            _ => {}
        }
    }

    #[inline]
    fn delete(&mut self, side: Side, price: Decimal) {
        let map = match side {
            Side::Bid => &mut self.bidmap,
            Side::Ask => &mut self.askmap,
        };

        if let Some(ptr) = map.remove(&price) {
            let (tree, top) = match side {
                Side::Bid => (&mut self.bids, &mut self.topbid),
                Side::Ask => (&mut self.asks, &mut self.topask),
            };

            tree.remove(&price);
            unsafe { drop(Box::from_raw(ptr.as_ptr())) };

            if let Some((top_price, _)) = top {
                if *top_price == price {
                    let next = if side == Side::Bid {
                        tree.iter().next_back()
                    } else {
                        tree.iter().next()
                    };
                    *top = next.map(|(p, q)| (*p, *q));
                }
            }
        }
    }

    #[inline]
    fn top(&self) -> (Option<(&Decimal, &Decimal)>, Option<(&Decimal, &Decimal)>) {
        let bid = self
            .topbid
            .as_ref()
            .map(|(p, q)| (p, unsafe { &*q.as_ptr() }));

        let ask = self
            .topask
            .as_ref()
            .map(|(p, q)| (p, unsafe { &*q.as_ptr() }));

        (bid, ask)
    }

    #[inline]
    fn bids(&self) -> impl Iterator<Item = (&Decimal, &Decimal)> {
        self.bids
            .iter()
            .rev()
            .map(|(p, ptr)| (p, unsafe { &*ptr.as_ptr() }))
    }

    #[inline]
    fn asks(&self) -> impl Iterator<Item = (&Decimal, &Decimal)> {
        self.asks
            .iter()
            .map(|(p, ptr)| (p, unsafe { &*ptr.as_ptr() }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orderbook::tests::*;

    #[test]
    fn test_hybrid_all() {
        test_all(HybridBook::new);
    }
}
