#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cosmwasm_std::StdError;

use crate::error::ContractError;
use crate::msg::{PenInfoResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{store, Pen, store_query};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:learn-r";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let pen = Pen {
        token_id: msg.name,
        id: msg.id,
        owner: msg.owner,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let key = pen.id.as_bytes();
    store(deps.storage).save(key, &pen)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddNew {
            token_id,
            id,
            owner,
        } => mint(deps, token_id, id, owner),
        // ExecuteMsg::Sell { id, amount } => sell(deps, id, amount),
    }
}

pub fn mint(
    deps: DepsMut,
    token_id: String,
    id: String,
    owner: i32,
) -> Result<Response, ContractError> {
    let pen = Pen {
        token_id,
        id,
        owner,
    };
    let key = pen.id.as_bytes();
    if (store(deps.storage).may_load(key)?).is_some() {
        // id is already taken
        return Err(ContractError::IdTaken { token_id: pen.token_id });
    }
    store(deps.storage).save(key, &pen)?;
    Ok(Response::new()
        .add_attribute("method", "mint")
        .add_attribute("token_id", pen.token_id))
}

// pub fn sell(deps: DepsMut, id: String, amount: i32) -> Result<Response, ContractError> {
//     let key = id.as_bytes();
//     store(deps.storage).update(key, |record| {
//         if let Some(mut record) = record {
//             if amount > record.amount {
//                 //The amount of pens left is not enough
//                 return Err(ContractError::NotEnoughAmount {});
//             }
//             record.amount -= amount;
//             Ok(record)
//         } else {
//             Err(ContractError::IdNotExists { id: id.clone() })
//         }
//     })?;

//     Ok(Response::new().add_attribute("method", "sell"))
// }


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPen { id } => query_pen(deps, token_id),
    }
}

fn query_pen(deps: Deps, token_id: String) -> StdResult<Binary> {
    let key = id.as_bytes();
    let pen = match store_query(deps.storage).may_load(key)? {
        Some(pen) => Some(pen),
        None => return Err(StdError::generic_err("Pen does not exist")),
    };

    let resp = PenInfoResponse { pen };
    to_binary(&resp)
}
