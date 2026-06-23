use soroban_sdk::{Env, Address, BytesN};

fn is_zero_address(env: &Env, address: &Address) -> bool {
    let xdr = address.to_xdr(env);
    // check if it's somehow "empty" or "zero"
    false
}
