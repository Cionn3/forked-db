pub mod database_error;
pub use database_error::DatabaseError;

pub mod global_backend;
pub use global_backend::*;

pub mod fork_db;
pub mod fork_factory;



// ** Convert Types from Ethers To Revm Primitive Types **

// Converts from Ethers U256 to revm::primitives::U256
pub fn to_revm_u256(u: ethers::types::U256) -> revm::primitives::U256 {
    let mut bytes = [0u8; 32];
    u.to_little_endian(&mut bytes);
    revm::primitives::U256::from_le_bytes(bytes)
}

// Converts from revm primitive U256 to ethers U256
pub fn to_ethers_u256(u: revm::primitives::U256) -> ethers::types::U256 {
    let bytes: [u8; 32] = u.to_be_bytes();
    ethers::types::U256::from_big_endian(&bytes)
}

// converts from revm primitive Address to ethers Address
pub fn to_ethers_address(address: revm::primitives::Address) -> ethers::types::Address {
    let bytes: [u8; 20] = address.0.into();
    let addr = ethers::types::H160::from(bytes);
    addr
}

// converts from Ethers Address to revm primitive Address
pub fn to_revm_address(address: ethers::types::Address) -> revm::primitives::Address {
    let bytes: [u8; 20] = address.0.into();
    let addr = revm::primitives::Address::from(bytes);
    addr
}