use std::ops::Add;

use cosmwasm_std::{coin, BankMsg, StdError, SubMsg, WasmMsg};
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    DENOM.save(deps.storage, &String::from("cudo"))?;
    OWNER.set(deps, Some(info.sender))?;
    Ok(Response::default())
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
        ExecuteMsg::AcceptLease {
            rentee,
            property_id,
        } => execute_accept_lease(deps, env, info, rentee, property_id),
        ExecuteMsg::RequestForLease { property_id } => {
            execute_request_lease(deps, env, info, property_id)
        }
        ExecuteMsg::TerminateLease { property_id } => {
            execute_terminate_lease(deps, env, info, property_id)
        }
        ExecuteMsg::PayRent { property_id } => execute_pay_rent(deps, env, info, property_id),
    }
}

fn execute_pay_rent(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: usize,
) -> Result<Response, ContractError> {
    let empty_vec: Vec<FlatInfo> = Vec::new();
    let denom = DENOM.load(deps.storage)?;

    let mut flat_list = FLAT_LIST.may_load(deps.storage)?.unwrap_or(empty_vec);
    let d = info
        .funds
        .iter()
        .find(|x| x.denom == denom)
        .ok_or_else(|| ContractError::InvalidDenom {})?;
    if id > flat_list.len() {
        return Err(ContractError::NotFound {});
    }
    let mut m = flat_list[id].clone();
    if d.amount < m.rent {
        return Err(ContractError::LessThanRent {});
    }
    if !m.is_rented {
        return Err(ContractError::IsNotRented {});
    }
    if let Some(rentee) = m.rentee.clone() {
        if rentee != info.sender.to_string() {
            return Err(ContractError::InvalidRentee {});
        }
    }
    match m.expires {
        Some(h) => {
            if h.is_expired(&env.block) {
                return Err(ContractError::Expired {})?;
            } else {
                m.expires = Some(h.add(Duration::Height(411428))?);
            }
        }
        None => Err(ContractError::Unauthorized {})?,
    }

    flat_list[id] = m;
    let mut rs = Response::default();
    let mut message: Vec<SubMsg> = vec![];
    message.push(SubMsg::new(BankMsg::Send {
        amount: vec![coin(flat_list[id].rent.u128(), denom.clone())],
        to_address: flat_list[id].renter.to_string(),
    }));
    if d.amount > flat_list[id].rent {
        let change = d
            .amount
            .checked_sub(flat_list[id].rent.clone())
            .map_err(StdError::overflow)?;
        message.push(SubMsg::new(BankMsg::Send {
            amount: vec![coin(change.u128(), denom)],
            to_address: info.sender.to_string(),
        }));
    }
    FLAT_LIST.save(deps.storage, &flat_list)?;

    rs.messages = message;

    Ok(rs)
}

fn execute_accept_lease(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    rentee: String,
    id: usize,
) -> Result<Response, ContractError> {
    let denom = DENOM.load(deps.storage)?;
    let empty_vec: Vec<FlatInfo> = Vec::new();
    let mut flat_list = FLAT_LIST.may_load(deps.storage)?.unwrap_or(empty_vec);
    if id > flat_list.len() {
        return Err(ContractError::NotFound {});
    }
    let mut m = flat_list[id].clone();
    if m.renter != info.sender.to_string() {
        return Err(ContractError::InvalidRenter {});
    }
    if m.is_rented {
        return Err(ContractError::IsRented {});
    }
    match m.rentee {
        Some(_) => m.is_rented = true,
        None => Err(ContractError::InvalidRentee {})?,
    }
    m.expires = Some(Expiration::AtHeight(env.block.height + 411428));
    flat_list[id] = m;
    FLAT_LIST.save(deps.storage, &flat_list)?;
    let mut rs = Response::default();
    rs.messages = vec![SubMsg::new(BankMsg::Send {
        amount: vec![coin(flat_list[id].rent.u128(), denom)],
        to_address: flat_list[id].renter.to_string(),
    })];
    Ok(rs)
}

fn execute_add_property(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rent: Uint128,
) -> Result<Response, ContractError> {
    let flat = FlatInfo {
        is_rented: false,
        rent,
        rentee: None,
        renter: info.sender.to_string(),
        expires: None,
    };
    let empty_vec: Vec<FlatInfo> = Vec::new();
    let mut list = FLAT_LIST.may_load(deps.storage)?.unwrap_or(empty_vec);
    list.push(flat);
    RENTER_TO_FLAT_ID.update(
        deps.storage,
        &info.sender,
        |owned| -> StdResult<Vec<usize>> {
            match owned {
                Some(mut address) => {
                    address.push(list.len());
                    Ok(address)
                }
                None => Ok(vec![list.len()]),
            }
        },
    )?;
    FLAT_LIST.save(deps.storage, &list)?;

    Ok(Response::default())
}

fn execute_request_lease(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: usize,
) -> Result<Response, ContractError> {
    let empty_vec: Vec<FlatInfo> = Vec::new();
    let denom = DENOM.load(deps.storage)?;

    let mut flat_list = FLAT_LIST.may_load(deps.storage)?.unwrap_or(empty_vec);
    let d = info
        .funds
        .iter()
        .find(|x| x.denom == denom)
        .ok_or_else(|| ContractError::InvalidDenom {})?;
    if id > flat_list.len() {
        return Err(ContractError::NotFound {});
    }
    let mut m = flat_list[id].clone();
    if d.amount < m.rent {
        return Err(ContractError::LessThanRent {});
    }
    m.rentee = Some(info.sender.to_string());
    flat_list[id] = m;

    FLAT_LIST.save(deps.storage, &flat_list)?;
    let mut rs = Response::default();
    if d.amount > flat_list[id].rent {
        let change = d
            .amount
            .checked_sub(flat_list[id].rent.clone())
            .map_err(StdError::overflow)?;

        rs.messages = vec![SubMsg::new(BankMsg::Send {
            amount: vec![coin(change.u128(), denom)],
            to_address: info.sender.to_string(),
        })];
    }
    Ok(rs)
}

fn execute_terminate_lease(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: usize,
) -> Result<Response, ContractError> {
    let empty_vec: Vec<FlatInfo> = Vec::new();
    let mut flat_list = FLAT_LIST.may_load(deps.storage)?.unwrap_or(empty_vec);
    if id > flat_list.len() {
        return Err(ContractError::NotFound {});
    }
    let mut m = flat_list[id].clone();
    if m.renter != info.sender.to_string() {
        return Err(ContractError::InvalidRenter {});
    }
    if !m.is_rented {
        return Err(ContractError::IsNotRented {});
    }
    m.is_rented = false;
    m.rentee = None;
    flat_list[id] = m;
    FLAT_LIST.save(deps.storage, &flat_list)?;
    Ok(Response::default())
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

pub fn query_property_info(deps: Deps, id: usize) -> StdResult<FlatInfo> {
    let flat = FLAT_LIST.load(deps.storage).unwrap();
    Ok(flat[id].clone())
}

pub fn query_show_all_available_properties(deps: Deps) -> StdResult<Vec<usize>> {
    let flat_list = FLAT_LIST.load(deps.storage)?;
    let mut list: Vec<usize> = vec![];
    for (i, flat) in flat_list.iter().enumerate() {
        if !flat.is_rented {
            list.push(i);
        }
    }
    Ok(list)
}

pub fn query_get_total_property(deps: Deps) -> StdResult<usize> {
    let flat_list = FLAT_LIST.load(deps.storage)?;
    Ok(flat_list.len())
}
