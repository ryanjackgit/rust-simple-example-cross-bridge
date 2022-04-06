use std::env;
use std::str::FromStr;

use tracing::{info,error,debug};

pub mod mainbridge;
pub mod sidebridge;


//  key to Contract address,After deploye contract

const MAIN_TOKEN_CONTRACT_ADDRESS:&'static str="0x2D087e51eD54a348de214E0B39d85Be5976D7779";
const MAIN_BRIDGE_CONTRACT_ADDRESS:&'static str="0x923C2d576eEb35644447d177940088Fa2a94b5d6";
const SIDE_TOKEN_CONTRACT_ADDRESS:&'static str="0xE0daEd63ce045833C22862A7fA3a95527DF8bcdC";
const SIDE_BRIDGE_CONTRACT_ADDRESS:&'static str="0x5e4A4859d1Af4A315080DEb43B9bFB01Fa2016ef";

// private key as gateway sign transactions. sercrity devlepemt and cross bridge key future
// no 0x begin
const PRIVATE_KEY:&'static str="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rust_simple_example_cross_bridge=debug,web3=debug");
    }


    tracing_subscriber::fmt::init();

//  concrrrent run in background
tokio::spawn( async {
if let Err(e) = mainbridge::listen_event_erc20_to_mainbridge_rinkby().await {
    error!("main happend--{:?}",e);
}
});

info!("cross bridge  ERC20 :   Rankby --->   Ropsten     Ropsten ---> Rankby");

if let Err(e) = sidebridge::listen_event_erc20_to_sidebridge_ropsten().await {
    error!("side happend--{:?}",e);
}




Ok(())

}








