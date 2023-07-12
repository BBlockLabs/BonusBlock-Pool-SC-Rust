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

The latest deployed contract is archway1nh0nkr0u3axd87asedm8s857ld50z49j3kkssh3f2ada769uma7q62dlhn

## Function calls
**deposit** — Create reward pool 12345 with size 1000000aconst
```
archwayd tx wasm execute archway1nh0nkr0u3axd87asedm8s857ld50z49j3kkssh3f2ada769uma7q62dlhn '{ "deposit" : { "campaign_id": "12345" } }' --amount 1000000aconst --from admin --gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 400000 | jq -r '.txhash'
```

**check** — Check if the reward pool has been created and calculate the transfer fee
```
archwayd tx wasm execute archway1nh0nkr0u3axd87asedm8s857ld50z49j3kkssh3f2ada769uma7q62dlhn '{ "check" : {"requests": [{"campaign_id": "12345", "amount": "900000"}]} }' --from admin --gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 400000 | jq -r '.txhash'
```

**reward_all** — Assign reward of 10000uconst to user archway18v032krrt0sud25y2vk9vj49lvwkg2hxlxupwf 
```
archwayd tx wasm execute archway1nh0nkr0u3axd87asedm8s857ld50z49j3kkssh3f2ada769uma7q62dlhn '{ "reward_all" : {"user_rewards": [{"campaign_id": "12345", "user_address": "archway18v032krrt0sud25y2vk9vj49lvwkg2hxlxupwf", "amount": "10000"}]} }' --from admin --gas-prices $(archwayd q rewards estimate-fees 1 | jq -r '.gas_unit_price | (.amount + .denom)') -y --gas 400000 | jq -r '.txhash'
```

## Queries

**get_cpool** — Query campaign pool 12345
```
archwayd q wasm contract-state smart archway1nh0nkr0u3axd87asedm8s857ld50z49j3kkssh3f2ada769uma7q62dlhn '{ "get_cpool" : { "campaign_id": "12345" } }'
```

**get_upool** — Query user pool for user archway18v032krrt0sud25y2vk9vj49lvwkg2hxlxupwf and reward pool 12345
```
archwayd q wasm contract-state smart archway1nh0nkr0u3axd87asedm8s857ld50z49j3kkssh3f2ada769uma7q62dlhn '{ "get_upool" : { "user_address": "archway18v032krrt0sud25y2vk9vj49lvwkg2hxlxupwf", "campaign_id": "12345" } }'
Multiple options can be selected.
```
