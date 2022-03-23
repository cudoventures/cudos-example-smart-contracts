use std::ops::{Add, Sub};

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128,
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
    let denom = String::from("acudos");
    DENOM.save(deps.storage, &denom)?;
    OWNER.save(deps.storage, &info.sender)?;
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

    let mut cfg_flatlist = FLAT_LIST.may_load(deps.storage)?.unwrap();

    if id > cfg_flatlist.len() - 1 {
        return Err(ContractError::NotFound {});
    }

    let my_property = cfg_flatlist[id].clone();

    if my_property.expires == None {
        return Err(ContractError::ExpirationDoesNotExist {});
    };

    if my_property.expires.unwrap().is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    let rentee_cudo_index = info
        .funds
        .iter()
        .position(|x| x.denom == String::from("acudos"))
        .unwrap();

    let rentee_cudo = info.funds[rentee_cudo_index].amount;

    if rentee_cudo < my_property.rent {
        return Err(ContractError::LessThanRent {});
    };
    let new_expiry = my_property
        .expires
        .unwrap()
        .add(Duration::Height(411428u64))?;
    cfg_flatlist[id].expires = Some(new_expiry);

    let double_rent = my_property.rent + my_property.rent;

    if rentee_cudo != double_rent && rentee_cudo > my_property.rent {
        let excess_rent = rentee_cudo - my_property.rent;
        BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                amount: excess_rent,
                denom: String::from("acudos"),
            }],
        };
    }

    if rentee_cudo == double_rent {
        let new_expiry = my_property
            .expires
            .unwrap()
            .add(Duration::Height(411428u64 * 2))?;
        cfg_flatlist[id].expires = Some(new_expiry);
    };

    FLAT_LIST.save(deps.storage, &cfg_flatlist)?;

    Ok(Response::default())
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

    let mut cfg_flatlist = FLAT_LIST.may_load(deps.storage)?.unwrap();

    if cfg_flatlist[id].rentee == None {
        return Err(ContractError::IsNotRented {});
    };

    if id > cfg_flatlist.len() - 1 {
        return Err(ContractError::NotFound {});
    };

    if cfg_flatlist[id].renter != info.sender.to_string() {
        return Err(ContractError::InvalidRenter {});
    };

    if cfg_flatlist[id].rentee != None && cfg_flatlist[id].expires != None {
        return Err(ContractError::IsAcceptedByRenter {});
    };

    let rentee_deposite = cfg_flatlist[id].rent + cfg_flatlist[id].rent;

    let codocrypto = Coin {
        amount: rentee_deposite,
        denom: String::from("acudos"),
    };

    BankMsg::Send {
        to_address: cfg_flatlist[id].rentee.as_ref().unwrap().to_string(),
        amount: vec![codocrypto],
    };

    cfg_flatlist[id].expires = None;
    cfg_flatlist[id].rentee = None;

    FLAT_LIST.save(deps.storage, &cfg_flatlist)?;

    Ok(Response::default())
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

    let mut cfg_flatlist = FLAT_LIST.may_load(deps.storage)?.unwrap();

    if id > cfg_flatlist.len() - 1 {
        return Err(ContractError::NotFound {});
    };

    if cfg_flatlist[id].renter != info.sender.to_string() {
        return Err(ContractError::InvalidRenter {});
    };

    if cfg_flatlist[id].rentee == None {
        return Err(ContractError::IsNotRented {});
    };

    let t = env.block.height + 411428;
    cfg_flatlist[id].expires = Some(Expiration::AtHeight(t));

    let rentee_deposite = cfg_flatlist[id].rent + cfg_flatlist[id].rent;

    let codocrypto = Coin {
        amount: rentee_deposite,
        denom: String::from("acudos"),
    };

    BankMsg::Send {
        to_address: cfg_flatlist[id].rentee.as_ref().unwrap().to_string(),
        amount: vec![codocrypto],
    };

    FLAT_LIST.save(deps.storage, &cfg_flatlist)?;

    Ok(Response::default())
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

    let data = FlatInfo {
        renter: info.sender.clone().to_string(),
        rentee: None,
        rent: rent,
        expires: None,
    };

    let mut cfg_flatlist = FLAT_LIST.may_load(deps.storage)?;

    if cfg_flatlist == None {
        cfg_flatlist = Some(vec![data.clone()]);
        FLAT_LIST.save(deps.storage, &cfg_flatlist.clone().unwrap())?;
    } else {
        FLAT_LIST.update(deps.storage, |mut val| -> StdResult<Vec<FlatInfo>> {
            val.push(data);
            Ok(val)
        })?;
    };

    let mut l = FLAT_LIST.load(deps.storage)?.len();
    if l.gt(&usize::from(0u16)) {
        l = l.sub(usize::from(1u16));
    }
    RENTER_TO_FLAT_ID.update(deps.storage, &info.sender, |val| -> StdResult<Vec<usize>> {
        match val {
            Some(mut h) => {
                h.push(l);
                Ok(h)
            }
            None => {
                Ok(vec![l])
            }
        }
    })?;

    Ok(Response::default())
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

    let mut cfg_flatlist = FLAT_LIST.may_load(deps.storage)?.unwrap();
    let caller_is_renter = cfg_flatlist
        .iter()
        .any(|a| a.renter.as_ref() == info.sender.to_string());

    if caller_is_renter {
        return Err(ContractError::InvalidRentee {});
    }

    if id > cfg_flatlist.len() - 1 {
        return Err(ContractError::NotFound {});
    }

    if cfg_flatlist[id].rentee != None {
        return Err(ContractError::RenteeExist {});
    }

    let denom_check = info
        .funds
        .iter()
        .any(|x| x.denom == DENOM.load(deps.storage).unwrap());

    if !denom_check {
        return Err(ContractError::InvalidDenom {});
    }

    let rent = cfg_flatlist[id].rent;
    let amount_to_pay = rent + rent;

    if info
        .funds
        .iter()
        .any(|x| x.amount >= amount_to_pay && x.denom == DENOM.load(deps.storage).unwrap())
        == false
    {
        return Err(ContractError::LessThanRent {});
    };

    cfg_flatlist[id].rentee = Some(info.sender.to_string());

    FLAT_LIST.save(deps.storage, &cfg_flatlist)?;

    Ok(Response::new()
        .add_attribute("action", "refund")
        .add_attribute("id", id.to_string()))
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
    let mut cfg_flatlist = FLAT_LIST.may_load(deps.storage)?.unwrap();

    if id > cfg_flatlist.len() - 1 {
        return Err(ContractError::NotFound {});
    };

    if cfg_flatlist[id].renter != info.sender.to_string() {
        return Err(ContractError::InvalidRenter {});
    };

    if cfg_flatlist[id].expires == None {
        return Err(ContractError::IsNotRented {});
    };

    let exp = cfg_flatlist[id].expires.unwrap().is_expired(&env.block);
    if !exp {
        return Err(ContractError::NotExpired {});
    }

    let codocrypto = Coin {
        amount: cfg_flatlist[id].rent,
        denom: String::from("acudos"),
    };

    BankMsg::Send {
        to_address: cfg_flatlist[id].rentee.as_ref().unwrap().to_string(),
        amount: vec![codocrypto],
    };

    cfg_flatlist[id].expires = None;
    cfg_flatlist[id].rentee = None;

    FLAT_LIST.save(deps.storage, &cfg_flatlist)?;
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
    let flat_list = FLAT_LIST.may_load(deps.storage)?.unwrap();
    if id > flat_list.len() {
        return Err(StdError::NotFound {
            kind: String::from("property not found"),
        });
    }
    Ok(flat_list[id].clone())
}

pub fn query_show_all_available_properties(deps: Deps) -> StdResult<Vec<usize>> {
    let mut flat_list = vec![];
    let flat_list2 = FLAT_LIST.may_load(deps.storage)?.unwrap();
    flat_list.push(flat_list2.len());
    Ok(flat_list)
}

pub fn query_get_total_property(deps: Deps) -> StdResult<usize> {
    let flat_list2 = FLAT_LIST.may_load(deps.storage)?.unwrap();
    Ok(flat_list2.len())
}
