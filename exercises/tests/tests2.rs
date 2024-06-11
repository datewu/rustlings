// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug, PartialEq)]
struct WBB;

#[cfg(test)]
mod tests {
    // module
    use super::*;
    #[test]
    fn you_can_assert_eq() {
        // asset_eq arg1 arg2 the same type: Debug + PartialEq // trait
        // i32
        let a = WBB;

        let b = WBB;
        assert_eq!(a, b);
    }
}
