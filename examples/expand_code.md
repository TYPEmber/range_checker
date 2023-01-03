
# Expand Example

## Check

```rust
impl range_checker::Check for TestStruct {
    fn check(&self) -> Result<(), ()> {
        if !((..=5).contains(&self.v0) || (..=5).contains(&self.v0)
            || (20..).contains(&self.v0) && (|x| x % 2 != 0)(self.v0))
        {
            return Err(());
        }
        if !((-1.0..=5.0).contains(&self.v1) || (-1.0..=5.0).contains(&self.v1)) {
            return Err(());
        }
        if !((|x| x > 8.0)(self.v2) && (|x| x > 8.0)(self.v2)) {
            return Err(());
        }
        if !((..-1).contains(&self.v3) || (..-1).contains(&self.v3)) {
            return Err(());
        }
        Ok(())
    }
    #[allow(unreachable_code)]
    fn check_with_fallback(&mut self) -> Result<(), ()> {
        if !((..=5).contains(&self.v0) || (..=5).contains(&self.v0)
            || (20..).contains(&self.v0) && (|x| x % 2 != 0)(self.v0))
        {
            self.v0 = 255;
        }
        if !((-1.0..=5.0).contains(&self.v1) || (-1.0..=5.0).contains(&self.v1)) {
            self.v1 = 3.1;
        }
        if !((|x| x > 8.0)(self.v2) && (|x| x > 8.0)(self.v2)) {
            self.v2 = 9.9;
        }
        if !((..-1).contains(&self.v3) || (..-1).contains(&self.v3)) {
            self
                .v3 = (|x| {
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["test fallback closure.\n"],
                            &[],
                        ),
                    );
                };
                x - 5
            })(self.v3);
        }
        Ok(())
    }
}
```

## CheckVerbose

```rust
impl range_checker::CheckVerbose for TestStruct {
    fn check(&self) -> Result<(), Vec<range_checker::Error>> {
        let mut err_vec = ::alloc::vec::Vec::new();
        if !((..=5).contains(&self.v0) || (..=5).contains(&self.v0)
            || (20..).contains(&self.v0) && (|x| x % 2 != 0)(self.v0))
        {
            err_vec
                .push(range_checker::Error::CheckFailed {
                    ident: "v0".to_owned(),
                    value: (self.v0).to_string(),
                    check_statement: "(..= 5).contains(& self.v0) || (..= 5).contains(& self.v0) ||\n(20 ..).contains(& self.v0) && (| x | x % 2 != 0) (self.v0)"
                        .to_owned(),
                })
        }
        if !((-1.0..=5.0).contains(&self.v1) || (-1.0..=5.0).contains(&self.v1)) {
            err_vec
                .push(range_checker::Error::CheckFailed {
                    ident: "v1".to_owned(),
                    value: (self.v1).to_string(),
                    check_statement: "(- 1.0 ..= 5.0).contains(& self.v1) || (- 1.0 ..= 5.0).contains(& self.v1)"
                        .to_owned(),
                })
        }
        if !((|x| x > 8.0)(self.v2) && (|x| x > 8.0)(self.v2)) {
            err_vec
                .push(range_checker::Error::CheckFailed {
                    ident: "v2".to_owned(),
                    value: (self.v2).to_string(),
                    check_statement: "(| x | x > 8.0) (self.v2) && (| x | x > 8.0) (self.v2)"
                        .to_owned(),
                })
        }
        if !((..-1).contains(&self.v3) || (..-1).contains(&self.v3)) {
            err_vec
                .push(range_checker::Error::CheckFailed {
                    ident: "v3".to_owned(),
                    value: (self.v3).to_string(),
                    check_statement: "(.. - 1).contains(& self.v3) || (.. - 1).contains(& self.v3)"
                        .to_owned(),
                })
        }
        if err_vec.is_empty() { Ok(()) } else { Err(err_vec) }
    }
    fn check_with_fallback(
        &mut self,
    ) -> Result<Vec<range_checker::Error>, Vec<range_checker::Error>> {
        let mut ret_vec = ::alloc::vec::Vec::new();
        let mut failed = false;
        if !((..=5).contains(&self.v0) || (..=5).contains(&self.v0)
            || (20..).contains(&self.v0) && (|x| x % 2 != 0)(self.v0))
        {
            let fallback = 255;
            ret_vec
                .push(range_checker::Error::Fallback {
                    ident: "v0".to_owned(),
                    value: (self.v0).to_string(),
                    check_statement: "(..= 5).contains(& self.v0) || (..= 5).contains(& self.v0) ||\n(20 ..).contains(& self.v0) && (| x | x % 2 != 0) (self.v0)"
                        .to_owned(),
                    fallback: fallback.to_string(),
                });
            self.v0 = fallback;
        }
        if !((-1.0..=5.0).contains(&self.v1) || (-1.0..=5.0).contains(&self.v1)) {
            let fallback = 3.1;
            ret_vec
                .push(range_checker::Error::Fallback {
                    ident: "v1".to_owned(),
                    value: (self.v1).to_string(),
                    check_statement: "(- 1.0 ..= 5.0).contains(& self.v1) || (- 1.0 ..= 5.0).contains(& self.v1)"
                        .to_owned(),
                    fallback: fallback.to_string(),
                });
            self.v1 = fallback;
        }
        if !((|x| x > 8.0)(self.v2) && (|x| x > 8.0)(self.v2)) {
            let fallback = 9.9;
            ret_vec
                .push(range_checker::Error::Fallback {
                    ident: "v2".to_owned(),
                    value: (self.v2).to_string(),
                    check_statement: "(| x | x > 8.0) (self.v2) && (| x | x > 8.0) (self.v2)"
                        .to_owned(),
                    fallback: fallback.to_string(),
                });
            self.v2 = fallback;
        }
        if !((..-1).contains(&self.v3) || (..-1).contains(&self.v3)) {
            let fallback = (|x| {
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &["test fallback closure.\n"],
                            &[],
                        ),
                    );
                };
                x - 5
            })(self.v3);
            ret_vec
                .push(range_checker::Error::Fallback {
                    ident: "v3".to_owned(),
                    value: (self.v3).to_string(),
                    check_statement: "(.. - 1).contains(& self.v3) || (.. - 1).contains(& self.v3)"
                        .to_owned(),
                    fallback: fallback.to_string(),
                });
            self.v3 = fallback;
        }
        if !failed { Ok(ret_vec) } else { Err(ret_vec) }
    }
}
```