## Everscale states RPC

### How to build

```bash
RUSTFLAGS='-C target_cpu=native' cargo build --release
```

### How to run

```bash
everscale-states-rpc run --config config.yaml --global-config ton-global.config.json
```

### Usage example

```bash
curl -X 'GET' \
  -H 'accept: application/json' \
  http://127.0.0.1:10000/account/-1:503a4121eb0c77f4248a1906397890b315699b82afc14697516f5f0b584376ad
```

Output:
```json
{
  "account": "te6ccgECEwEAAngAAnPP9QOkEh6wx39CSKGQY5eJCzFWmbgq/BRpdRb18LWEN2rSVohtAxKtL/AAAA3Q5kp9EZT7RSx1K9NAAgEAWde+CNQ7SFf5T97Vwloudc3ijnS4yMbMA7Qhz8yijR5iAAABgB6QZ1CxKtL/QAQkiu1TIOMDIMD/4wIgwP7jAvILEAQDEgKc7UTQ10nDAfhmIds80wABjhKBAgDXGCD5AVj4QiD4ZfkQ8qje0z8B+EMhufK0IPgjgQPoqIIIG3dAoLnytPhj0x8B+CO88rnTHwHbPPI8CAUDSu1E0NdJwwH4ZiLQ1wsDqTgA3CHHAOMCIdcNH/K8IeMDAds88jwPDwUEUCCCEB+k2i264wIgghA3kP42uuMCIIIQaLVfP7rjAiCCEH03KaK64wILCQcGAVAw0ds8+EohjhyNBHAAAAAAAAAAAAAAAAA/TcpooMjOyx/JcPsA3vIADgI+MPhCbuMA+Ebyc9H4RSBukjBw3vhCuvLgZPgA2zzyAAgMAUTtRNDXScIBio4XcO1E0PQFcPhqgED0DvK91wv/+GJw+GPiDgNeMPhG8uBM+EJu4wD6QZXU0dD6QN/XDX+V1NHQ03/f1wwAldTR0NIA39HbPOMA8gAOCgwATPhFIG6SMHDe+EK68uBk+AASyM+FgMoAc89AzgH6AoBrz0DJcPsAAygw+Eby4Ez4Qm7jANMf0ds82zzyAA4NDAAk+Er4Q/hCyMv/yz/Pg8sfye1UADD4Srr4RSBukjBw3vhCurDy4GT4APgj+GoAJu1E0NP/0z/TADHTH9H4avhj+GIACvhG8uBMAgr0pCD0oRIRABRzb2wgMC41MC4wAAA=",
  "lastTransactionId": {
    "lt": "3478551000003",
    "hash": "cd3493c4c61cac77bf769db01e39209e9f5c5b028fa487f0722befcc30b559cb"
  }
}
```

### Example config

```yaml
api_settings:
  listen_address: "0.0.0.0:10000"

node_config:
  # Root directory for node DB. Default: "./db"
  db_path: "./db"

  # UDP port, used for ADNL node. Default: 30303
  adnl_port: 30000

  # Path to temporary ADNL keys.
  # NOTE: Will be generated if it was not there.
  # Default: "./adnl-keys.json"
  temp_keys_path: "./adnl-keys.json"

  # Archives map queue. Default: 16
  parallel_archive_downloads: 32

# log4rs settings.
# See https://docs.rs/log4rs/1.0.0/log4rs/ for more details
logger_settings:
  appenders:
    stdout:
      kind: console
      encoder:
        pattern: "{h({l})} {M} = {m} {n}"
  root:
    level: warn
    appenders:
      - stdout
  loggers:
    ton_indexer:
      level: info
      appenders:
        - stdout
      additive: false
    ton_kafka_producer:
      level: info
      appenders:
        - stdout
      additive: false
```
