pub type Digit = u64;

/// Computes `a` + `b` using long addition, and assigns the result to `a`. If the computation
/// overflowed, this function returns `true`. Otherwise, it returns `false`.
pub fn add_assign_slice(a: &mut [Digit], b: &[Digit]) -> bool {
    let (a_suffix, a_prefix) = a.split_at_mut(b.len());
    let mut carry = false;
    for (a_item, b_item) in a_suffix.iter_mut().zip(b) {
        add_assign_with_carry(a_item, *b_item, &mut carry);
    }
    if carry {
        for a_item in a_prefix {
           add_assign_with_carry(a_item, 0, &mut carry);
           if !carry {
               break;
           } 
        }
    }
    carry
}

/// Computes `a` + `b` + `carry`, and assigns the result to `a`. If the computation overflowed,
/// `carry` is set to `true`. Otherwise, it is set to `false`.
pub fn add_assign_with_carry(a: &mut Digit, b: Digit, carry: &mut bool) {
    let (a_out, carry_out) = a.overflowing_add(b);
    if carry_out {
        *a = a_out + *carry as Digit;
        *carry = carry_out;
    } else {
        let (a_out, carry_out) = a_out.overflowing_add(*carry as Digit);
        *a = a_out;
        *carry = carry_out;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assign_slice() {
        let max_value = Digit::max_value();

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [0, 0]);

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[1, 0]) == false);
        assert!(a == [1, 0]);

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == false);
        assert!(a == [max_value, 0]);

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[0, 1]) == false);
        assert!(a == [0, 1]);

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[1, 1]) == false);
        assert!(a == [1, 1]);

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == false);
        assert!(a == [max_value, 1]);

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == false);
        assert!(a == [0, max_value]);

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == false);
        assert!(a == [1, max_value]);

        let mut a = [0, 0];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == false);
        assert!(a == [max_value, max_value]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [1, 0]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[1, 0]) == false);
        assert!(a == [2, 0]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == false);
        assert!(a == [0, 1]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[0, 1]) == false);
        assert!(a == [1, 1]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[1, 1]) == false);
        assert!(a == [2, 1]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == false);
        assert!(a == [0, 2]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == false);
        assert!(a == [1, max_value]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == false);
        assert!(a == [2, max_value]);

        let mut a = [1, 0];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == true);
        assert!(a == [0, 0]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [max_value, 0]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[1, 0]) == false);
        assert!(a == [0, 1]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == false);
        assert!(a == [max_value - 1, 1]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[0, 1]) == false);
        assert!(a == [max_value, 1]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[1, 1]) == false);
        assert!(a == [0, 2]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == false);
        assert!(a == [max_value - 1, 2]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == false);
        assert!(a == [max_value, max_value]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == true);
        assert!(a == [0, 0]);

        let mut a = [max_value, 0];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == true);
        assert!(a == [max_value - 1, 0]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [0, 1]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[1, 0]) == false);
        assert!(a == [1, 1]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == false);
        assert!(a == [max_value, 1]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[0, 1]) == false);
        assert!(a == [0, 2]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[1, 1]) == false);
        assert!(a == [1, 2]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == false);
        assert!(a == [max_value, 2]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == true);
        assert!(a == [0, 0]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == true);
        assert!(a == [1, 0]);

        let mut a = [0, 1];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == true);
        assert!(a == [max_value, 0]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [1, 1]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[1, 0]) == false);
        assert!(a == [2, 1]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == false);
        assert!(a == [0, 2]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[0, 1]) == false);
        assert!(a == [1, 2]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[1, 1]) == false);
        assert!(a == [2, 2]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == false);
        assert!(a == [0, 3]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == true);
        assert!(a == [1, 0]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == true);
        assert!(a == [2, 0]);

        let mut a = [1, 1];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == true);
        assert!(a == [0, 1]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [max_value, 1]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[1, 0]) == false);
        assert!(a == [0, 2]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == false);
        assert!(a == [max_value - 1, 2]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[0, 1]) == false);
        assert!(a == [max_value, 2]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[1, 1]) == false);
        assert!(a == [0, 3]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == false);
        assert!(a == [max_value - 1, 3]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == true);
        assert!(a == [max_value, 0]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == true);
        assert!(a == [0, 1]);

        let mut a = [max_value, 1];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == true);
        assert!(a == [max_value - 1, 1]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [0, max_value]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[1, 0]) == false);
        assert!(a == [1, max_value]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == false);
        assert!(a == [max_value, max_value]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[0, 1]) == true);
        assert!(a == [0, 0]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[1, 1]) == true);
        assert!(a == [1, 0]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == true);
        assert!(a == [max_value, 0]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == true);
        assert!(a == [0, max_value - 1]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == true);
        assert!(a == [1, max_value -1]);

        let mut a = [0, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == true);
        assert!(a == [max_value, max_value - 1]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [1, max_value]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[1, 0]) == false);
        assert!(a == [2, max_value]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == true);
        assert!(a == [0, 0]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[0, 1]) == true);
        assert!(a == [1, 0]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[1, 1]) == true);
        assert!(a == [2, 0]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == true);
        assert!(a == [0, 1]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == true);
        assert!(a == [1, max_value - 1]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == true);
        assert!(a == [2, max_value -1]);

        let mut a = [1, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == true);
        assert!(a == [0, max_value]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[0, 0]) == false);
        assert!(a == [max_value, max_value]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[1, 0]) == true);
        assert!(a == [0, 0]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, 0]) == true);
        assert!(a == [max_value - 1, 0]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[0, 1]) == true);
        assert!(a == [max_value, 0]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[1, 1]) == true);
        assert!(a == [0, 1]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, 1]) == true);
        assert!(a == [max_value - 1, 1]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[0, max_value]) == true);
        assert!(a == [max_value, max_value - 1]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[1, max_value]) == true);
        assert!(a == [0, max_value]);

        let mut a = [max_value, max_value];
        assert!(add_assign_slice(&mut a, &[max_value, max_value]) == true);
        assert!(a == [max_value - 1, max_value]);
    }

    #[test]
    fn test_add_assign_with_carry() {
        let max_value = Digit::max_value();

        let mut a = 0;
        let mut carry = false;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 0);
        assert!(carry == false);

        let mut a = 0;
        let mut carry = true;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 1);
        assert!(carry == false);

        let mut a = 0;
        let mut carry = false;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 1);
        assert!(carry == false);

        let mut a = 0;
        let mut carry = true;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 2);
        assert!(carry == false);

        let mut a = 0;
        let mut carry = false;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == max_value);
        assert!(carry == false);

        let mut a = 0;
        let mut carry = true;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == 0);
        assert!(carry == true);

        let mut a = 1;
        let mut carry = false;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 1);
        assert!(carry == false);

        let mut a = 1;
        let mut carry = true;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 2);
        assert!(carry == false);

        let mut a = 1;
        let mut carry = false;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 2);
        assert!(carry == false);

        let mut a = 1;
        let mut carry = true;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 3);
        assert!(carry == false);

        let mut a = 1;
        let mut carry = false;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == 0);
        assert!(carry == true);

        let mut a = 1;
        let mut carry = true;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == 1);
        assert!(carry == true);

        let mut a = max_value;
        let mut carry = false;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == max_value);
        assert!(carry == false);

        let mut a = max_value;
        let mut carry = true;
        add_assign_with_carry(&mut a, 0, &mut carry);
        assert!(a == 0);
        assert!(carry == true);

        let mut a = max_value;
        let mut carry = false;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 0);
        assert!(carry == true);
        
        let mut a = max_value;
        let mut carry = true;
        add_assign_with_carry(&mut a, 1, &mut carry);
        assert!(a == 1);
        assert!(carry == true);

        let mut a = max_value;
        let mut carry = false;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == max_value - 1);
        assert!(carry == true);

        let mut a = max_value;
        let mut carry = true;
        add_assign_with_carry(&mut a, max_value, &mut carry);
        assert!(a == max_value);
        assert!(carry == true);
    }
}
