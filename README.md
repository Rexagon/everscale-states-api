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
  "account": "te6ccgECEwEAAncAAnKf6gdIJD1hjv6EkUMgxy8SFmKtM3BV+CjS6i3r4WsIbtWkCRDaBiSJsXAAABlPTl+eIyo954FdRyYCAQBZ174I1DtIV/lP3tXCWi51zeKOdLjIxswDtCHPzKKNHmIAAAF/653K+bEkTYvABCSK7VMg4wMgwP/jAiDA/uMC8gsQBAMSApztRNDXScMB+GYh2zzTAAGOEoECANcYIPkBWPhCIPhl+RDyqN7TPwH4QyG58rQg+COBA+iogggbd0CgufK0+GPTHwH4I7zyudMfAds88jwIBQNK7UTQ10nDAfhmItDXCwOpOADcIccA4wIh1w0f8rwh4wMB2zzyPA8PBQRQIIIQH6TaLbrjAiCCEDeQ/ja64wIgghBotV8/uuMCIIIQfTcporrjAgsJBwYBUDDR2zz4SiGOHI0EcAAAAAAAAAAAAAAAAD9NymigyM7LH8lw+wDe8gAOAj4w+EJu4wD4RvJz0fhFIG6SMHDe+EK68uBk+ADbPPIACAwBRO1E0NdJwgGKjhdw7UTQ9AVw+GqAQPQO8r3XC//4YnD4Y+IOA14w+Eby4Ez4Qm7jAPpBldTR0PpA39cNf5XU0dDTf9/XDACV1NHQ0gDf0ds84wDyAA4KDABM+EUgbpIwcN74Qrry4GT4ABLIz4WAygBzz0DOAfoCgGvPQMlw+wADKDD4RvLgTPhCbuMA0x/R2zzbPPIADg0MACT4SvhD+ELIy//LP8+Dyx/J7VQAMPhKuvhFIG6SMHDe+EK6sPLgZPgA+CP4agAm7UTQ0//TP9MAMdMf0fhq+GP4YgAK+Eby4EwCCvSkIPShEhEAFHNvbCAwLjUwLjAAAA==",
  "lastTransactionId": {
    "lt": "3478551000003",
    "hash": "cd3493c4c61cac77bf769db01e39209e9f5c5b028fa487f0722befcc30b559cb"
  }
}
```

### Example config

```yaml
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
