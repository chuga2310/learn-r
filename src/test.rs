#[test]
    fn works_with_add_new_and_sell() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        
        let pen_id = "1234";
        let owner = "aura1csk4psx5gz0l7c9u65289pwecntpwk9vf0c2xv";

        let msg = ExecuteMsg::Mint {
            id: pen_id.to_string(),
            owner : owner.to_string()
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
        )
        .unwrap();
        let pen = Pen {
            id: pen_id.to_string(),
            owner : owner.to_string()
        };
        let expected = PenInfoResponse {
            pen: Some(pen),
        };
        let value: PenInfoResponse = from_binary(&res).unwrap();
        assert_eq!(expected, value);
    }
