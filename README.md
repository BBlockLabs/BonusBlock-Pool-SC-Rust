# Smart Contract basic usage/deployment
Examples provided for the Archway blockchain using `archwayd` CLI tool

## Deployment
### Store contract code
```
archwayd tx wasm store cosmos_reward_pool_contract.wasm --from admin --gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 5000000 | jq -r '.txhash'
```

### Query code id
```
archwayd q tx <previous-tx-hash> | jq -r '.logs [0] .events[] | select(.type=="store_code").attributes[] | select(.key=="code_id") | .value'
```

### Instantiate contract
```
archwayd tx wasm instantiate <code-id> '{}' --gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 5000000 --label test123 --no-admin --from admin | jq -r '.txhash'
```

### Get the contract address
```
archwayd q tx <previous-tx-hash> | jq -r '.logs [0] .events[] | select(.type=="instantiate").attributes[] | select(.key=="_contract_address") | .value'
```

The latest deployed contract is archway124ljgdsns7zqngyx0jengsh90kh06jv9dqq8kxuvnw4509mhrwgqmlnrg7

## Function calls
**deposit** — Create a new reward pool
```
archwayd tx wasm execute archway124ljgdsns7zqngyx0jengsh90kh06jv9dqq8kxuvnw4509mhrwgqmlnrg7 '{ "deposit" : { "campaign_id": "12345" } }' --amount 1000000aconst --from admin --gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 400000 | jq -r '.txhash'
```


**claim** — Claim a reward that was assigned to a user
```
archwayd tx wasm execute archway124ljgdsns7zqngyx0jengsh90kh06jv9dqq8kxuvnw4509mhrwgqmlnrg7 '{ "claim" : {"campaign_id": "12345"} }' --from admin--gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 400000 | jq -r '.txhash'
```

**withdraw** — Withdraw a specific amount of coins from the contract
```
archwayd tx wasm execute archway124ljgdsns7zqngyx0jengsh90kh06jv9dqq8kxuvnw4509mhrwgqmlnrg7 '{ "withdraw" : {"amount": "1000"} }' --from admin --gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 400000 | jq -r '.txhash'
```



**set_cpool** — Modify the amount of a campaign pool, create new if it doesn’t exist
```
archwayd tx wasm execute archway124ljgdsns7zqngyx0jengsh90kh06jv9dqq8kxuvnw4509mhrwgqmlnrg7 '{ "set_cpool" : {"campaign_id": "1111", "amount": "1000"} }' --from admin --gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 400000 | jq -r '.txhash'
```



## Queries

**get_cpool** — Query campaign pool
```
archwayd q wasm contract-state smart archway124ljgdsns7zqngyx0jengsh90kh06jv9dqq8kxuvnw4509mhrwgqmlnrg7 '{ "get_cpool" : { "campaign_id": "12345" } }'
```

