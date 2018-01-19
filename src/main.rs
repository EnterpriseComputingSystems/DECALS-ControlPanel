
extern crate DECALS_base;
use DECALS_base::{Network};


fn main() {

    let mut interests: Vec<String> = Vec::new();
    interests.push("SysStatus".to_string());

    let net: Network = Network::new(interests);

    println!("{}", net.get_num_devices());

}
