pub mod contract;
mod error;
pub mod msg;
pub mod state;

#[test]
fn not_works_with_add_new_id_existed() {
    use contract::execute;
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
    };
    use error::ContractError;
    use msg::ExecuteMsg;
    use state::ExtensionPen;
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
    let pen_id = "1234";
    let owner = "aura1csk4psx5gz0l7c9u65289pwecntpwk9vf0c2xv";
    let quality = "uncommon";
    let level: i32 = 2;
    let effect: i32 = 5;
    let resilience: i32 = 6;
    let number_of_mints: i32 = 7;
    let durability: i32 = 7;
    let extension = ExtensionPen {
        quality: quality.to_string(),
        level: level,
        effect: effect,
        resilience: resilience,
        number_of_mints: number_of_mints,
        durability: durability,
    };
    let msg_asiatic = ExecuteMsg::Mint {
        id: pen_id.to_string(),
        owner: owner.to_string(),
        extension,
    };
    let info = mock_info("creator", &coins(1000, "earth"));
    // we can just call .unwrap() to assert this was a success
    let res = execute(deps.as_mut(), mock_env(), info, msg_asiatic).unwrap();
    assert_eq!(0, res.messages.len());

    let info = mock_info("creator", &coins(1000, "earth"));
    let extension = ExtensionPen {
        quality: quality.to_string(),
        level: level,
        effect: effect,
        resilience: resilience,
        number_of_mints: number_of_mints,
        durability: durability,
    };
    let msg_oriental = ExecuteMsg::Mint {
        id: pen_id.to_string(),
        owner: owner.to_string(),
        extension,
    };
    let err = execute(deps.as_mut(), mock_env(), info, msg_oriental).unwrap_err();
    match err {
        ContractError::IdTaken { id } => {
            assert_eq!(pen_id.to_string(), id);
        }
        e => panic!("unexpected error: {}", e),
    }
}

/* Only run when test url metadata */
// #[test]
// fn works_with_add_new_and_sell() {
//     use crate::contract::query;
//     use cosmwasm_std::{from_binary, to_binary};

//     use contract::execute;
//     use cosmwasm_std::{
//         coins,
//         testing::{mock_dependencies_with_balance, mock_env, mock_info},
//     };
//     use msg::ExecuteMsg;
//     use msg::{MetadataPenResponse, QueryMsg};
//     use state::ExtensionPen;
//     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

//     let pen_id = "1234";
//     let owner = "aura1csk4psx5gz0l7c9u65289pwecntpwk9vf0c2xv";
//     let quality = "uncommon";
//     let level = 2;
//     let effect = 4;
//     let resilience = 4;
//     let number_of_mints = 0;
//     let durability = 100;
//     let extension = ExtensionPen {
//         quality: quality.to_string(),
//         level: level,
//         effect: effect,
//         resilience: resilience,
//         number_of_mints: number_of_mints,
//         durability: durability,
//     };
//     let msg = ExecuteMsg::Mint {
//         id: pen_id.to_string(),
//         owner: owner.to_string(),
//         extension,
//     };
//     let info = mock_info("creator", &coins(1000, "earth"));
//     // we can just call .unwrap() to assert this was a success
//     let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
//     assert_eq!(0, res.messages.len());
//     // it worked, let's query the pen
//     let res = query(
//         deps.as_ref(),
//         mock_env(),
//         QueryMsg::GetMetadata {
//             id: pen_id.to_string(),
//         },
//     )
//     .unwrap();

//     let url: String = from_binary(&res).unwrap();
//     println!("URL: {url}");
//     assert_eq!(false, true);
// }
