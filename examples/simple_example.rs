fn main() {
    let mut a = TestStruct {
        v0: 20,
        v1: -3.0,
        v3: 1,
        ..Default::default()
    };

    let _ = dbg!(a.check_with_fallback());

    dbg!(&a);

    let _ = dbg!(a.check());
}

use range_checker::*;
// use range_checker_error::Error;

#[derive(Debug, Default, RangeCheckerVerbose)]
struct TestStruct {
    #[range(..=5)]
    #[range(20..)]
    #[filter(|x| x % 2 != 0)]
    #[fallback(255)]
    pub v0: u8,
    #[range(-1.0..=5.0)]
    #[fallback(3.1)]
    v1: f32,
    #[filter(|x| x > 8.0)]
    #[fallback(9.9)]
    v2: f64,
    #[range(..-1)]
    #[fallback(|x| x - 5)]
    v3: isize,
}
