use std::env;
use std::str::FromStr;

use tracing::{info,error,debug};

use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};
use web3::types::{BlockId, BlockNumber,TransactionParameters,Bytes};
use web3::Web3;
use web3::transports::WebSocket;
use web3::contract::tokens::Tokenize;


use secp256k1::SecretKey;

use hex_literal::hex;



#[derive(Debug,Clone)]
pub struct TransferPO {
   pub from:ethabi::ethereum_types::H160,
   pub to:ethabi::ethereum_types::H160,
   pub value:ethabi::ethereum_types::U256,
   pub tx_hash:web3::types::H256,
}


async fn balance_test() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY")?).await?;
    let web3s = web3::Web3::new(websocket);
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS")?)?);
    info!("Accounts: {:?}", accounts);
    let wei_conv: U256 = U256::exp10(18);
    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        debug!(
            "Eth balance of {:?}: {}",
            account,
          //  balance.checked_div(wei_conv).unwrap()
          balance
        );
    }

    Ok(())

}

async fn contract_test() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY")?).await?;
    let web3s = web3::Web3::new(websocket);

    let aave_addr = Address::from_str("0x2D087e51eD54a348de214E0B39d85Be5976D7779")?;
    let token_contract =
        Contract::from_json(web3s.eth(), aave_addr, include_bytes!("contract_abi.json"))?;
    let token_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await?;
    let total_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await?;
    info!("Token name: {}, total supply: {}", token_name, total_supply);
    Ok(())

}

async fn get_latest_block() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY")?).await?;
    let web3s = web3::Web3::new(websocket);

    let latest_block = web3s
    .eth()
    .block(BlockId::Number(BlockNumber::Latest))
    .await
    .unwrap()
    .unwrap();
info!(
    "block number {}, number of transactions: {}, difficulty {}",
    latest_block.number.unwrap(),
    &latest_block.transactions.len(),
    &latest_block.total_difficulty.unwrap()
);

    Ok(())

}

async fn get_ropsten_latest_block() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_ROPSTEN")?).await?;
    let web3s = web3::Web3::new(websocket);

    let latest_block = web3s
    .eth()
    .block(BlockId::Number(BlockNumber::Latest))
    .await
    .unwrap()
    .unwrap();
info!(
    "block number {}, number of transactions: {}, difficulty {}",
    latest_block.number.unwrap(),
    &latest_block.transactions.len(),
    &latest_block.total_difficulty.unwrap()
);

    Ok(())

}


async fn balance_ropsten_test() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_ROPSTEN")?).await?;
    let web3s = web3::Web3::new(websocket);
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS")?)?);
    info!("Accounts: {:?}", accounts);
    let wei_conv: U256 = U256::exp10(18);
    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        debug!(
            "Eth balance of {:?}: {}",
            account,
          //  balance.checked_div(wei_conv).unwrap()
          balance
        );
    }

    Ok(())

}


