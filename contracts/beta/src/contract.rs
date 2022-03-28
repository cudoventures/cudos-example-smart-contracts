use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender,
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::default())
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::ValidateGame { arr, is_cross } => to_binary(&validate_game(arr, is_cross)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

fn validate_game(arr: [[Option<bool>; 3]; 3], is_cross: bool) -> StdResult<bool> {
    if !is_not_empty(&arr) {
        return Err(StdError::generic_err("Please check content of tic tac toe"));
    }

    if validate_rows_or_cols(&arr, &is_cross, true) {
        return Ok(true);
    }

    if validate_rows_or_cols(&arr, &is_cross, false) {
        return Ok(true);
    }

    if validate_diagonals(&arr, &is_cross) {
        return Ok(true);
    }

    Ok(false)
}

fn validate_rows_or_cols(arr: &[[Option<bool>; 3]; 3], is_cross: &bool, is_row: bool) -> bool {
    for i in 0..3 {
        let mut is_win = true;
        for j in 0..3 {
            if is_row {
                if let Some(is_eq_cross) = arr[i][j] {
                    if is_eq_cross != *is_cross {
                        is_win = false;
                        break;
                    }
                }
            } else {
                if let Some(is_eq_cross) = arr[j][i] {
                    if is_eq_cross != *is_cross {
                        is_win = false;
                        break;
                    }
                }
            }
        }
        if is_win {
            return true;
        }
    }
    false
}

fn validate_diagonals(arr: &[[Option<bool>; 3]; 3], is_cross: &bool) -> bool {
    if validate_diagonals_inter(arr, is_cross, false) {
        return true;
    } else {
        if validate_diagonals_inter(arr, is_cross, true) {
            return true;
        }
    }
    false
}

fn validate_diagonals_inter(arr: &[[Option<bool>; 3]; 3], is_cross: &bool, is_rev: bool) -> bool {
    for i in 0..3 {
        if is_rev {
            if let Some(is_eq_cross) = arr[i][2 - i] {
                if is_eq_cross != *is_cross {
                    return false;
                }
            }
        } else {
            if let Some(is_eq_cross) = arr[i][i] {
                if is_eq_cross != *is_cross {
                    return false;
                }
            }
        }
    }
    true
}

fn is_not_empty(arr: &[[Option<bool>; 3]; 3]) -> bool {
    let mut count = 0;
    for row in arr.iter() {
        for col in row.iter() {
            if let Some(data) = col {
                if *data {
                    count += 1;
                } else {
                    count -= 1
                }

                continue;
            }
            return false;
        }
    }
    if count != 1 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }

    #[test]
    fn test_validate() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ValidateGame {
                arr: [
                    [Some(true), Some(true), Some(false)],
                    [Some(true), Some(false), Some(true)],
                    [Some(true), Some(false), Some(false)],
                ],
                is_cross: true,
            },
        )
        .unwrap();
        let value: bool = from_binary(&res).unwrap();
        assert_eq!(value, true);

        // verifing diagonals
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ValidateGame {
                arr: [
                    [Some(false), Some(true), Some(true)],
                    [Some(false), Some(false), Some(true)],
                    [Some(true), Some(true), Some(false)],
                ],
                is_cross: false,
            },
        )
        .unwrap();
        let value: bool = from_binary(&res).unwrap();
        assert_eq!(value, true);

        // verifing rev diagonals
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ValidateGame {
                arr: [
                    [Some(true), Some(true), Some(false)],
                    [Some(false), Some(false), Some(true)],
                    [Some(false), Some(true), Some(true)],
                ],
                is_cross: false,
            },
        )
        .unwrap();
        let value: bool = from_binary(&res).unwrap();
        assert_eq!(value, true);
        // verifying row
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ValidateGame {
                arr: [
                    [Some(true), Some(true), Some(true)],
                    [Some(false), Some(false), Some(true)],
                    [Some(false), Some(false), Some(true)],
                ],
                is_cross: true,
            },
        )
        .unwrap();
        let value: bool = from_binary(&res).unwrap();
        assert_eq!(value, true);

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ValidateGame {
                arr: [
                    [Some(true), Some(true), Some(false)],
                    [Some(true), Some(false), Some(true)],
                    [Some(true), Some(false), None],
                ],
                is_cross: true,
            },
        )
        .unwrap_err();
        assert_eq!(
            res,
            StdError::generic_err("Please check content of tic tac toe")
        )
    }
}
