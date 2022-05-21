#[allow(non_snake_case)]
mod Sha512Util {
    use std::ops::Add;

    use crate::constants::SHA512_PRIME;
    use crate::encoders::{BinaryRep, HexRep};
    use crate::utils::ChunkUtils::{rotate_right_by_n_bits, shift_right_by_n_bits};
    use crate::scooch;

    struct Chunks(Vec<Vec<u64>>);

    impl Chunks {
        pub fn new() -> Self {
            let chunks: Vec<Vec<u64>> = Vec::new();

            Chunks(chunks)
        }

        pub fn operate(&mut self, init_val: &String, num_chunks: usize) {
            let Chunks(chunks) = self;

            let mut slice_ind_beg = 0usize;

            for i in 0usize..num_chunks {
                let mut chunk: Vec<u64> = vec![];

                let slice_ind_end = slice_ind_beg + 1024;

                let concerned_block = &init_val[(slice_ind_beg..slice_ind_end)];

                slice_ind_beg = slice_ind_end;

                for j in (0usize..1024usize).step_by(64) {
                    let val = &concerned_block[(j..j + 64)];

                    let int = BinaryRep::binary_to_int(&val.to_string());

                    chunk.push(int);
                }

                for m in 16..80 {
                    pad_with_words(&mut chunk, m);
                }

                chunks.push(chunk);
            }
        }
    }

    fn do_a(b: u64) -> u64 {
        let one = rotate_right_by_n_bits(&b, 19);
        let two = rotate_right_by_n_bits(&b, 61);
        let three = shift_right_by_n_bits(&b, 6);

        let ret_a = one ^ two ^ three;

        ret_a
    }

    fn do_c(b: u64) -> u64 {
        let one = rotate_right_by_n_bits(&b, 1);
        let two = rotate_right_by_n_bits(&b, 8);
        let three = shift_right_by_n_bits(&b, 7);

        let ret_c = one ^ two ^ three;

        ret_c
    }

    fn pad_with_words(chunk: &mut Vec<u64>, m: usize) {
        let a_el = chunk[m - 2];
        let c_el = chunk[m - 15];

        let A = do_a(a_el);
        let C = do_c(c_el);

        let B = chunk[m - 7];
        let D = chunk[m - 16];

        let added = A + B + C + D;

        chunk[m] = added;
    }

    pub struct Sha512Message {
        original: Vec<u8>,
        message: String,
        chunks: Chunks,
        num_chunks: usize,
    }

    impl Sha512Message {
        pub fn new(original: Vec<u8>) -> Self {
            let mut message = BinaryRep::bytes_to_binary(&original);
            let original_len = message.len();

            message.push_str("1");

            let val = message.len() % 1024;

            let to_be_added = {
                match 1024 - val >= 128 {
                    true => 1024 - val,
                    false => 2048 - val,
                }
            };

            message.push_str(&"0".repeat(to_be_added - 128));

            let int_to_be_added = BinaryRep::integer_to_binary(original_len, 128);

            message.push_str(int_to_be_added.as_str());

            let len_message_fin = message.len();

            let num_chunks = len_message_fin / 1024;

            let mut chunks = Chunks::new();

            chunks.operate(&message, num_chunks.clone());

            Sha512Message {
                original,
                message,
                chunks,
                num_chunks,
            }
        }

        pub fn get_block_ref(&self) -> &Vec<Vec<u64>> {
            let Chunks(blocks) = &self.chunks;

            blocks
        }
    }

    pub struct Sha512Buffer {
        A: u64,
        B: u64,
        C: u64,
        D: u64,
        E: u64,
        F: u64,
        G: u64,
        H: u64,
        APrev: u64,
        BPrev: u64,
        CPrev: u64,
        DPrev: u64,
        EPrev: u64,
        FPrev: u64,
        GPrev: u64,
        HPrev: u64,
    }

    impl Sha512Buffer {
        pub fn new() -> Self {
            Sha512Buffer {
                A: 0x6a09e667f3bcc908,
                B: 0xbb67ae8584caa73b,
                C: 0x3c6ef372fe94f82b,
                D: 0xa54ff53a5f1d36f1,
                E: 0x510e527fade682d1,
                F: 0x9b05688c2b3e6c1f,
                G: 0x1f83d9abfb41bd6b,
                H: 0x5be0cd19137e2179,
                APrev: 0x6a09e667f3bcc908,
                BPrev: 0xbb67ae8584caa73b,
                CPrev: 0x3c6ef372fe94f82b,
                DPrev: 0xa54ff53a5f1d36f1,
                EPrev: 0x510e527fade682d1,
                FPrev: 0x9b05688c2b3e6c1f,
                GPrev: 0x1f83d9abfb41bd6b,
                HPrev: 0x5be0cd19137e2179,
            }
        }

