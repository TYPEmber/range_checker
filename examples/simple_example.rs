fn main() {
    let mut a = TestStruct {
        v0: 20,
        v1: -3.0,
        v3: 1,
        ..Default::default()
    };

    // dbg!(range_checker::Check::check(&a));
    // dbg!(range_checker::CheckVerbose::check(&a));

    let _ = dbg!(a.check_with_fallback());

    dbg!(&a);

    let _ = dbg!(a.check());
}

use range_checker::Check;
use range_checker::CheckVerbose;

#[derive(Debug, Default, CheckVerbose)]
struct TestStruct {
    #[range(..=5)]
    #[range(20..)]
    #[filter(|x| x % 2 != 0)]
    #[fallback(255)]
    pub v0: u8,
    #[range(-1.0..=5.0)]
    #[fallback(3.1)]
    v1: f32,
    #[filter(|&x| x > 8.0)]
    #[fallback(9.9)]
    v2: f64,
    #[range(..-1)]
    #[fallback(|x| {println!("test fallback closure."); x - 5})]
    v3: isize,
    #[filter(|x:&str| !x.is_empty())]
    v4: String,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[derive(Debug, Default, Check)]
    struct TestCheckStruct {
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

    #[test]
    fn basic_check_Check() {
        let mut a = TestCheckStruct {
            v0: 20,
            v1: -3.0,
            v3: 1,
            ..Default::default()
        };
    
        let _ = dbg!(a.check_with_fallback());
    
        dbg!(&a);
    
        let _ = dbg!(a.check());
    }

    #[derive(Debug, Default, CheckVerbose)]
    struct TestCheckVerboseStruct {
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

    #[test]
    fn basic_check_CheckVerbose() {
        let mut a = TestCheckVerboseStruct {
            v0: 20,
            v1: -3.0,
            v3: 1,
            ..Default::default()
        };
    
        let _ = dbg!(a.check_with_fallback());
    
        dbg!(&a);
    
        let _ = dbg!(a.check());
    }
}
