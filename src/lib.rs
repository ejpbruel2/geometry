/// Computes `a` += `b`, using long addition.
/// 
/// If the computation overflows, the carry digit is returned.
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

/// Computes `a` += `b` + `carry`.
/// 
/// If the computation overflows, `carry` is set to the carry digit.
pub fn add_assign_with_carry(a: &mut u32, b: u32, carry: &mut u32) {
    let result = *a as u64 + b as u64 + *carry as u64;
    *a = (result & (1 << 32) - 1) as u32;
    *carry = (result >> 32) as u32;
}

/// Computes `a` += `b` * `c` + `carry`.
/// 
/// If the computation overflows, `carry` is set to the carry digit.
pub fn mul_add_assign_with_carry(a: &mut u32, b: u32, c: u32, carry: &mut u32) {
    let result = *a as u64 + b as u64 * c as u64 + *carry as u64;
    *a = (result & (1 << 32) - 1) as u32;
    *carry = (result >> 32) as u32; 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::u32;
    const MAX: u32 = u32::MAX;

    #[test]
    fn test_add_assign_big() {
        static TESTS: &[(&[u32], &[u32], &[u32], u32)] = &[
            (&[0, 0], &[0, 0], &[0, 0], 0),
            (&[0, 0], &[1, 0], &[1, 0], 0),
            (&[0, 0], &[MAX, 0], &[MAX, 0], 0),
            (&[0, 0], &[0, 1], &[0, 1], 0),
            (&[0, 0], &[1, 1], &[1, 1], 0),
            (&[0, 0], &[MAX, 1], &[MAX, 1], 0),
            (&[0, 0], &[0, MAX], &[0, MAX], 0),
            (&[0, 0], &[1, MAX], &[1, MAX], 0),
            (&[0, 0], &[MAX, MAX], &[MAX, MAX], 0),
            (&[1, 0], &[0, 0], &[1, 0], 0),
            (&[1, 0], &[1, 0], &[2, 0], 0),
            (&[1, 0], &[MAX, 0], &[0, 1], 0),
            (&[1, 0], &[0, 1], &[1, 1], 0),
            (&[1, 0], &[1, 1], &[2, 1], 0),
            (&[1, 0], &[MAX, 1], &[0, 2], 0),
            (&[1, 0], &[0, MAX], &[1, MAX], 0),
            (&[1, 0], &[1, MAX], &[2, MAX], 0),
            (&[1, 0], &[MAX, MAX], &[0, 0], 1),
            (&[MAX, 0], &[0, 0], &[MAX, 0], 0),
            (&[MAX, 0], &[1, 0], &[0, 1], 0),
            (&[MAX, 0], &[MAX, 0], &[MAX - 1, 1], 0),
            (&[MAX, 0], &[0, 1], &[MAX, 1], 0),
            (&[MAX, 0], &[1, 1], &[0, 2], 0),
            (&[MAX, 0], &[MAX, 1], &[MAX - 1, 2], 0),
            (&[MAX, 0], &[0, MAX], &[MAX, MAX], 0),
            (&[MAX, 0], &[1, MAX], &[0, 0], 1),
            (&[MAX, 0], &[MAX, MAX], &[MAX - 1, 0], 1),
            (&[0, 1], &[0, 0], &[0, 1], 0),
            (&[0, 1], &[1, 0], &[1, 1], 0),
            (&[0, 1], &[MAX, 0], &[MAX, 1], 0),
            (&[0, 1], &[0, 1], &[0, 2], 0),
            (&[0, 1], &[1, 1], &[1, 2], 0),
            (&[0, 1], &[MAX, 1], &[MAX, 2], 0),
            (&[0, 1], &[0, MAX], &[0, 0], 1),
            (&[0, 1], &[1, MAX], &[1, 0], 1),
            (&[0, 1], &[MAX, MAX], &[MAX, 0], 1),
            (&[1, 1], &[0, 0], &[1, 1], 0),
            (&[1, 1], &[1, 0], &[2, 1], 0),
            (&[1, 1], &[MAX, 0], &[0, 2], 0),
            (&[1, 1], &[0, 1], &[1, 2], 0),
            (&[1, 1], &[1, 1], &[2, 2], 0),
            (&[1, 1], &[MAX, 1], &[0, 3], 0),
            (&[1, 1], &[0, MAX], &[1, 0], 1),
            (&[1, 1], &[1, MAX], &[2, 0], 1),
            (&[1, 1], &[MAX, MAX], &[0, 1], 1),
            (&[MAX, 1], &[0, 0], &[MAX, 1], 0),
            (&[MAX, 1], &[1, 0], &[0, 2], 0),
            (&[MAX, 1], &[MAX, 0], &[MAX - 1, 2], 0),
            (&[MAX, 1], &[0, 1], &[MAX, 2], 0),
            (&[MAX, 1], &[1, 1], &[0, 3], 0),
            (&[MAX, 1], &[MAX, 1], &[MAX - 1, 3], 0),
            (&[MAX, 1], &[0, MAX], &[MAX, 0], 1),
            (&[MAX, 1], &[1, MAX], &[0, 1], 1),
            (&[MAX, 1], &[MAX, MAX], &[MAX - 1, 1], 1),
            (&[0, MAX], &[0, 0], &[0, MAX], 0),
            (&[0, MAX], &[1, 0], &[1, MAX], 0),
            (&[0, MAX], &[MAX, 0], &[MAX, MAX], 0),
            (&[0, MAX], &[0, 1], &[0, 0], 1),
            (&[0, MAX], &[1, 1], &[1, 0], 1),
            (&[0, MAX], &[MAX, 1], &[MAX, 0], 1),
            (&[0, MAX], &[0, MAX], &[0, MAX - 1], 1),
            (&[0, MAX], &[1, MAX], &[1, MAX - 1], 1),
            (&[0, MAX], &[MAX, MAX], &[MAX, MAX - 1], 1),  
            (&[1, MAX], &[0, 0], &[1, MAX], 0),
            (&[1, MAX], &[1, 0], &[2, MAX], 0),
            (&[1, MAX], &[MAX, 0], &[0, 0], 1),
            (&[1, MAX], &[0, 1], &[1, 0], 1),
            (&[1, MAX], &[1, 1], &[2, 0], 1),
            (&[1, MAX], &[MAX, 1], &[0, 1], 1),
            (&[1, MAX], &[0, MAX], &[1, MAX - 1], 1),
            (&[1, MAX], &[1, MAX], &[2, MAX - 1], 1),
            (&[1, MAX], &[MAX, MAX], &[0, MAX], 1),
            (&[MAX, MAX], &[0, 0], &[MAX, MAX], 0),
            (&[MAX, MAX], &[1, 0], &[0, 0], 1),
            (&[MAX, MAX], &[MAX, 0], &[MAX - 1, 0], 1),
            (&[MAX, MAX], &[0, 1], &[MAX, 0], 1),
            (&[MAX, MAX], &[1, 1], &[0, 1], 1),
            (&[MAX, MAX], &[MAX, 1], &[MAX - 1, 1], 1),
            (&[MAX, MAX], &[0, MAX], &[MAX, MAX - 1], 1),
            (&[MAX, MAX], &[1, MAX], &[0, MAX], 1),
            (&[MAX, MAX], &[MAX, MAX], &[MAX - 1, MAX], 1),
        ];

        for (a_in, b, a_out, carry_out) in TESTS {
            let mut a = a_in.to_vec();
            assert!(add_assign_big(a.as_mut_slice(), b) == *carry_out);
            assert!(&a == a_out);
        }
    }

    #[test]
    fn test_add_assign_with_carry() {
        const TESTS: &[(u32, u32, u32, u32, u32)] = &[
            (0, 0, 0, 0, 0),
            (0, 0, 1, 1, 0),
            (0, 1, 0, 1, 0),
            (0, 1, 1, 2, 0),
            (0, MAX, 0, MAX, 0),
            (0, MAX, 1, 0, 1),
            (1, 0, 0, 1, 0),
            (1, 0, 1, 2, 0),
            (1, 1, 0, 2, 0),
            (1, 1, 1, 3, 0),
            (1, MAX, 0, 0, 1),
            (1, MAX, 1, 1, 1),
            (MAX, 0, 0, MAX, 0),
            (MAX, 0, 1, 0, 1),
            (MAX, 1, 0, 0, 1),
            (MAX, 1, 1, 1, 1),
            (MAX, MAX, 0, MAX - 1, 1),
            (MAX, MAX, 1, MAX, 1),
        ];

        for (a_in, b, carry_in, a_out, carry_out) in TESTS {
            let mut a = *a_in;
            let mut carry = *carry_in;
            add_assign_with_carry(&mut a, *b, &mut carry);
            assert!(a == *a_out);
            assert!(carry == *carry_out);
        }
    }

    #[test]
    fn test_mul_add_assign_with_carry() {
        static TESTS: &[(u32, u32, u32, u32, u32, u32)] = &[
            (0, 0, 0, 0, 0, 0),
            (0, 0, 0, 1, 1, 0),
            (0, 0, 1, 0, 0, 0),
            (0, 0, 1, 1, 1, 0),
            (0, 0, 2, 0, 0, 0),
            (0, 0, 2, 1, 1, 0),
            (0, 0, MAX, 0, 0, 0),
            (0, 0, MAX, 1, 1, 0),
            (0, 1, 0, 0, 0, 0),
            (0, 1, 0, 1, 1, 0),
            (0, 1, 1, 0, 1, 0),
            (0, 1, 1, 1, 2, 0),
            (0, 1, 2, 0, 2, 0),
            (0, 1, 2, 1, 3, 0),
            (0, 1, MAX, 0, MAX, 0),
            (0, 1, MAX, 1, 0, 1),
            (0, 2, 0, 0, 0, 0),
            (0, 2, 0, 1, 1, 0),
            (0, 2, 1, 0, 2, 0),
            (0, 2, 1, 1, 3, 0),
            (0, 2, 2, 0, 4, 0),
            (0, 2, 2, 1, 5, 0),
            (0, 2, MAX, 0, MAX - 1, 1),
            (0, 2, MAX, 1, MAX, 1),
            (0, MAX, 0, 0, 0, 0),
            (0, MAX, 0, 1, 1, 0),
            (0, MAX, 1, 0, MAX, 0),
            (0, MAX, 1, 1, 0, 1),
            (0, MAX, 2, 0, MAX - 1, 1),
            (0, MAX, 2, 1, MAX, 1),
            (0, MAX, MAX, 0, 1, MAX - 1),
            (0, MAX, MAX, 1, 2, MAX - 1),
        ];

        for (a_in, b, c, carry_in, a_out, carry_out) in TESTS {
            let mut a = *a_in;
            let mut carry = *carry_in;
            mul_add_assign_with_carry(&mut a, *b, *c, &mut carry);
            assert!(a == *a_out);
            assert!(carry == *carry_out);
        }
    }
}
