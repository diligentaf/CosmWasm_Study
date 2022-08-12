// use cosmwasm_std::{
//     entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
// };

// #[entry_point] // raw Wasm entry point
// pub fn instantiate(
//     _deps: DepsMut, // querying and updating the contract state, querying other contract state, helper functions for dealing with cw address
//     _env: Env, // object representing blockchains state when executing the message - chain height, id, current timestamp, contract address
//     _info: MessageInfo, // metainformation about the message which triggered an execution. address that send the message, chain token sent with message
//     _msg: Empty, // message triggering execution itself. Argument can be anything that's deserializable
// ) -> StdResult<Response> {
//     Ok(Response::new())
// }

/// ###################################################################3

// use cosmwasm_std::{
//     entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo,
//     Response, StdResult,
// };
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct QueryResp {
//     message: String,
// }

// #[entry_point]
// pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
//     let resp = QueryResp {
//         message: "Hello World".to_owned(),
//     };

//     to_binary(&resp)
// }

// ###################################################################3

// use cosmwasm_std::{
//     entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo,
//     Response, StdResult,
// };

// use serde::{Deserialize, Serialize};
//  #[derive(Serialize, Deserialize)]
//  pub struct GreetResp {
//      message: String,
//  }
 
//  #[entry_point]
//  pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//      use QueryMsg::*;
 
//      match msg {
//          Greet {} => to_binary(&query::greet()?),
//      }
//  }
 
//  mod query {
//      use super::*;
 
//      pub fn greet() -> StdResult<GreetResp> {
//          let resp = GreetResp {
//              message: "Hello World".to_owned(),
//          };
 
//          Ok(resp)
//      }
//  }

// ###################################################################3

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

mod contract;
mod msg;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty)
  -> StdResult<Response>
{
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg)
  -> StdResult<Binary>
{
    contract::query(deps, env, msg)
}
