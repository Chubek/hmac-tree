use hmac_tree::sha512::Sha512Hash;


fn main() {
    let h = Sha512Hash::from_str("a");

    println!("{}", h.get_hex_digest());

}
