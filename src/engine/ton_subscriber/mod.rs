use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;
use serde::Serialize;
use tiny_adnl::utils::*;
use ton_block::HashmapAugType;
use ton_indexer::utils::*;
use ton_indexer::*;

pub struct TonSubscriber {
    masterchain_accounts_cache: RwLock<ton_block::ShardAccounts>,
    shard_accounts_cache: RwLock<FxHashMap<ton_block::ShardIdent, ton_block::ShardAccounts>>,
}

impl TonSubscriber {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            masterchain_accounts_cache: Default::default(),
            shard_accounts_cache: Default::default(),
        })
    }

    pub fn get_contract_state(
        &self,
        account: &ton_block::MsgAddressInt,
    ) -> Result<Option<ExistingContract>> {
        let is_masterchain = account.is_masterchain();
        let account = account.address().get_bytestring_on_stack(0);
        let account = ton_types::UInt256::from_slice(account.as_slice());

        if is_masterchain {
            let cache = self.masterchain_accounts_cache.read();
            ExistingContract::from_shard_account_opt(cache.get(&account)?)
        } else {
            let cache = self.shard_accounts_cache.read();
            for (shard_ident, shard_accounts) in cache.iter() {
                if !contains_account(shard_ident, &account) {
                    continue;
                }
                return ExistingContract::from_shard_account_opt(shard_accounts.get(&account)?);
            }
            Ok(None)
        }
    }

    async fn handle_block(
        &self,
        block_id: &ton_block::BlockIdExt,
        block: &ton_block::Block,
        shard_state: &ton_block::ShardStateUnsplit,
    ) -> Result<()> {
        let shard_accounts = shard_state.read_accounts()?;

        if block_id.is_masterchain() {
            *self.masterchain_accounts_cache.write() = shard_accounts;
        } else {
            let block_info = &block.read_info()?;

            let mut cache = self.shard_accounts_cache.write();

            cache.insert(*block_info.shard(), shard_accounts);
            if block_info.after_merge() || block_info.after_split() {
                log::warn!("Clearing shard states cache after shards merge/split");

                let block_ids = block_info.read_prev_ids()?;

                match block_ids.len() {
                    // Block after split
                    //       |
                    //       *  - block A
                    //      / \
                    //     *   *  - blocks B', B"
                    1 => {
                        // Find all split shards for the block A
                        let (left, right) = block_ids[0].shard_id.split()?;

                        // Remove parent shard of the block A
                        if cache.contains_key(&left) && cache.contains_key(&right) {
                            cache.remove(&block_ids[0].shard_id);
                        }
                    }

                    // Block after merge
                    //     *   *  - blocks A', A"
                    //      \ /
                    //       *  - block B
                    //       |
                    2 => {
                        // Find and remove all parent shards
                        for block_id in block_info.read_prev_ids()? {
                            cache.remove(&block_id.shard_id);
                        }
                    }
                    _ => {}
                }
            }
        };

        Ok(())
    }
}

#[async_trait::async_trait]
impl ton_indexer::Subscriber for TonSubscriber {
    async fn process_block(&self, ctx: ProcessBlockContext<'_>) -> Result<()> {
        if let Some(shard_state) = ctx.shard_state() {
            self.handle_block(ctx.id(), ctx.block(), shard_state).await
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExistingContract {
    pub account: String,
    pub last_transaction_id: LastTransactionId,
}

impl ExistingContract {
    fn from_shard_account_opt(
        shard_account: Option<ton_block::ShardAccount>,
    ) -> Result<Option<Self>> {
        shard_account.map(Self::from_shard_account).transpose()
    }

    fn from_shard_account(shard_account: ton_block::ShardAccount) -> Result<Self> {
        Ok(Self {
            account: base64::encode(&ton_types::serialize_toc(&shard_account.account_cell())?),
            last_transaction_id: LastTransactionId {
                lt: shard_account.last_trans_lt().to_string(),
                hash: shard_account.last_trans_hash().as_hex_string(),
            },
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LastTransactionId {
    pub lt: String,
    pub hash: String,
}

fn contains_account(shard: &ton_block::ShardIdent, account: &ton_types::UInt256) -> bool {
    let shard_prefix = shard.shard_prefix_with_tag();
    if shard_prefix == ton_block::SHARD_FULL {
        true
    } else {
        let len = shard.prefix_len();
        let account_prefix = account_prefix(account, len as usize) >> (64 - len);
        let shard_prefix = shard_prefix >> (64 - len);
        account_prefix == shard_prefix
    }
}

fn account_prefix(account: &ton_types::UInt256, len: usize) -> u64 {
    debug_assert!(len <= 64);

    let account = account.as_slice();

    let mut value: u64 = 0;

    let bytes = len / 8;
    for (i, byte) in account.iter().enumerate().take(bytes) {
        value |= (*byte as u64) << (8 * (7 - i));
    }

    let remainder = len % 8;
    if remainder > 0 {
        let r = account[bytes] >> (8 - remainder);
        value |= (r as u64) << (8 * (7 - bytes) + 8 - remainder);
    }

    value
}
