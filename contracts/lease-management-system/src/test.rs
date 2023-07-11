#![cfg(test)]
use crate::{
    contract::{
        execute, instantiate, query_get_total_property, query_property_info,
        query_show_all_available_properties,
    },
    msg::{ExecuteMsg, InstantiateMsg},
    state::FlatInfo,
    ContractError,
};
use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_env, mock_info},
    StdError, Uint128,
};
use cosmwasm_std::{Coin, DepsMut};
use cw_utils::Expiration;

fn do_instantiate(deps: DepsMut, owner: &String) {
    let msg = InstantiateMsg {};
    let info = mock_info(owner, &[]);
    instantiate(deps, mock_env(), info, msg).unwrap();
}

#[test]
fn add_property() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);

    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    println!("working");
    let d = query_property_info(deps.as_ref(), 1u16).unwrap();
    assert_eq!(
        d,
        FlatInfo {
            renter: renter.to_string(),
            rent: Uint128::new(200),
            rentee: None,
            expires: None
        }
    );

    // test auto increment of id
    let renter = String::from("new-renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(300),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let d = query_property_info(deps.as_ref(), 2u16).unwrap();
    assert_eq!(
        d,
        FlatInfo {
            renter: renter.to_string(),
            rent: Uint128::new(300),
            rentee: None,
            expires: None
        }
    );
}
#[test]
fn request_lease() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // appropriate denom is not given
    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(400),
            denom: String::from("kudo"),
        }],
    );
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::InvalidDenom {});

    // id not present
    let msg = ExecuteMsg::RequestForLease { property_id: 2u16 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(400),
            denom: String::from("acudos"),
        }],
    );
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert!(matches!(err, ContractError::NotFound {}));

    // Less than Rent error
    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(300),
            denom: String::from("acudos"),
        }],
    );
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert!(matches!(err, ContractError::LessThanRent {}));

    // Success response
    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(600),
            denom: String::from("acudos"),
        }],
    );
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // once requested cannot request again
    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("new-rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(400),
            denom: String::from("acudos"),
        }],
    );
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::RenteeExist {});
}

#[test]
fn accept_lease() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let property_id = 1u16;
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // if rentee is not present
    let msg = ExecuteMsg::AcceptLease { property_id };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::IsNotRented {});

    // request for lease
    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(rentee.as_str(), &coins(600u128, String::from("acudos")));
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::AcceptLease { property_id };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let q = query_property_info(deps.as_ref(), property_id).unwrap();
    assert_eq!(
        q,
        FlatInfo {
            expires: Some(Expiration::AtHeight(env.block.height + 411428)),
            rent: Uint128::new(200),
            rentee: Some(rentee.to_string()),
            renter: renter.to_string()
        }
    );

    // invalid renter
    let msg = ExecuteMsg::AcceptLease { property_id };
    let renter = String::from("new-renter");
    let info = mock_info(renter.as_str(), &[]);
    let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::InvalidRenter {});
}

#[test]
fn terminate_lease() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(600),
            denom: String::from("acudos"),
        }],
    );
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // error if rentee is not accepted by renter then renter cannot terminate the lease.
    let msg = ExecuteMsg::TerminateLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    assert_eq!(err, ContractError::IsNotRented {});

    let msg = ExecuteMsg::AcceptLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // renter cannot terminate the lease if agreement is not expired
    let msg = ExecuteMsg::TerminateLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let mut env1 = mock_env();
    env1.block.height = env.block.height + 411420u64;
    let err = execute(deps.as_mut(), env1, info, msg).unwrap_err();
    assert_eq!(err, ContractError::NotExpired {});

    // Success terminate, only possible if rentee lease is expired.
    let msg = ExecuteMsg::TerminateLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let mut env1 = mock_env();
    env1.block.height = env.block.height + 411430u64;
    execute(deps.as_mut(), env1, info, msg).unwrap();
}

