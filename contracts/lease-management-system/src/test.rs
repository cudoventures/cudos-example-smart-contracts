#![cfg(test)]
use crate::{
    contract::{execute, instantiate, query_property_info},
    msg::{ExecuteMsg, InstantiateMsg},
    state::FlatInfo,
};
use cosmwasm_std::{Coin, CosmosMsg, DepsMut, Empty, Response, WasmMsg};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Uint128,
};

fn do_instantiate(deps: DepsMut, owner: &String) {
    let msg = InstantiateMsg {};
    let info = mock_info(owner, &[]);
    instantiate(deps, mock_env(), info, msg).unwrap();
}

#[test]
fn add_renter() {
    let mut deps = mock_dependencies(&[]);
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);

    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let d = query_property_info(deps.as_ref(), 0).unwrap();
    assert_eq!(
        d,
        FlatInfo {
            renter: renter.to_string(),
            rent: Uint128::new(200),
            is_rented: false,
            rentee: None,
            expires: None
        }
    );
}
#[test]
fn request_lease() {
    let mut deps = mock_dependencies(&[]);
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let msg = ExecuteMsg::RequestForLease { property_id: 0 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(300),
            denom: String::from("cudo"),
        }],
    );
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    query_property_info(deps.as_ref(), 0).unwrap();
}
#[test]
fn accept_lease() {
    let mut deps = mock_dependencies(&[]);
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let msg = ExecuteMsg::RequestForLease { property_id: 0 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(300),
            denom: String::from("cudo"),
        }],
    );
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    query_property_info(deps.as_ref(), 0).unwrap();
    let msg = ExecuteMsg::AcceptLease {
        property_id: 0,
        rentee: rentee.to_string(),
    };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let d = query_property_info(deps.as_ref(), 0).unwrap();
}
#[test]
fn terminate_lease() {
    let mut deps = mock_dependencies(&[]);
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let msg = ExecuteMsg::RequestForLease { property_id: 0 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(300),
            denom: String::from("cudo"),
        }],
    );
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    query_property_info(deps.as_ref(), 0).unwrap();
    let msg = ExecuteMsg::AcceptLease {
        property_id: 0,
        rentee: rentee.to_string(),
    };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let msg = ExecuteMsg::TerminateLease { property_id: 0 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let d = query_property_info(deps.as_ref(), 0).unwrap();

    print!("{:?}", d);
}

#[test]
fn pay_rent() {
    let mut deps = mock_dependencies(&[]);
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let msg = ExecuteMsg::RequestForLease { property_id: 0 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(300),
            denom: String::from("cudo"),
        }],
    );
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    query_property_info(deps.as_ref(), 0).unwrap();
    let msg = ExecuteMsg::AcceptLease {
        property_id: 0,
        rentee: rentee.to_string(),
    };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let d = query_property_info(deps.as_ref(), 0).unwrap();
    println!("{:?}", d);

    let msg = ExecuteMsg::PayRent { property_id: 0 };
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(200),
            denom: String::from("cudo"),
        }],
    );
    let mut env1 = mock_env();
    env1.block.height = env.block.height + 411426;
    execute(deps.as_mut(), env1.clone(), info, msg).unwrap();
    let d = query_property_info(deps.as_ref(), 0).unwrap();
}
