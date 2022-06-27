pub mod contract;
mod error;
pub mod msg;
pub mod state;

#[test]
fn not_works_with_add_new_id_existed() {
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
    };
    use msg::{ExecuteMsg, PenInfoResponse, QueryMsg};

    use contract::{execute, query};
    use error::ContractError;
    use state::Pen;
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
    let lily_id = "lily_id";
    let msg_asiatic = ExecuteMsg::AddNew {
        id: lily_id.to_string(),
        name: "Asiatic lilies".to_string(),
        amount: 100,
        price: 100,
    };
    let info = mock_info("creator", &coins(1000, "earth"));
    // we can just call .unwrap() to assert this was a success
    let res = execute(deps.as_mut(), mock_env(), info, msg_asiatic).unwrap();
    assert_eq!(0, res.messages.len());

    let info = mock_info("creator", &coins(1000, "earth"));
    let msg_oriental = ExecuteMsg::AddNew {
        id: lily_id.to_string(),
        name: "Oriental lilies".to_string(),
        amount: 100,
        price: 100,
    };
    let err = execute(deps.as_mut(), mock_env(), info, msg_oriental).unwrap_err();
    match err {
        ContractError::IdTaken { id } => {
            assert_eq!(lily_id.to_string(), id);
        }
        e => panic!("unexpected error: {}", e),
    }
}

#[test]
fn works_with_add_new_and_sell() {
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
    };
    use msg::{ExecuteMsg, PenInfoResponse, QueryMsg};

    use contract::{execute, query};
    use error::ContractError;
    use state::Pen;
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = ExecuteMsg::AddNew {
        id: "lily_id".to_string(),
        name: "lily".to_string(),
        amount: 100,
        price: 100,
    };
    let info = mock_info("creator", &coins(1000, "earth"));
    // we can just call .unwrap() to assert this was a success
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    // it worked, let's query the pen
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetPen {
            id: "lily_id".to_string(),
        },
    )
    .unwrap();
    let pen = Pen {
        id: "lily_id".to_string(),
        name: "lily".to_string(),
        amount: 100,
        price: 100,
    };
    let expected = PenInfoResponse { pen: Some(pen) };
    let value: PenInfoResponse = from_binary(&res).unwrap();
    assert_eq!(expected, value);

    let msg = ExecuteMsg::Sell {
        id: "lily_id".to_string(),
        amount: 45,
    };
    let info = mock_info("creator", &coins(1000, "earth"));
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    // it worked, let's query the flower
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetPen {
            id: "lily_id".to_string(),
        },
    )
    .unwrap();
    let pen = Pen {
        id: "lily_id".to_string(),
        name: "lily".to_string(),
        amount: 55,
        price: 100,
    };
    let expected = PenInfoResponse { pen: Some(pen) };
    let value: PenInfoResponse = from_binary(&res).unwrap();
    assert_eq!(expected, value);
}
