use ethers::prelude::*;
use ethers::abi::parse_abi;
use std::str::FromStr;
use std::sync::Arc;
use crate::forked_db::fork_factory::ForkFactory;
use crate::forked_db::{to_revm_address, to_revm_u256};
use revm::primitives::{TransactTo, ExecutionResult, Output};
use revm::db::{CacheDB, EmptyDB};
use revm::EVM;


async fn example() {


    // setup client
    let url: &str = "ws://localhost:8546";
    let provider = Provider::<Ws>::connect(url).await.unwrap();
    let client = Arc::new(provider);

    let latest_block = client.get_block_number().await.unwrap();

    // convert to BlockId
    let latest_block_id = Some(
        BlockId::Number(BlockNumber::Number(latest_block))
    );

    // create an empty cache db
    let cache_db = CacheDB::new(EmptyDB::default());

    // setup backend
    let fork_factory = ForkFactory::new_sandbox_factory(client, cache_db, latest_block_id);
    let fork_db = fork_factory.new_sandbox_fork();

    // get the balance of weth from an address

    let owner_address = Address::from_str("0xyouraddress").unwrap();
    let weth_address = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();

    // create an evm instance
    let mut evm = EVM::new();
    evm.database(fork_db);

    let erc20 = BaseContract::from(
        parse_abi(&["function balanceOf(address) external returns (uint)"]).unwrap()
    );

    let ethers_bytes: ethers::core::types::Bytes = erc20
    .encode("balanceOf", owner_address)
    .unwrap();

    // setup evm fields

    // convert ethers to primitive types
    evm.env.tx.caller = to_revm_address(owner_address);

    evm.env.tx.transact_to = TransactTo::Call(weth_address.0.into());
    evm.env.tx.data = revm::primitives::Bytes::from(ethers_bytes.0);
    evm.env.tx.gas_price = to_revm_u256(U256::from(3000000000u64));
    evm.env.tx.gas_limit = 1000000;
    evm.env.tx.value = to_revm_u256(U256::zero());

    let result = evm.transact_ref().unwrap().result;

        // ** get the output
        let output: revm::primitives::Bytes = match result {
            ExecutionResult::Success { output, .. } => {
                match &output {
                    Output::Call(o) => { o.clone().into() }
                    Output::Create(o, _) => { o.clone().into() }
                }
            }
            ExecutionResult::Revert { .. } => {
                panic!("Failed to get balance")
            }
            ExecutionResult::Halt { .. } => {
                panic!("Failed to get balance")
            }
        };

        let balance: U256 = erc20.decode_output("balanceOf", &output).unwrap_or_default();
        println!("Balance: {}", balance);

}