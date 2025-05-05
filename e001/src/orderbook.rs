use rust_decimal::Decimal;
use std::hash::Hash;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}

pub trait OrderBook {
    fn insert(&mut self, side: Side, price: Decimal, quantity: Decimal);
    fn delete(&mut self, side: Side, price: Decimal);

    fn top(&self) -> (Option<(&Decimal, &Decimal)>, Option<(&Decimal, &Decimal)>);
    fn bids(&self) -> impl Iterator<Item = (&Decimal, &Decimal)>;
    fn asks(&self) -> impl Iterator<Item = (&Decimal, &Decimal)>;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal::Decimal;

    // The constructor is a function/closure that returns a new instance of the book
    fn test_insert<T: OrderBook>(mut new_book: impl FnMut() -> T) {
        let mut book = new_book();

        book.insert(Side::Bid, Decimal::from(100), Decimal::from(10));
        assert_eq!(
            book.top(),
            (Some((&Decimal::from(100), &Decimal::from(10))), None)
        );
    }

    fn test_modify<T: OrderBook>(mut new_book: impl FnMut() -> T) {
        let mut book = new_book();
        book.insert(Side::Bid, Decimal::from(100), Decimal::from(10));
        book.insert(Side::Bid, Decimal::from(100), Decimal::from(20));

        assert_eq!(
            book.top(),
            (Some((&Decimal::from(100), &Decimal::from(20))), None)
        );
    }

    fn test_delete<T: OrderBook>(mut new_book: impl FnMut() -> T) {
        let mut book = new_book();
        book.insert(Side::Bid, Decimal::from(100), Decimal::from(10));
        book.delete(Side::Bid, Decimal::from(100));

        assert_eq!(book.top(), (None, None));
    }

    fn test_top<T: OrderBook>(mut new_book: impl FnMut() -> T) {
        let mut book = new_book();

        book.insert(Side::Ask, Decimal::from(140), Decimal::from(20));
        book.insert(Side::Ask, Decimal::from(130), Decimal::from(10));
        book.insert(Side::Ask, Decimal::from(120), Decimal::from(30));

        book.insert(Side::Bid, Decimal::from(110), Decimal::from(10));
        book.insert(Side::Bid, Decimal::from(100), Decimal::from(20));
        book.insert(Side::Bid, Decimal::from(90), Decimal::from(30));

        let top = book.top();
        assert_eq!(
            top,
            (
                Some((&Decimal::from(110), &Decimal::from(10))),
                Some((&Decimal::from(120), &Decimal::from(30)))
            )
        );

        book.delete(Side::Bid, Decimal::from(110));
        book.delete(Side::Bid, Decimal::from(100));
        book.delete(Side::Bid, Decimal::from(90));

        book.delete(Side::Ask, Decimal::from(120));
        book.delete(Side::Ask, Decimal::from(130));
        book.delete(Side::Ask, Decimal::from(140));

        assert_eq!(book.top(), (None, None));
    }

    fn test_bids<T: OrderBook>(mut new_book: impl FnMut() -> T) {
        let mut book = new_book();
        book.insert(Side::Bid, Decimal::from(100), Decimal::from(10));
        book.insert(Side::Bid, Decimal::from(90), Decimal::from(20));
        book.insert(Side::Bid, Decimal::from(80), Decimal::from(30));

        let bids = book.bids().collect::<Vec<_>>();
        assert_eq!(
            bids,
            vec![
                (&Decimal::from(100), &Decimal::from(10)),
                (&Decimal::from(90), &Decimal::from(20)),
                (&Decimal::from(80), &Decimal::from(30)),
            ]
        );
    }

    fn test_asks<T: OrderBook>(mut new_book: impl FnMut() -> T) {
        let mut book = new_book();
        book.insert(Side::Ask, Decimal::from(100), Decimal::from(10));
        book.insert(Side::Ask, Decimal::from(90), Decimal::from(20));
        book.insert(Side::Ask, Decimal::from(80), Decimal::from(30));

        let asks = book.asks().collect::<Vec<_>>();
        assert_eq!(
            asks,
            vec![
                (&Decimal::from(80), &Decimal::from(30)),
                (&Decimal::from(90), &Decimal::from(20)),
                (&Decimal::from(100), &Decimal::from(10)),
            ]
        );
    }

    pub fn test_all<T: OrderBook>(mut new_book: impl FnMut() -> T) {
        test_insert(&mut new_book);
        test_modify(&mut new_book);
        test_delete(&mut new_book);
        test_top(&mut new_book);
        test_bids(&mut new_book);
        test_asks(&mut new_book);
    }
}
