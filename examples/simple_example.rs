fn main() {
    if ().eq(&()) {
        dbg!("eq");
    }
    let mut a = TestStruct {
        v0: 20,
        v1: -3.0,
        v3: 100,
        ..Default::default()
    };
    
    dbg!(a.check_with_fallback());

    dbg!(&a);

    dbg!(a.check());

}

use range_checker::*;
use std::ops::{RangeBounds, Bound, Range};

#[derive(Debug, Default, RangeChecker)]
struct TestStruct {
    #[range(..=5)]
    #[range(20..)]
    #[fallback(255u8)]
    pub v0: u8,
    #[range(-1.0..=5.0)]
    #[fallback(3.1)]
    v1: f32,
    v2: f64,
    #[range(..-5)]
    v3: isize,
}
