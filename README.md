## Everscale states RPC

### How to build

```bash
RUSTFLAGS='-C target_cpu=native' cargo build --release
```

### How to run

```bash
everscale-states-rpc run --config config.yaml --global-config ton-global.config.json
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