pub async fn listen_event_erc20_to_mainbridge_rinkby() -> anyhow::Result<()> {

    use web3::{
        futures,
        contract::{Contract, Options},
        futures::StreamExt,
        types::FilterBuilder,
    };
    
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY")?).await?;
    let web3 = web3::Web3::new(websocket);

 // let transport = web3::transports::Http::new(&env::var("INFURA_RINKEBY_HTTP")?)?;
 // let web3 = web3::Web3::new(transport);


  let mut accounts = web3.eth().accounts().await?;
  debug!("Accounts: {:?}", accounts);


  //listen contract address
    let contract_address = Address::from_str(crate::MAIN_TOKEN_CONTRACT_ADDRESS)?;
    /*
    let contract = Contract::from_json(
        web3.eth(),
        hex!("d9145CCE52D386f254917e481eB44e9943F39138").into(),
        include_bytes!("../contracts/uniswap_v3_pool.json"),
    )?;
*/


    //listen event    get_log(address,address,uint256)  on   https://emn178.github.io/online-tools/keccak_256.html    sign

    //keccak("Transfer(address,address,uint256)")

    let filter = FilterBuilder::default()
    .address(vec![contract_address])
    .topics(
        Some(vec![hex!(
            "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        )
        .into()]),
        None,
        None,
        None,
    )
    .build();

let filter = web3.eth_filter().create_logs_filter(filter).await?;

let logs_stream = filter.stream(std::time::Duration::from_secs(1));
futures::pin_mut!(logs_stream);

/*
let tx = contract.call("hello", (), accounts[0], Options::default()).await?;
println!("got tx: {:?}", tx);
*/

while let Some(log) = logs_stream.next().await {
   
    let log=match log {
        Ok(l) => l,
        Err(e) => {
            error!("recode this error {:?}",e);
            continue;
        }
    };


   if let Ok(Some(po))=parse_transfer_event(log,&web3).await {
    tokio::spawn(process_tranfer_event(po));
   } 


}



    Ok(())
}


async fn parse_transfer_event(log:web3::types::Log,web3:&Web3<WebSocket>) -> anyhow::Result<Option<TransferPO>> {

    info!("got log: {:?}", log);
    let hash=log.transaction_hash.ok_or(anyhow::anyhow!("logs transaction_hash is none"))?;
    info!("the hash is {:?}",hash);
   
    use ethabi::{Event, EventParam, ParamType, Log, RawLog};
    
    
        let params = vec![EventParam {
            name: "from".to_string(),
            kind: ParamType::Address,
            indexed: true
        },EventParam {
            name: "to".to_string(),
            kind: ParamType::Address,
            indexed: true
        },EventParam {
            name: "value".to_string(),
            kind: ParamType::Uint(256),
            indexed: false
        }];
    
        let event = Event {
            name: "Transfer".to_string(),
            inputs: params,
            anonymous: false
        };
    
        let ev_hash = event.signature();

        info!("the ev_hash is {:?}",ev_hash);

        let web3_format_hash=web3::types::H256::from_slice(&ev_hash.0);
        info!("the web3_format_hash is {:?}",web3_format_hash);

        let receipt = web3.eth()
                .transaction_receipt(hash)
                .await?.unwrap();

                info!("the receipt is {:?}",receipt);

        let log_source = receipt.logs.iter().find(|log| {
            log.topics.iter().find(|topic| topic == &&web3_format_hash).is_some()
        });


        info!("the log ----- is {:?}",log_source);

        let log=log_source.ok_or(anyhow::anyhow!("before log parse is none"))?;

        //  have prameter index type 
        let event_hash=ethabi::ethereum_types::H256::from_slice(&log.topics[0].0);
        let index_one=ethabi::ethereum_types::H256::from_slice(&log.topics[1].0);
        let index_two=ethabi::ethereum_types::H256::from_slice(&log.topics[2].0);
    
       
        let res=match log_source {
            Some(l) => {
                Some(event.parse_log(RawLog {
                    topics: vec![event_hash,index_one,index_two],
                    data: l.data.clone().0
                })?)
            },
            None => None
        };
        
       let res_po=res.ok_or(anyhow::anyhow!("the parse po res is none"))?;
       let from=match res_po.params[0].value {
           ethabi::token::Token::Address(address) => address,
           _ => {anyhow::bail!("not a from address")}
       };

       let to=match res_po.params[1].value {
        ethabi::token::Token::Address(address) => address,
        _ => {anyhow::bail!("not a to address")}
    };


    let value=match res_po.params[2].value {
        ethabi::token::Token::Uint(amount) => amount,
        _ => {anyhow::bail!("not a to U256")}
    };

    //  attention !1!    to address must equal mainBridge address
     let main_bridge_address= Address::from_str(crate::MAIN_BRIDGE_CONTRACT_ADDRESS)?;
    if to!=ethabi::ethereum_types::H160::from_slice(&main_bridge_address.0) {
        // Nothing else 
        info!("receive  tansfer,but destination is not mainbridge ,should not handle");
        return Ok(None);
    }

       let po=TransferPO {
           from:from,
           to:to,
           value:value,
           tx_hash:hash,
       };

       Ok(Some(po))

}


async fn process_tranfer_event(po:TransferPO) -> anyhow::Result<()> {

  info!("the get po -----{:?}",po);

    //  mainbridge  lock
   info!("main bridging starting locking tokens");

   if let Err(e) =gateway_to_mainbridge_lock_token(&po).await {
       error!("gateway mainbridge lock token ---{:?}",e);
       anyhow::bail!("gateway_to_mainbridge_lock_token call error,don't continue");
   }


   //sidebridge  mint

   if let Err(e) = gateway_to_sidebridge_mint_token(&po).await {
    error!("gateway sidebridge miint token ---{:?}",e);
   }

    Ok(())

}


async fn gateway_to_mainbridge_lock_token(po:&TransferPO) -> anyhow::Result<()> {

    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY")?).await?;
    let web3 = web3::Web3::new(websocket);

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let contract_address = Address::from_str(crate::MAIN_BRIDGE_CONTRACT_ADDRESS)?;

    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)   
    let prvk = SecretKey::from_str(crate::PRIVATE_KEY)?;

 
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("mainbridge_abi.json"),
    )?;


    // encode call data
    let value:U256=U256(po.value.0);
    let fun_para= (web3::types::H160::from_slice(&po.from.0), value, value);

    let fn_data = contract
    .abi()
    .function("lockTokens")
    .and_then(|function| function.encode_input(&fun_para.into_tokens()))
   
    .map_err(|err| anyhow::anyhow!("the call_data is {:?}",err))?;

    let options= Options::default();
    
    /*Options::with(|options| {

        options.gas_price = Some(10_000_000.into());

    }),
*/
    // Build the tx object
    let tx_object = TransactionParameters {
        nonce: options.nonce,
        to: Some(contract.address()),
        gas_price: options.gas_price,
        data: Bytes(fn_data),
        transaction_type: options.transaction_type,
     
     
        ..Default::default()
    };

    //let data=

    // Sign the tx (can be done offline)
    let signed = web3
        .accounts()
        .sign_transaction(tx_object, &prvk)
        .await?;

    // Send the tx to server
    let result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;

    info!("lockTokens Tx succeeded with hash: {}", result);
    

    Ok(())


}

