/// Computes `a` += `b`, using long addition. If the computation overflows, the carry digit is
/// returned.
pub fn add_assign_big(a: &mut [u32], b: &[u32]) -> u32 {
    let mut carry = 0;
    for (a_i, b_i) in a.iter_mut().zip(b) {
        add_assign_with_carry(a_i, *b_i, &mut carry);
    }
    for a_i in &mut a[b.len()..] {
        if carry == 0 {
            break;
        }
        add_assign_with_carry(a_i, 0, &mut carry);
    }
    carry
}

/// Computes `a` += `b` + `carry`. If the computation overflows, `carry` is set to the carry digit.
pub fn add_assign_with_carry(a: &mut u32, b: u32, carry: &mut u32) {
    let result = *a as u64 + b as u64 + *carry as u64;
    *a = (result & (1 << 32) - 1) as u32;
    *carry = (result >> 32) as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assign_big() {
        let max_value = u32::max_value();

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [0, 0]);

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[1, 0]) == 0);
        assert!(a == [1, 0]);

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 0);
        assert!(a == [max_value, 0]);

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[0, 1]) == 0);
        assert!(a == [0, 1]);

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[1, 1]) == 0);
        assert!(a == [1, 1]);

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 0);
        assert!(a == [max_value, 1]);

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 0);
        assert!(a == [0, max_value]);

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 0);
        assert!(a == [1, max_value]);

        let mut a = [0, 0];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 0);
        assert!(a == [max_value, max_value]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [1, 0]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[1, 0]) == 0);
        assert!(a == [2, 0]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 0);
        assert!(a == [0, 1]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[0, 1]) == 0);
        assert!(a == [1, 1]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[1, 1]) == 0);
        assert!(a == [2, 1]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 0);
        assert!(a == [0, 2]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 0);
        assert!(a == [1, max_value]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 0);
        assert!(a == [2, max_value]);

        let mut a = [1, 0];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 1);
        assert!(a == [0, 0]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [max_value, 0]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[1, 0]) == 0);
        assert!(a == [0, 1]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 0);
        assert!(a == [max_value - 1, 1]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[0, 1]) == 0);
        assert!(a == [max_value, 1]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[1, 1]) == 0);
        assert!(a == [0, 2]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 0);
        assert!(a == [max_value - 1, 2]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 0);
        assert!(a == [max_value, max_value]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 1);
        assert!(a == [0, 0]);

        let mut a = [max_value, 0];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 1);
        assert!(a == [max_value - 1, 0]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [0, 1]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[1, 0]) == 0);
        assert!(a == [1, 1]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 0);
        assert!(a == [max_value, 1]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[0, 1]) == 0);
        assert!(a == [0, 2]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[1, 1]) == 0);
        assert!(a == [1, 2]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 0);
        assert!(a == [max_value, 2]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 1);
        assert!(a == [0, 0]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 1);
        assert!(a == [1, 0]);

        let mut a = [0, 1];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 1);
        assert!(a == [max_value, 0]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [1, 1]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[1, 0]) == 0);
        assert!(a == [2, 1]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 0);
        assert!(a == [0, 2]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[0, 1]) == 0);
        assert!(a == [1, 2]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[1, 1]) == 0);
        assert!(a == [2, 2]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 0);
        assert!(a == [0, 3]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 1);
        assert!(a == [1, 0]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 1);
        assert!(a == [2, 0]);

        let mut a = [1, 1];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 1);
        assert!(a == [0, 1]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [max_value, 1]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[1, 0]) == 0);
        assert!(a == [0, 2]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 0);
        assert!(a == [max_value - 1, 2]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[0, 1]) == 0);
        assert!(a == [max_value, 2]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[1, 1]) == 0);
        assert!(a == [0, 3]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 0);
        assert!(a == [max_value - 1, 3]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 1);
        assert!(a == [max_value, 0]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 1);
        assert!(a == [0, 1]);

        let mut a = [max_value, 1];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 1);
        assert!(a == [max_value - 1, 1]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [0, max_value]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[1, 0]) == 0);
        assert!(a == [1, max_value]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 0);
        assert!(a == [max_value, max_value]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[0, 1]) == 1);
        assert!(a == [0, 0]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[1, 1]) == 1);
        assert!(a == [1, 0]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 1);
        assert!(a == [max_value, 0]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 1);
        assert!(a == [0, max_value - 1]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 1);
        assert!(a == [1, max_value -1]);

        let mut a = [0, max_value];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 1);
        assert!(a == [max_value, max_value - 1]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [1, max_value]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[1, 0]) == 0);
        assert!(a == [2, max_value]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 1);
        assert!(a == [0, 0]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[0, 1]) == 1);
        assert!(a == [1, 0]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[1, 1]) == 1);
        assert!(a == [2, 0]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 1);
        assert!(a == [0, 1]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 1);
        assert!(a == [1, max_value - 1]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 1);
        assert!(a == [2, max_value -1]);

        let mut a = [1, max_value];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 1);
        assert!(a == [0, max_value]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[0, 0]) == 0);
        assert!(a == [max_value, max_value]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[1, 0]) == 1);
        assert!(a == [0, 0]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[max_value, 0]) == 1);
        assert!(a == [max_value - 1, 0]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[0, 1]) == 1);
        assert!(a == [max_value, 0]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[1, 1]) == 1);
        assert!(a == [0, 1]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[max_value, 1]) == 1);
        assert!(a == [max_value - 1, 1]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[0, max_value]) == 1);
        assert!(a == [max_value, max_value - 1]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[1, max_value]) == 1);
        assert!(a == [0, max_value]);

        let mut a = [max_value, max_value];
        assert!(add_assign_big(&mut a, &[max_value, max_value]) == 1);
        assert!(a == [max_value - 1, max_value]);
    }

    #[test]
    fn test_add_assign_with_carry() {
        let max_value = u32::max_value();

        let mut a = 0;
        let mut carry = 0;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 0);
        assert!(carry == 0);

        let mut a = 0;
        let mut carry = 1;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 1);
        assert!(carry == 0);

        let mut a = 0;
        let mut carry = 0;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 1);
        assert!(carry == 0);

        let mut a = 0;
        let mut carry = 1;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 2);
        assert!(carry == 0);

        let mut a = 0;
        let mut carry = 0;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == max_value);
        assert!(carry == 0);

        let mut a = 0;
        let mut carry = 1;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == 0);
        assert!(carry == 1);

        let mut a = 1;
        let mut carry = 0;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 1);
        assert!(carry == 0);

        let mut a = 1;
        let mut carry = 1;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 2);
        assert!(carry == 0);

        let mut a = 1;
        let mut carry = 0;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 2);
        assert!(carry == 0);

        let mut a = 1;
        let mut carry = 1;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 3);
        assert!(carry == 0);

        let mut a = 1;
        let mut carry = 0;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == 0);
        assert!(carry == 1);

        let mut a = 1;
        let mut carry = 1;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == 1);
        assert!(carry == 1);

        let mut a = max_value;
        let mut carry = 0;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == max_value);
        assert!(carry == 0);

        let mut a = max_value;
        let mut carry = 1;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 0);
        assert!(carry == 1);

        let mut a = max_value;
        let mut carry = 0;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 0);
        assert!(carry == 1);
        
        let mut a = max_value;
        let mut carry = 1;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 1);
        assert!(carry == 1);

        let mut a = max_value;
        let mut carry = 0;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == max_value - 1);
        assert!(carry == 1);

        let mut a = max_value;
        let mut carry = 1;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == max_value);
        assert!(carry == 1);
    }
}
