pub mod chunk_utils {
    pub fn rotate_right_by_n_bits(b: &u64, n: u64) -> u64 {
        (b >> n) | (b << (64 - n))
    }

    pub fn shift_right_by_n_bits(b: &u64, n: u64) -> u64 {
        b >> n
    }
}

pub mod macro_rules {
    use paste;

    #[macro_export]
    macro_rules! scooch {
        ($obj: ident, $a_rep: ident, $b_rep: ident, $c_rep: ident, $d_rep: ident, $e_rep: ident, $f_rep: ident, $g_rep: ident,$h_rep: ident) => {
            (
                $obj.A, $obj.B, $obj.C, $obj.D, $obj.E, $obj.F, $obj.G, $obj.H,
            ) = {
                (
                    $obj.$a_rep,
                    $obj.$b_rep,
                    $obj.$c_rep,
                    $obj.$d_rep,
                    $obj.$e_rep,
                    $obj.$f_rep,
                    $obj.$g_rep,
                    $obj.$h_rep,
                )
            };
        };
    }

    #[macro_export]
    macro_rules! add_unchecked {
        ( $( $x: ident ), * ) => {
            {
              let mut fin = 0u64;

              $(
                  fin = crate::utils::math::no_overflow_add(fin, $x);
              )*

              fin

            }
        }

    }

    #[macro_export]
    macro_rules! add_unchecked_field_swap {
        ($name:ident { $($field:literal),* }, $suff: literal) => {
            $(
                paste::paste! {
                    $name.[<$field>] = crate::utils::math::no_overflow_add($name.[< $field >], $name.[<$field $suff>]);
                }
            )*

            $(
                paste::paste! {
                    $name.[<$field $suff>] = $name.[<$field>];
                }
            )*

        };
    }
}

pub mod math {
    pub fn no_overflow_add(a_in: u64, b_in: u64) -> u64 {
        let (a, b) = (a_in as u128, b_in as u128);

        let mut carry = a & b;
        let mut res = a ^ b;

        while carry != 0 {
            let shifted_carry = carry << 1;
            carry = res & shifted_carry;
            res = res ^ shifted_carry;
        }

        let ret = match res > u64::MAX as u128 {
            true => (res - u64::MAX as u128) as u64 - 1,
            false => res as u64,
        };

        ret
    }
}