async fn gateway_to_sidebridge_mint_token(po:&TransferPO) -> anyhow::Result<()> {

    //sign 

    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_ROPSTEN")?).await?;
    let web3 = web3::Web3::new(websocket);

      // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let contract_address = Address::from_str(crate::SIDE_BRIDGE_CONTRACT_ADDRESS)?;

      // Insert the 32-byte private key in hex format (do NOT prefix with 0x)   
    let prvk = SecretKey::from_str(crate::PRIVATE_KEY)?;
  
   
    let contract = Contract::from_json(
          web3.eth(),
          contract_address,
          include_bytes!("sidebridge_abi.json"),
      )?;
  
      // 
  
      let value:U256=U256(po.value.0);
      let fun_para= (web3::types::H160::from_slice(&po.from.0), value, value);
  
      let fn_data = contract
      .abi()
      .function("bridgeTokens")
      .and_then(|function| function.encode_input(&fun_para.into_tokens()))
     
      .map_err(|err| anyhow::anyhow!("the side bridgeTOkens call_data is {:?}",err))?;
  
      let options= Options::default();
      
      /*Options::with(|options| {
  
          options.gas_price = Some(10_000_000.into());
  
      }),
  */
      // Build the tx object
      let tx_object = TransactionParameters {
        nonce: options.nonce,
        to: Some(contract.address()),
        gas_price: options.gas_price,
        data: Bytes(fn_data),
        transaction_type: options.transaction_type,       
        ..Default::default()
      };
    // Sign the tx (can be done offline)
    let signed = web3
        .accounts()
        .sign_transaction(tx_object, &prvk)
        .await?;

    // Send the tx to infura
    let result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;

    info!("bridgeTokens Tx succeeded with hash: {}", result);
    

    Ok(())



}

