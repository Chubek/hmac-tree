use hmac_tree::hmac::Hmac;


fn main() {
    let hmac = Hmac::from_str("lll", "sss");

    println!("{}", hmac.calculate().1)

}
