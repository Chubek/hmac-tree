pub mod binary_rep {
    use std::fmt::Binary;

    pub fn bytes_to_binary(v: &Vec<u8>) -> String {
        let mut res = v
                        .iter()
                        .map(|c| format!("{:08b}", c))
                        .collect::<Vec<String>>()
                        .join("");

        res
    }

    pub fn binary_to_int(s: &String) -> u64 {
        let int = {
                match u64::from_str_radix(s, 2) {
                    Ok(i) => i,
                    Err(e) => panic!("{:?}", e),
                }
        };

        int
    }

    pub fn integer_to_binary<T: Binary>(int: T, padding: u8) -> String {
        match padding {
            8 => format!("{:08b}", int),
            16 => format!("{:016b}", int),
            32 => format!("{:032b}", int),
            64 => format!("{:064b}", int),
            128 => format!("{:0128b}", int),
            _ => format!("{:b}", int)
        }

    }
}

pub mod hex_rep {
    pub fn decimal_to_hex(dec: u64) -> String {
        format!("{:016x}", dec)
    }

    pub fn decode_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect::<Vec<u8>>()
    }

    pub fn encode_hex(v: Vec<u8>) -> String {
        let ret = v
                .iter()
                .map(|x| format!("{:02x}", x))
                .collect::<Vec<String>>()
                .join("");

        ret
    }

}


pub mod serializer {
    pub trait HtreeJsonSerializer {
        fn ser_into_json(&self) -> Vec<u8>;
    }

    pub fn htree_vec_serializer(vec: &Vec<u8>) -> String {
        let mut ret = String::from("");

        for b in vec {
            ret = format!("{ret}{:02x}", b);
        }

        ret
    }
}


pub mod index_parser {
    use std::borrow::{Borrow, BorrowMut};

    pub enum IndexType {
        Root,
        Mixed(Vec<(usize, usize, char)>),
        OnlyLeft(Vec<usize>),
        OnlyRight(Vec<usize>),
    }

    impl From<String> for IndexType {
        fn from(index: String) -> Self {
            let index_no_ws = index
                    .to_lowercase()
                    .replace(" ", "");

            let index_no_root = index_no_ws.replace("root", "");

            let index_split = index_no_root.split("->");
    

            let ret = if index_no_ws.trim() == "root" {
                IndexType::Root
            } else if !index.to_lowercase().contains("l") {
                let ret = index_split.map(|x| {
                    let uo = x.split("r").map(|xi| {
                        let u = usize::from_str_radix(xi, 10).unwrap();

                        u
                    }).next().unwrap();

                    uo
                }).collect::<Vec<usize>>();

                IndexType::OnlyRight(ret)            
            } else if !index.to_lowercase().contains("r") {
                let ret = index_split.map(|x| {
                    let uo = x.split("l").map(|xi| {
                        let u = usize::from_str_radix(xi, 10).unwrap();

                        u
                    }).next().unwrap();

                    uo
                }).collect::<Vec<usize>>();

                IndexType::OnlyLeft(ret) 
            } else {
                let mut ret: Vec<(usize, usize, char)> = vec![];

                index_split.map(|x| {
                    let chars = x.chars();

                    let mut ph = String::new();

                    let mut last_l = -1;
                    let mut last_r = -1;
                    let mut gotten = false;
                    let mut first_gotten = 'n';

                    for c in chars {    
                        if c.is_numeric() {
                            ph = format!("{ph}{c}");
                        } else {
                            match c {
                                'l' => {
                                    last_l = i32::from_str_radix(&ph, 10).unwrap();

                                    ph = String::new();

                                    if !gotten {
                                        first_gotten = 'l';
                                    }

                                    gotten = true;
                                },
                                'r' => {
                                    last_r = i32::from_str_radix(&ph, 10).unwrap();

                                    ph = String::new();

                                    if !gotten {
                                        first_gotten = 'r';
                                    }

                                    gotten = true;
                                },
                                _ => panic!("Wrong index"),
                            }
                        }

                        if last_l != -1 && last_r != -1 {
                            let r = last_r.abs() as usize;
                            let l = last_l.abs() as usize;

                            ret.push((l, r, first_gotten));

                            (last_l, last_r, gotten, first_gotten) = (-1, -1, false, 'm');
                        }
                    }

                });
                
                IndexType::Mixed(ret) 
            };

            ret
        }   
    }
}