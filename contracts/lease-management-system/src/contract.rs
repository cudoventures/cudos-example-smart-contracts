use std::ops::Add;

use cosmwasm_std::{coin, BankMsg, Coin, StdError, SubMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw0::{Duration, Expiration};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{FlatInfo, DENOM, FLAT_LIST, OWNER, RENTER_TO_FLAT_ID},
    ContractError,
};
// instantiate the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set default denom
    // Set info.sender as the owner of the contract
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddProperty { rent } => todo!(),
        ExecuteMsg::AcceptLease { property_id } => {
            execute_accept_lease(deps, env, info, property_id)
        }
        ExecuteMsg::RequestForLease { property_id } => {
            execute_request_lease(deps, env, info, property_id)
        }
        ExecuteMsg::TerminateLease { property_id } => {
            execute_terminate_lease(deps, env, info, property_id)
        }
        ExecuteMsg::PayRent { property_id } => execute_pay_rent(deps, env, info, property_id),
        ExecuteMsg::RejectLease { property_id } => {
            execute_reject_lease(deps, env, info, property_id)
        }
    }
}
// function is called when rentee wants to pay rent
fn execute_pay_rent(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: usize,
) -> Result<Response, ContractError> {
    // It can only be done after renter accepted the rentee.
    // Can only be called by the rentee of the flat within completion of month.
    // If rentee pay the rent after 1 month then it is expired.
    // If rentee paid rent twice in the month then rentee agrement is valid for two months.
    // If amount provided by  rentee is more than one month rent then refund the excess rent to the rentee.
}

fn execute_reject_lease(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: usize,
) -> Result<Response, ContractError> {
    // It is used to reject the rentee and release the amount locked by the rentee for a given property.
    // Can be called only by Renter of the property
    // The rent of the first month+security locked inside the contract is released to the Rentee
    // Also update the expiration date with None.
    // Update rentee with None.
}

fn execute_accept_lease(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: usize,
) -> Result<Response, ContractError> {
    // Can be called only by Renter of the property
    // The rent of the first month locked inside the contract is released to the Renter
    // Also update the expiration date with one month.
}

fn execute_add_property(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rent: Uint128,
) -> Result<Response, ContractError> {
    // It is used to list the property for rent. The caller of this function will be the renter of that property.
    // If the renter is listing property first time, will register as a renter else update list with newly listed properties
    // Rent amount must be in the native currency of the chain ie.. cudos in this case.
    // Property is assigned with propertyid
    // PropertyId is auto-incremental id ie... if the contract has 100 properties (listed by different renter) then id would start from 1 to 100 and the next property id will be 101.
}

fn execute_request_lease(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: usize,
) -> Result<Response, ContractError> {
    // The caller of this function is Rentee who wants to rent a property and will pay rent + security in desired denomination mentioned in the contract ie.. native currency.
    // Locks rent of the first month with a security deposit which is equivalent to one month rent to the contract ie.. rentee needs to lock 2x amount of rent.
    // This rent of the first month + security is released when the Renter of the property accepts the rent.
    // If amount provided by rentee is more than one month rent + security then refund the excess rent to the rentee.
}

fn execute_terminate_lease(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: usize,
) -> Result<Response, ContractError> {
    // Can be called by Renter of the property and is used to terminate the lease only if Rentee defaults on any month rent.
    // Release the security deposit to rentee.
    // Update the expiration date with None
    // Remove the rentee with that property id.
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _info: MessageInfo, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PropertyDetail(id) => to_binary(&query_property_info(deps, id)?),
        QueryMsg::ShowAllAvailableProperties => {
            to_binary(&query_show_all_available_properties(deps)?)
        }
        QueryMsg::GetTotalProperties => to_binary(&query_get_total_property(deps)?),
    }
}

pub fn query_property_info(deps: Deps, id: usize) -> StdResult<FlatInfo> {}

pub fn query_show_all_available_properties(deps: Deps) -> StdResult<Vec<usize>> {}

pub fn query_get_total_property(deps: Deps) -> StdResult<usize> {}
