#[test]
fn works_with_add_new_and_sell() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let pen_id = "1234";
    let owner = "aura1csk4psx5gz0l7c9u65289pwecntpwk9vf0c2xv";
    let quality = "uncommon";
    let level = 2;
    let effect = 4;
    let resilience = 4;
    let number_of_mints = 0;
    let durability = 100;
    let extension = ExtensionPen {
        quality: quality.to_string(),
        level: level,
        effect: effect,
        resilience: resilience,
        number_of_mints: number_of_mints,
        durability: durability,
    };
    let msg = ExecuteMsg::Mint {
        id: pen_id.to_string(),
        owner: owner.to_string(),
        extension,
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
            id: pen_id.to_string(),
        },
        QueryMsg::GetMetadata {
            id: pen_id.to_string(),
        },
    )
    .unwrap();
    let extension = ExtensionPen {
        level: level,
        effect: effect,
        resilience: resilience,
        number_of_mints: number_of_mints,
        durability: durability,
    };
    let pen = Pen {
        id: pen_id.to_string(),
        owner: owner.to_string(),
        extension,
    };
    let expected = PenInfoResponse { pen: Some(pen) };
    let value: PenInfoResponse = from_binary(&res).unwrap();
    assert_eq!(expected, value);
}
