fn main() {
    let a = TestStruct {
        v0: 20,
        v1: -3.0,
        ..Default::default()
    };
    
    dbg!(a.check());
}

use range_checker::*;

#[derive(Default, RangeChecker)]
struct TestStruct {
    #[range(..=5)]
    pub v0: u8,
    #[range(-1.0..=5.0)]
    v1: f32,
    v2: isize,
}
