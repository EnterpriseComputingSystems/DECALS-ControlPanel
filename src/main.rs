
extern crate DECALS_base;
use DECALS_base::Network;


fn main() {

    let net: Network = Network::new();

    println!("{} . {}", net.get_num_devices(), net.connected);

}
