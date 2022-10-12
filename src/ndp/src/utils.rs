use candid::{Nat, Principal};

use crate::dip20;

const NDP_TOKEN_CANISTER_ID_TEXT: &str = "vgqnj-miaaa-aaaal-qaapa-cai";
const NDP_STAKING_CANISTER_ID_TEXT: &str = "eijhz-biaaa-aaaah-abs6q-cai";

async fn check_ndp_balance(caller: Principal, amount: u64) -> Result<Nat, String> {
    let dip_client = dip20::Service::new(Principal::from_text(NDP_TOKEN_CANISTER_ID_TEXT).unwrap());
    let balance = dip_client
        .balanceOf(caller)
        .await
        .map_err(|(code, string)| string)?;

    let amount = candid::Nat::from(amount);
    if balance.0 < amount {
        return Err("Insufficient balance!".to_string());
    }
    Ok(balance.0)
}

pub async fn transfer_ndp(caller: Principal, amount: u64) -> Result<Nat, String> {
    check_ndp_balance(caller, amount).await?;
    let ndp_staking_principal = Principal::from_text(NDP_STAKING_CANISTER_ID_TEXT).unwrap();
    let dip_client = dip20::Service::new(Principal::from_text(NDP_TOKEN_CANISTER_ID_TEXT).unwrap());
    dip_client
        .transferFrom(caller, ndp_staking_principal, candid::Nat::from(amount))
        .await
        .map_or_else(
            |(code, str)| Err(str),
            |result| match result {
                (dip20::Result::Ok(res),) => Ok(res),
                (dip20::Result::Err(err),) => Err("transfer failed!".to_owned()),
            },
        )
}
