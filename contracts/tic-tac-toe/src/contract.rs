use cosmwasm_std::{
    coins, entry_point, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, GameResult, InstantiateMsg, QueryMsg};
use crate::state::{Game, GameBoard, State, GAME_MAP, STATE};

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
        ExecuteMsg::CreateGame { bet, zero } => try_create_game(deps, env, info, zero, bet),
        ExecuteMsg::JoinGame { game_id } => try_join_game(deps, info, game_id),
        ExecuteMsg::WithdrawBet { game_id } => try_withdraw_bets(deps, info, game_id),
        ExecuteMsg::UpdateGame {
            game_id,
            side,
            i,
            j,
        } => try_update_game(deps, info, game_id, i, j, side),
    }
}

fn try_withdraw_bets(
    deps: DepsMut,
    info: MessageInfo,
    game_id: Uint128,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    let mut game = GAME_MAP
        .may_load(deps.storage, game_id.to_string())?
        .unwrap();
    if game.is_completed || game.is_pending {
        return Err(ContractError::Unauthorized {});
    }
    let validated_game = find_winner_by_board(game.game)?;
    let res = Response::default();
    let game_bet = game.bet.amount.checked_mul(Uint128::from(2u128)).unwrap();
    match validated_game {
        GameResult::Nought => {
            game.complete_game();
            GAME_MAP.save(deps.storage, game_id.to_string(), &game)?;
            let msg = BankMsg::Send {
                to_address: game.nought.to_string(),
                amount: coins(game_bet.u128(), game.bet.denom),
            };
            Ok(res
                .add_message(msg)
                .add_attribute("to", game.nought.to_string())
                .add_attribute("amount", game_bet.to_string()))
        }
        GameResult::Zero => {
            game.complete_game();
            GAME_MAP.save(deps.storage, game_id.to_string(), &game)?;
            let msg = BankMsg::Send {
                to_address: game.zero.to_string(),
                amount: coins(game_bet.u128(), game.bet.denom),
            };
            Ok(res
                .add_message(msg)
                .add_attribute("to", game.zero.to_string())
                .add_attribute("amount", game_bet.to_string()))
        }
        GameResult::Draw => {
            game.complete_game();
            GAME_MAP.save(deps.storage, game_id.to_string(), &game)?;
            Ok(res)
        }
        GameResult::NoResult => Err(ContractError::Std(StdError::generic_err(
            "Game is not complete",
        ))),
    }
}

pub fn nonpayable(info: &MessageInfo) -> Result<(), ContractError> {
    if info.funds.is_empty() {
        Ok(())
    } else {
        Err(ContractError::Std(StdError::generic_err(
            "does not require coins",
        )))
    }
}

pub fn try_join_game(
    deps: DepsMut,
    info: MessageInfo,
    game_id: Uint128,
) -> Result<Response, ContractError> {
    let mut game = GAME_MAP
        .may_load(deps.storage, game_id.to_string())?
        .unwrap();
    let is_fund_present = info.funds.iter().any(|funds| funds.eq(&game.bet));
    if !is_fund_present {
        return Err(ContractError::Unauthorized {});
    }
    if !game.is_pending || game.is_completed || info.sender != game.zero {
        return Err(ContractError::Unauthorized {});
    }
    game.start_game();
    GAME_MAP.save(deps.storage, game_id.to_string(), &game)?;
    Ok(Response::default().add_attribute("game_id", game_id.to_string()))
}

pub fn try_update_game(
    deps: DepsMut,
    info: MessageInfo,
    game_id: Uint128,
    i: usize,
    j: usize,
    side: bool,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    let mut game = GAME_MAP
        .may_load(deps.storage, game_id.to_string())?
        .unwrap();
    if game.is_pending || game.is_completed || game.next_move != side {
        return Err(ContractError::Unauthorized {});
    }
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
    game.update_side();
    GAME_MAP.save(deps.storage, game_id.to_string(), &game)?;
    Ok(Response::default())
}

