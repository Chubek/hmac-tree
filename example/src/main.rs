use hmac_procs::htree_json;
use hmac_tree::encoders::serializer::HtreeJsonSerializer;

#[htree_json(ignore = [bb])]
pub struct T {
    tt: Vec<u8>,
    bb: String,
}


fn main() {
    let t = T { tt: vec![0u8, 38u8], bb: "SSS".to_string() };

    let j = t.ser_into_json();

    println!("{}", String::from_utf8(j).unwrap())

}
