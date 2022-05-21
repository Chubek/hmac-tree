use crate::sha512::Sha512Hash;
use std::iter::repeat;

pub struct Hmac {
    data: Vec<u8>,
    key: Vec<u8>,
}

impl Hmac {
    pub fn from_bytes(data: Vec<u8>, key: Vec<u8>) -> Self {
        if key.len() > 64 {
            panic!("Key length must be less than 64")
        }

        let mut key_mut = key.clone();

        Self::pad_key(&mut key_mut);

        Hmac { key: key_mut, data }
    }


    pub fn from_str(data: &str, key: &str) -> Self {
        if key.len() > 64 {
            panic!("Key length must be less than 64")
        }

        let mut key_mut = key.as_bytes().to_vec();

        Self::pad_key(&mut key_mut);

        Hmac { data: data.as_bytes().to_vec(), key: key_mut }
    }


    fn pad_key(key: &mut Vec<u8>) {
        key.extend(repeat(0x00).take(64 - key.len()).collect::<Vec<u8>>());

    }

    fn xor(&self, pad: Vec<u8>) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];

        let len_p = pad.len();
        let len_k = self.key.len();

        let min = usize::min(len_p, len_k);

        for i in 0..min {
            ret.push(self.key[i] ^ pad[i])
        }

        ret
    }

    pub fn calculate(&self) -> Vec<u8> {
        let ipad: Vec<u8> = repeat(0x36).take(64).collect();
        let opad: Vec<u8> = repeat(0x5c).take(64).collect();

        let mut h_inner = Sha512Hash::from_bytes(self.xor(ipad));
        h_inner.update_from_bytes(self.data.clone());

        let mut h_outer = Sha512Hash::from_bytes(self.xor(opad));
        h_outer.update_from_bytes(h_inner.get_digest());

        h_outer.get_digest()
        
    }
    
    fn verify(&self, other_digest: Vec<u8>) -> bool {
        let own_digest = self.calculate();

        other_digest == own_digest
    }
}