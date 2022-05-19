use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use tic_tac_toe::msg::{CountResponse, ExecuteMsg, GameResult, InstantiateMsg, QueryMsg};
use tic_tac_toe::state::{Game, GameBoard};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(CountResponse), &out_dir);
    export_schema(&schema_for!(GameResult), &out_dir);
    export_schema(&schema_for!(Game), &out_dir);
    export_schema(&schema_for!(GameBoard), &out_dir);
}