        fn add_and_set_prev(&mut self) {
            self.A += self.APrev;
            self.B += self.BPrev;
            self.C += self.CPrev;
            self.D += self.DPrev;
            self.E += self.EPrev;
            self.F += self.FPrev;
            self.G += self.GPrev;
            self.H += self.HPrev;

            self.APrev = self.A;
            self.BPrev = self.B;
            self.CPrev = self.C;
            self.DPrev = self.D;
            self.EPrev = self.E;
            self.FPrev = self.F;
            self.GPrev = self.G;
            self.HPrev = self.H;
        }

        fn major_value(&self) -> u64 {
            (self.A & self.B) ^ (self.B & self.C) ^ (self.C & self.A)
        }

        fn ch_val(&self) -> u64 {
            (self.E & self.F) ^ (!self.E & self.G)
        }

        fn sigma_e(&self) -> u64 {
            let e_14 = rotate_right_by_n_bits(&self.E, 14);
            let e_18 = rotate_right_by_n_bits(&self.E, 18);
            let e_41 = rotate_right_by_n_bits(&self.E, 41);

            let res_e = e_14 ^ e_18 ^ e_41;

            res_e
        }

        fn sigma_a(&self) -> u64 {
            let a_28 = rotate_right_by_n_bits(&self.A, 28);
            let a_34 = rotate_right_by_n_bits(&self.A, 34);
            let a_39 = rotate_right_by_n_bits(&self.A, 39);

            let res_a = a_28 ^ a_34 ^ a_39;

            res_a
        }

        fn rotate(&mut self, prime_k :u64, message_k: &u64) {
            let T1 = self.H + self.ch_val() + self.sigma_e() + message_k + prime_k;
            let T2 = self.sigma_a() + self.major_value();

            self.D += T1;
            self.H += T1 + T2;
        }

        pub fn process_block(&mut self, chunk_vec: &Vec<u64>) {
            for i in 0..80 {
                if i != 0 {
                    scooch!(self, H, A, B, C, D, E, F, G);
                }

                let message_k = chunk_vec[i];
                let prime_k = SHA512_PRIME[i];

                self.rotate(prime_k, &message_k);
            }

            scooch!(self, H, A, B, C, D, E, F, G);
            
            self.add_and_set_prev();
        }

        pub fn get_hex_rep(&self) -> String {
            let A = HexRep::decimal_to_hex(self.A);
            let B = HexRep::decimal_to_hex(self.B);
            let C = HexRep::decimal_to_hex(self.C);
            let D = HexRep::decimal_to_hex(self.D);
            let E = HexRep::decimal_to_hex(self.E);
            let F = HexRep::decimal_to_hex(self.F);
            let G = HexRep::decimal_to_hex(self.G);
            let H = HexRep::decimal_to_hex(self.H);

            format!("{}{}{}{}{}{}{}{}", A, B, C, D, E, F, G, H)
        }

        pub fn get_byte_rep(&self) -> Vec<u8> {
            let hex_rep = self.get_hex_rep();

            HexRep::decode_hex(hex_rep.as_str())
        }

    }
}


pub struct Sha512Hash {
    message: Sha512Util::Sha512Message,
    buffer: Sha512Util::Sha512Buffer,
}

impl Sha512Hash {
    pub fn from_bytes(s: Vec<u8>) -> Self {
        let message = Sha512Util::Sha512Message::new(s);
        let buffer = Sha512Util::Sha512Buffer::new();

        let mut obj = Sha512Hash {message, buffer};

        obj.calculate();

        obj
    }

    pub fn from_str(s: &str) -> Self {
        Self::from_bytes(s.as_bytes().to_vec())
    }

    fn calculate(&mut self) {
        for block in self.message.get_block_ref() {
            self.buffer.process_block(block);
        }
    }

    pub fn update_from_bytes(&mut self, s: Vec<u8>) {
        self.message = Sha512Util::Sha512Message::new(s);
        self.calculate();
    }

    pub fn update_from_str(&mut self, s: &str) {
        self.message = Sha512Util::Sha512Message::new(s.as_bytes().to_vec());
        self.calculate();
    }

    pub fn get_digest(&self) -> Vec<u8> {
        self.buffer.get_byte_rep()
    }

    pub fn get_hex_digest(&self) -> String {
        self.buffer.get_hex_rep()
    }
}