pub fn try_create_game(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    zero: String,
    bet: Coin,
) -> Result<Response, ContractError> {
    let is_fund_present = info.funds.iter().any(|funds| funds.eq(&bet));
    if !is_fund_present {
        return Err(ContractError::Unauthorized {});
    }
    let o_address = deps.api.addr_validate(zero.as_str()).unwrap();
    let game = Game::new(&info.sender, &o_address, &bet);
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
        // QueryMsg::FindWinnerUsingBoard { game } => to_binary(&find_winner_by_board(game)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

fn query_game(deps: Deps, game_id: Uint128) -> StdResult<Game> {
    let game = GAME_MAP.load(deps.storage, game_id.to_string())?;
    Ok(game)
}

fn get_winner(deps: Deps, game_id: Uint128) -> StdResult<GameResult> {
    let game = GAME_MAP.load(deps.storage, game_id.to_string())?;
    find_winner_by_board(game.game)
}

fn find_winner_by_board(game: GameBoard) -> StdResult<GameResult> {
    if validate_game(game.clone(), true)? {
        if !is_valid_board(&game, false) {
            return Err(StdError::generic_err("Please check content of tic tac toe"));
        }
        return Ok(GameResult::Nought);
    }
    if validate_game(game.clone(), false)? {
        if !is_valid_board(&game, false) {
            return Err(StdError::generic_err("Please check content of tic tac toe"));
        }
        return Ok(GameResult::Zero);
    }
    if is_valid_board(&game, true) {
        Ok(GameResult::Draw)
    } else {
        Ok(GameResult::NoResult)
    }
}

fn validate_game(arr: GameBoard, is_cross: bool) -> StdResult<bool> {
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

fn validate_rows_or_cols(arr: &GameBoard, is_cross: &bool, is_row: bool) -> bool {
    for i in 0..3 {
        let mut is_win = true;
        for j in 0..3 {
            let mut val = arr[i][j];
            if !is_row {
                val = arr[j][i];
            }
            match val {
                Some(is_eq_cross) => {
                    if is_eq_cross != *is_cross {
                        is_win = false;
                        break;
                    }
                }
                None => {
                    is_win = false;
                    break;
                }
            }
        }
        if is_win {
            return true;
        }
    }
    false
}

fn validate_diagonals(arr: &GameBoard, is_cross: &bool) -> bool {
    if validate_diagonals_inter(arr, is_cross, false) {
        return true;
    } else {
        if validate_diagonals_inter(arr, is_cross, true) {
            return true;
        }
    }
    false
}

fn validate_diagonals_inter(arr: &GameBoard, is_cross: &bool, is_rev: bool) -> bool {
    for i in 0..3 {
        if is_rev {
            if let Some(is_eq_cross) = arr[i][2 - i] {
                if is_eq_cross != *is_cross {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            if let Some(is_eq_cross) = arr[i][i] {
                if is_eq_cross != *is_cross {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn is_valid_board(arr: &GameBoard, is_full_mode: bool) -> bool {
    let mut x_count = 0;
    let mut o_count = 0;
    for row in arr.iter() {
        for col in row.iter() {
            if let Some(data) = col {
                if *data {
                    x_count += 1;
                } else {
                    o_count += 1;
                }
            }
        }
    }
    if is_full_mode {
        if x_count == 5 && o_count == 4 {
            true
        } else {
            false
        }
    } else {
        if x_count == 0 || x_count > 5 || o_count == 0 || o_count > 4 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_binary, SubMsg};

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
        let bet = coin(2u128, "cudos");

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let msg = ExecuteMsg::CreateGame {
            bet: bet.clone(),
            zero: zero.clone(),
        };
        let info = mock_info(&nought, &[bet.clone()]);
        let env = mock_env();
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        let new_bet = coins(1u128, "cudos");
        let msg = ExecuteMsg::CreateGame {
            bet: bet.clone(),
            zero: zero.clone(),
        };
        let info = mock_info(&nought, &new_bet);
        let mut env = mock_env();
        env.block.height += 100u64;
        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
        assert_eq!(res, ContractError::Unauthorized {});
    }
    #[test]
    fn join_game() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let nought = String::from("Nought");
        let zero = String::from("zero");
        let bet = coin(2u128, "cudos");

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let msg = ExecuteMsg::CreateGame {
            bet: bet.clone(),
            zero: zero.clone(),
        };
        let info = mock_info(&nought, &[bet.clone()]);
        let env = mock_env();
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        assert_eq!(d.is_pending, true);
        let msg = ExecuteMsg::JoinGame {
            game_id: Uint128::from(env.block.height),
        };
        let info = mock_info(&zero, &[bet.clone()]);
        let env = mock_env();
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        assert_eq!(d.is_pending, false);
    }
    #[test]
    fn update_game() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let nought = String::from("Nought");
        let zero = String::from("zero");
        let bet = coin(2u128, "cudos");

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let msg = ExecuteMsg::CreateGame {
            bet: bet.clone(),
            zero: zero.clone(),
        };
        let info = mock_info(&nought, &[bet.clone()]);
        let env = mock_env();
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        assert_eq!(d.is_pending, true);

        let msg = ExecuteMsg::JoinGame {
            game_id: Uint128::from(env.block.height),
        };
        let info = mock_info(&zero, &[bet.clone()]);
        let env = mock_env();
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        assert_eq!(d.is_pending, false);
        let following: [(&String, bool, usize, usize); 9] = [
            (&nought, true, 0, 0),
            (&zero, false, 2, 0),
            (&nought, true, 0, 2),
            (&zero, false, 1, 0),
            (&nought, true, 1, 2),
            (&zero, false, 1, 1),
            (&nought, true, 2, 2),
            (&zero, false, 2, 1),
            (&nought, true, 0, 1),
        ];
        for (sender, com, i, j) in following {
            let msg = ExecuteMsg::UpdateGame {
                game_id: Uint128::from(env.block.height),
                side: com,
                i: i,
                j: j,
            };
            let info = mock_info(sender, &[]);
            let env = mock_env();
            let _res = execute(deps.as_mut(), env, info, msg).unwrap();
        }
        let msg = ExecuteMsg::UpdateGame {
            game_id: Uint128::from(env.block.height),
            side: false,
            i: 0,
            j: 0,
        };
        let info = mock_info(&zero, &[]);
        let env = mock_env();
        let res = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(
            res,
            ContractError::Std(StdError::generic_err("illegal move"))
        );
    }
    #[test]
    fn withdraw_bet() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let nought = String::from("Nought");
        let zero = String::from("zero");
        let bet = coin(2u128, "cudos");

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        let msg = ExecuteMsg::CreateGame {
            bet: bet.clone(),
            zero: zero.clone(),
        };
        let info = mock_info(&nought, &[bet.clone()]);
        let env = mock_env();
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        assert_eq!(d.is_pending, true);

        let msg = ExecuteMsg::JoinGame {
            game_id: Uint128::from(env.block.height),
        };
        let info = mock_info(&zero, &[bet.clone()]);
        let env = mock_env();
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        assert_eq!(d.is_pending, false);
        let following: [(&String, bool, usize, usize); 9] = [
            (&nought, true, 0, 0),
            (&zero, false, 2, 0),
            (&nought, true, 0, 2),
            (&zero, false, 1, 0),
            (&nought, true, 1, 2),
            (&zero, false, 1, 1),
            (&nought, true, 2, 2),
            (&zero, false, 2, 1),
            (&nought, true, 0, 1),
        ];
        for (sender, com, i, j) in following {
            let msg = ExecuteMsg::UpdateGame {
                game_id: Uint128::from(env.block.height),
                side: com,
                i: i,
                j: j,
            };
            let info = mock_info(sender, &[]);
            let env = mock_env();
            let _res = execute(deps.as_mut(), env, info, msg).unwrap();
        }
        let msg = ExecuteMsg::WithdrawBet {
            game_id: Uint128::from(env.block.height),
        };
        let info = mock_info("anyone", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(
            &res.messages[0],
            &SubMsg::new(BankMsg::Send {
                to_address: nought.to_string(),
                amount: coins(4u128, "cudos"),
            })
        );
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        assert_eq!(d.is_completed, true);

        let msg = ExecuteMsg::CreateGame {
            bet: bet.clone(),
            zero: zero.clone(),
        };
        let info = mock_info(&nought, &[bet.clone()]);
        let mut env = mock_env();
        env.block.height += 100u64;

        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        assert_eq!(d.is_pending, true);

        let msg = ExecuteMsg::JoinGame {
            game_id: Uint128::from(env.block.height),
        };
        let info = mock_info(&zero, &[bet.clone()]);
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let following: [(&String, bool, usize, usize); 5] = [
            (&nought, true, 0, 0),
            (&zero, false, 2, 0),
            (&nought, true, 0, 2),
            (&zero, false, 1, 0),
            (&nought, true, 0, 1),
        ];
        for (sender, com, i, j) in following {
            let msg = ExecuteMsg::UpdateGame {
                game_id: Uint128::from(env.block.height),
                side: com,
                i: i,
                j: j,
            };
            let info = mock_info(sender, &[]);
            let env = mock_env();
            let _res = execute(deps.as_mut(), env, info, msg).unwrap();
        }
        let msg = ExecuteMsg::WithdrawBet {
            game_id: Uint128::from(env.block.height),
        };
        let info = mock_info("anyone", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(
            &res.messages[0],
            &SubMsg::new(BankMsg::Send {
                to_address: nought.to_string(),
                amount: coins(4u128, "cudos"),
            })
        );
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        assert_eq!(d.is_completed, true);

        let msg = ExecuteMsg::CreateGame {
            bet: bet.clone(),
            zero: zero.clone(),
        };
        let info = mock_info(&nought, &[bet.clone()]);
        let mut env = mock_env();
        env.block.height += 1001u64;

        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let d = query_game(deps.as_ref(), Uint128::from(env.block.height)).unwrap();
        matches!(d, Game { .. });
        assert_eq!(d.is_pending, true);

        let msg = ExecuteMsg::JoinGame {
            game_id: Uint128::from(env.block.height),
        };
        let info = mock_info(&zero, &[bet.clone()]);
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let following: [(&String, bool, usize, usize); 4] = [
            (&nought, true, 0, 0),
            (&zero, false, 2, 0),
            (&nought, true, 0, 2),
            (&zero, false, 1, 0),
        ];
        for (sender, com, i, j) in following {
            let msg = ExecuteMsg::UpdateGame {
                game_id: Uint128::from(env.block.height),
                side: com,
                i: i,
                j: j,
            };
            let info = mock_info(sender, &[]);
            let env = mock_env();
            let _res = execute(deps.as_mut(), env, info, msg).unwrap();
        }
        let msg = ExecuteMsg::WithdrawBet {
            game_id: Uint128::from(env.block.height),
        };
        let info = mock_info("anyone", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(
            res,
            ContractError::Std(StdError::generic_err("Game is not complete"))
        )
    }
}
