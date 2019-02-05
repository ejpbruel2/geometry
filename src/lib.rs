pub type Digit = u64;

/// Computes `a` + `b` + `carry_in`.
/// 
/// Returns a tuple `(carry_out, sum)`, where `carry_out` represents the most significant bit, and
/// `sum` the remaining bits of the result.
pub fn add_with_carry(a: Digit, b: Digit, carry_in: bool) -> (bool, Digit) {
    let (sum, carry_out) = a.overflowing_add(b);
    if carry_out {
        debug_assert!(sum < Digit::max_value()); 
        (true, sum + carry_in as Digit)
    } else {
        let (sum, carry_out) = sum.overflowing_add(carry_in as Digit);
        (carry_out, sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_with_carry() {
        let max_value = Digit::max_value();
        assert!(add_with_carry(0, 0, false) == (false, 0));
        assert!(add_with_carry(0, 0, true) == (false, 1));
        assert!(add_with_carry(0, 1, false) == (false, 1));
        assert!(add_with_carry(0, 1, true) == (false, 2));
        assert!(add_with_carry(0, max_value, false) == (false, max_value));
        assert!(add_with_carry(0, max_value, true) == (true, 0));
        assert!(add_with_carry(1, 0, false) == (false, 1));
        assert!(add_with_carry(1, 0, true) == (false, 2));
        assert!(add_with_carry(1, 1, false) == (false, 2));
        assert!(add_with_carry(1, 1, true) == (false, 3));
        assert!(add_with_carry(1, max_value, false) == (true, 0));
        assert!(add_with_carry(1, max_value, true) == (true, 1));
        assert!(add_with_carry(max_value, 0, false) == (false, max_value));
        assert!(add_with_carry(max_value, 0, true) == (true, 0));
        assert!(add_with_carry(max_value, 1, false) == (true, 0));
        assert!(add_with_carry(max_value, 1, true) == (true, 1));
        assert!(add_with_carry(max_value, max_value, false) == (true, max_value - 1));
        assert!(add_with_carry(max_value, max_value, true) == (true, max_value));
    }
}