#[test]
fn pay_rent() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(
        rentee.as_str(),
        &[Coin {
            amount: Uint128::new(400),
            denom: String::from("acudos"),
        }],
    );
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // error if rentee trying to pay the rent without approval from renter
    let msg = ExecuteMsg::PayRent { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(rentee.as_str(), &coins(200u128, "acudos"));
    let env = mock_env();
    let err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::ExpirationDoesNotExist {});

    let msg = ExecuteMsg::AcceptLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // pay rent 1 time within the month end extends the expiry upto second month
    let msg = ExecuteMsg::PayRent { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(rentee.as_str(), &coins(200u128, "acudos"));
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let q = query_property_info(deps.as_ref(), 1u16).unwrap();
    assert_eq!(
        q,
        FlatInfo {
            renter: renter.to_string(),
            rentee: Some(rentee.to_string()),
            rent: Uint128::new(200),
            expires: Some(Expiration::AtHeight(
                env.clone().block.height + 411428u64 * 2
            ))
        }
    );

    // pay rent 2 time within the month end extends the expiry upto third month
    let msg = ExecuteMsg::PayRent { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(rentee.as_str(), &coins(200u128, "acudos"));
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let q = query_property_info(deps.as_ref(), 1u16).unwrap();
    assert_eq!(
        q,
        FlatInfo {
            renter: renter.to_string(),
            rentee: Some(rentee.to_string()),
            rent: Uint128::new(200),
            expires: Some(Expiration::AtHeight(
                env.clone().block.height + 411428u64 * 3
            ))
        }
    );
    // error if less than requested rent is paid by the rentee.
    let msg = ExecuteMsg::PayRent { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(rentee.as_str(), &coins(100u128, "acudos"));
    let env = mock_env();
    let err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
    assert!(matches!(err, ContractError::LessThanRent {}));
}
#[test]
fn reject_lease() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    // error if no rentee is requested for lease
    let msg = ExecuteMsg::RejectLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    let err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::IsNotRented {});

    // error if invalid renter is trying to reject lease
    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(rentee.as_str(), &coins(400u128, "acudos"));
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = ExecuteMsg::RejectLease { property_id: 1u16 };
    let renter = String::from("new-renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    let err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::InvalidRenter {});

    // successful rejection of request for lease by renter
    let msg = ExecuteMsg::RejectLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let q = query_property_info(deps.as_ref(), 1u16).unwrap();
    assert_eq!(
        q,
        FlatInfo {
            renter: renter.to_string(),
            rentee: None,
            rent: Uint128::new(200),
            expires: None
        }
    );

    // error if invalid renter trying to reject the rentee
    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(rentee.as_str(), &coins(400u128, "acudos"));
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = ExecuteMsg::RejectLease { property_id: 1u16 };
    let renter = String::from("new-renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    let err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::InvalidRenter {});

    // error if rentee accepted by the renter then renter cannot reject it later
    let msg = ExecuteMsg::AcceptLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let msg = ExecuteMsg::RejectLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    let err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
    assert_eq!(err, ContractError::IsAcceptedByRenter {});
}
#[test]
fn query_get_total_properties() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(300),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let total = query_get_total_property(deps.as_ref()).unwrap();
    assert_eq!(total, 2u16);
}
#[test]
fn query_show_all_available() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(300),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::RequestForLease { property_id: 1u16 };
    let rentee = String::from("rentee");
    let info = mock_info(rentee.as_str(), &coins(600u128, String::from("acudos")));
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::AcceptLease { property_id: 1u16 };
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let env = mock_env();
    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    let q = query_show_all_available_properties(deps.as_ref()).unwrap();
    assert_eq!(q, vec![2u16]);
}

#[test]
fn query_property() {
    let mut deps = mock_dependencies();
    let owner = String::from("owner");
    do_instantiate(deps.as_mut(), &owner);
    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(200),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let renter = String::from("renter");
    let info = mock_info(renter.as_str(), &[]);
    let msg = ExecuteMsg::AddProperty {
        rent: Uint128::new(300),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let q = query_property_info(deps.as_ref(), 1u16).unwrap();
    assert_eq!(
        q,
        FlatInfo {
            renter: renter.to_string(),
            rentee: None,
            rent: Uint128::new(200),
            expires: None
        }
    );
    let q = query_property_info(deps.as_ref(), 3u16);
    assert!(q.is_err());
    assert_eq!(q.unwrap_err(), StdError::generic_err("NotFound"));
}
