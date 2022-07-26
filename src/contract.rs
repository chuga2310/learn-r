#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::StdError;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use schemars::_serde_json::to_string;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, MetadataMsg, MetadataPenResponse, PenInfoResponse, QueryMsg,
};
use crate::state::{store, store_query, ExtensionPen, Pen};

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
        id: "-1".to_string(),
        owner: msg.owner,
        extension: msg.extension,
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
            extension,
        } => mint(deps, id, owner, extension),
        // ExecuteMsg::Sell { id, amount } => sell(deps, id, amount),
    }
}

pub fn mint(
    deps: DepsMut,
    id: String,
    owner: String,
    extension: ExtensionPen,
) -> Result<Response, ContractError> {
    let pen = Pen {
        id,
        owner,
        extension,
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn metadata(deps: Deps, _env: Env, msg: MetadataMsg) -> StdResult<String> {
    match msg {
        MetadataMsg::GetMetadata { id } => get_metadata(deps, _env, id),
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

fn get_metadata(deps: Deps, _env: Env, id: String) -> StdResult<String> {
    let key = id.as_bytes();
    let pen = match store_query(deps.storage).may_load(key)? {
        Some(pen) => Some(pen),
        None => return Err(StdError::generic_err("Pen does not exist")),
    };

    let mut url = "http://learn.com/metadata/".to_owned();
    let contract = _env.contract.address.to_string();
    url.push_str(&contract);
    url.push_str("/token/");
    url.push_str(&pen.unwrap().id.to_string());
    return Ok(url);
}
