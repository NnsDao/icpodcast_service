use ic_cdk_macros::init;
use crate::OWNER_DATA_STATE;
use ic_cdk::export::candid::Principal;


#[init]
// fn init(admin: Principal) {
fn init() {
    ic_cdk::setup();
    let admin = ic_cdk::caller();
    OWNER_DATA_STATE.with(|owner_service| owner_service.borrow_mut().add_owner(admin));
    OWNER_DATA_STATE.with(|owner_service| owner_service.borrow_mut().init_admin(admin));
}
