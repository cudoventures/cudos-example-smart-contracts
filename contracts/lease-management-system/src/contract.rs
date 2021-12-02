use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::FlatInfo,
    ContractError,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddProperty { rent } => execute_add_property(deps, env, info, rent),
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

fn execute_pay_rent(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: Uint128,
) -> Result<Response, ContractError> {
    // It can only be done after the Renter accepted the Rentee.
    // Can only be called by the Rentee of the flat within the completion of the month.
    // If the Rentee pays the rent after 1 month then it is expired.
    // If the Rentee paid rent twice in the month then the Rentee agreement is valid for two months.
    // If the amount provided by the Rentee is more than one month’s rent then refund the excess rent to the Rentee.
}

fn execute_reject_lease(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: Uint128,
) -> Result<Response, ContractError> {
    // It is used to reject the Rentee and release the amount locked by the Rentee for a given property.
    // Can be called only by Renter of the property
    // The rent of the first month+security locked inside the contract is released to the Rentee
    // Also, update the expiration date with None.
    // Update Rentee with None.
}

fn execute_accept_lease(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: Uint128,
) -> Result<Response, ContractError> {
    // Can be called only by Renter of the property
    // The rent of the first month locked inside the contract is released to the Renter
    // Also, update the expiration date by one month.
}

fn execute_add_property(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rent: Uint128,
) -> Result<Response, ContractError> {
    // It is used to list the property for rent. The caller of this function will be the Renter of that property.
    // If the Renter is listing property first time, will register as a Renter else update the list with newly listed properties
    // The rent amount must be in the native currency of the chain ie.. cudos in this case.
    // Property is assigned with propertyid
    // PropertyId is auto-incremental id
}

fn execute_request_lease(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: Uint128,
) -> Result<Response, ContractError> {
    // The caller of this function is Rentee who wants to rent a property and will pay rent + security in desired denomination mentioned in the contract ie.. native currency.
    // Locks rent of the first month with a security deposit which is equivalent to one month rent to the contract ie... Rentee needs to lock 2x amount of rent.
    // This rent of the first month + security is released when the Renter of the property accepts the rent.
    // If the amount provided by the Rentee is more than one month’s rent + security then refund the excess rent to the Rentee.
}

fn execute_terminate_lease(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: Uint128,
) -> Result<Response, ContractError> {
    // can be called by the Renter of the property and is used to terminate the lease only if Rentee defaults on any month’s rent.
    // Release the security deposit to Rentee.
    // Update the expiration date with **None**
    // Remove the Rentee with that property id.
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PropertyDetail { id } => to_binary(&query_property_info(deps, id)?),
        QueryMsg::ShowAllAvailableProperties {} => {
            to_binary(&query_show_all_available_properties(deps)?)
        }
        QueryMsg::GetTotalProperties {} => to_binary(&query_get_total_property(deps)?),
    }
}

pub fn query_property_info(deps: Deps, id: Uint128) -> StdResult<FlatInfo> {
    // It is to view Renter, Rentee, and rent.
}

pub fn query_show_all_available_properties(deps: Deps) -> StdResult<Vec<Uint128>> {
    // It is used to view unrented properties
}

pub fn query_get_total_property(deps: Deps) -> StdResult<Uint128> {
    // It is used to view total number of properties.
}
