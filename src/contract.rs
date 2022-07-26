#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::StdError;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PenInfoResponse, QueryMsg};
use crate::state::{store, store_query, Pen};

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
        id: msg.id,
        owner: msg.owner,
        quality: msg.quality,
        level: msg.level,
        effect: msg.effect,
        resilience: msg.resilience,
        number_of_mints: msg.number_of_mints,
        durability: msg.durability,
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
        ExecuteMsg::Mint {
            id,
            owner,
            quality,
            level,
            effect,
            resilience,
            number_of_mints,
            durability,
        } => mint(
            deps,
            id,
            owner,
            quality,
            level,
            effect,
            resilience,
            number_of_mints,
            durability,
        ),
        // ExecuteMsg::Sell { id, amount } => sell(deps, id, amount),
    }
}

pub fn mint(
    deps: DepsMut,
    id: String,
    owner: String,
    quality: String,
    level: i32,
    effect: i32,
    resilience: i32,
    number_of_mints: i32,
    durability: i32,
) -> Result<Response, ContractError> {
    let pen = Pen {
        id,
        owner,
        quality,
        level,
        effect,
        resilience,
        number_of_mints,
        durability,
    };
    let key = pen.id.as_bytes();
    if (store(deps.storage).may_load(key)?).is_some() {
        // id is already taken
        return Err(ContractError::IdTaken { id: pen.id });
    }
    store(deps.storage).save(key, &pen)?;
    Ok(Response::new()
        .add_attribute("method", "Mint")
        .add_attribute("id", pen.id))
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
        QueryMsg::GetPen { id } => query_pen(deps, id),
    }
}

fn query_pen(deps: Deps, id: String) -> StdResult<Binary> {
    let key = id.as_bytes();
    let pen = match store_query(deps.storage).may_load(key)? {
        Some(pen) => Some(pen),
        None => return Err(StdError::generic_err("Pen does not exist")),
    };

    let resp = PenInfoResponse { pen };
    to_binary(&resp)
}
