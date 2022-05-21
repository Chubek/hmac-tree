pub mod ChunkUtils {
    pub fn rotate_right_by_n_bits(b: &u64, n: u64) -> u64 {
        (b >> n) | (b << (64 - n))
    }

    pub fn shift_right_by_n_bits(b: &u64, n: u64) -> u64 {
        b >> n
    }
}

pub mod MacroRules {

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
}
