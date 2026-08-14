//! Lab 02 — create wallets and receiving addresses.

use crate::rpc::{parse_cli_value, RpcClient};
use crate::LabResult;

/// Create a wallet with the supplied name.
pub fn create_wallet<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<()> {
    // TODO: call createwallet with the wallet name.
    // todo!("Lab 02: create a wallet")
    client.call(None, "createwallet", &[wallet_name.to_owned()])?;
    Ok(())
}

/// Return every wallet currently loaded by this node.
pub fn list_wallets<C: RpcClient>(client: &C) -> LabResult<Vec<String>> {
    // TODO: call listwallets and decode its JSON string array.
    // todo!("Lab 02: list loaded wallets")
    let raw = client.call(None, "listwallets", &[])?;
    Ok(serde_json::from_str(&raw)?)
}

/// Generate a labelled address in the selected wallet.
pub fn get_new_address<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    label: &str,
) -> LabResult<String> {
    // TODO: use wallet context and call getnewaddress with the supplied label.
    // todo!("Lab 02: generate a wallet address")
    let raw = client.call(Some(wallet_name), "getnewaddress", &[label.to_owned()])?;
    let response = parse_cli_value(&raw)?;

    response
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| crate::LabError::Parse("Expected an address in type String".to_owned()))
}

/// Ask the selected wallet whether it controls the supplied address.
pub fn address_belongs_to_wallet<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    address: &str,
) -> LabResult<bool> {
    // TODO: call getaddressinfo and return the `ismine` field.
    // todo!("Lab 02: verify address ownership")
    let raw = client.call(Some(wallet_name), "getaddressinfo", &[address.to_owned()])?;
    let response = parse_cli_value(&raw)?;

    response
        .get("ismine")
        .and_then(serde_json::Value::as_bool)
        .ok_or(crate::LabError::MissingField("ismine"))
}
