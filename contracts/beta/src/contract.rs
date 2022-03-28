use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Game, State, GAME_MAP, STATE};

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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::CreateGame { nought, zero } => try_create_game(deps, env, nought, zero),
        ExecuteMsg::UpdateGame {
            game_id,
            side,
            i,
            j,
        } => try_update_game(deps, info, game_id, i, j, side),
    }
}

pub fn try_update_game(
    deps: DepsMut,
    info: MessageInfo,
    game_id: u64,
    i: usize,
    j: usize,
    side: bool,
) -> Result<Response, ContractError> {
    let mut game = GAME_MAP
        .may_load(deps.storage, game_id.to_string())?
        .unwrap();
    if game.next_move == side {
        if side == true {
            if game.nought != info.sender {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: String::from("sender is not a x"),
                }));
            }
        } else {
            if game.zero != info.sender {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: String::from("sender is not a 0"),
                }));
            }
        }

        let success = game.update_game(i, j, side);
        if !success {
            return Err(ContractError::Std(StdError::generic_err("illegal move")));
        }
        game.next_move = !side;
    }
    GAME_MAP.save(deps.storage, game_id.to_string(), &game)?;
    Ok(Response::default())
}

pub fn try_create_game(
    deps: DepsMut,
    env: Env,
    nought: String,
    zero: String,
) -> Result<Response, ContractError> {
    let x_address = deps.api.addr_validate(nought.as_str()).unwrap();
    let o_address = deps.api.addr_validate(zero.as_str()).unwrap();
    let game = Game::new(&x_address, &o_address);
    GAME_MAP.save(deps.storage, env.block.height.to_string(), &game)?;
    Ok(Response::default().add_attribute(String::from("id"), env.block.height.to_string()))
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
        QueryMsg::GetWinner { game_id } => to_binary(&get_winner(deps, game_id)?),
        QueryMsg::QueryGame { game_id } => to_binary(&query_game(deps, game_id)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

fn query_game(deps: Deps, game_id: u64) -> StdResult<Game> {
    let game = GAME_MAP.load(deps.storage, game_id.to_string())?;
    Ok(game)
}

fn get_winner(deps: Deps, game_id: u64) -> StdResult<String> {
    let game = GAME_MAP.load(deps.storage, game_id.to_string())?;
    if validate_game(game.game.clone(), true)? {
        return Ok(String::from("Nought"));
    }
    if validate_game(game.game.clone(), false)? {
        return Ok(String::from("Zought"));
    }
    Ok(String::from("Draw"))
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
    use std::convert::TryFrom;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr, Api};

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
    fn create_game() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let nought = String::from("Nought");
        let zero = String::from("zero");

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let env = mock_env();
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        let msg = ExecuteMsg::CreateGame {
            nought: nought.clone(),
            zero: zero.clone(),
        };
        let info = mock_info("creator", &coins(2, "token"));
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        query_game(deps.as_ref(), env.block.height).unwrap();

        let msg = ExecuteMsg::UpdateGame {
            game_id: env.block.height,
            side: true,
            i: 1,
            j: 1,
        };
        let info = mock_info(&nought, &[]);
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), env.block.height).unwrap();
        let msg = ExecuteMsg::UpdateGame {
            game_id: env.block.height,
            side: false,
            i: 1,
            j: 1,
        };
        let info = mock_info(&nought, &[]);
        let e = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(
            e,
            ContractError::Std(StdError::generic_err("sender is not a 0"))
        );
        let msg = ExecuteMsg::UpdateGame {
            game_id: env.block.height,
            side: false,
            i: 1,
            j: 1,
        };
        let info = mock_info(&zero, &[]);
        let e = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(e, ContractError::Std(StdError::generic_err("illegal move")));
        
    }
}
