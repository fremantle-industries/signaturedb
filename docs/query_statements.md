# Query Statements - SignatureDB

## INSERT

```sql
INSERT INTO usdc.token_balances (account, value)
VALUES ('0x1', 10.1)
```
## SELECT

```sql
SELECT
  usdc.token_balances.account,
  usdc.token_balances.value AS usdc_value,
  aave.token_balances.value AS aave_value
FROM usdc.token_balances
  INNER JOIN aave.token_balances ON aave.token_balances.account = usdc.token_balances.account
```

## UPDATE

```sql
```

## DELETE

```sql
```
