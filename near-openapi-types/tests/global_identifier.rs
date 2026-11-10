use near_openapi_types::NonDelegateAction;
use serde_json::json;

#[test]
fn delegated_global_account_accepts_rpc_view_and_preserves_canonical_json() {
    for tag in ["account_id", "AccountId"] {
        let input = json!({"UseGlobalContract": {
            "contract_identifier": {tag: "oracle.near"}
        }});
        let action: NonDelegateAction = serde_json::from_value(input).unwrap();
        assert_eq!(
            serde_json::to_value(action).unwrap(),
            json!({"UseGlobalContract": {
                "contract_identifier": {"AccountId": "oracle.near"}
            }})
        );
    }
}

#[test]
fn delegated_global_hash_accepts_rpc_view_and_preserves_canonical_json() {
    for tag in ["hash", "CodeHash"] {
        let input = json!({"UseGlobalContract": {
            "contract_identifier": {tag: "11111111111111111111111111111111"}
        }});
        let action: NonDelegateAction = serde_json::from_value(input).unwrap();
        assert_eq!(
            serde_json::to_value(action).unwrap(),
            json!({"UseGlobalContract": {
                "contract_identifier": {"CodeHash": "11111111111111111111111111111111"}
            }})
        );
    }
}
