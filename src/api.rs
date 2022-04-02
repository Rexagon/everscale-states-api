use std::convert::Infallible;
use std::str::FromStr;
use std::sync::Arc;

use warp::http::StatusCode;
use warp::{Filter, Reply};

use crate::config::ApiConfig;
use crate::engine::TonSubscriber;

pub async fn serve(subscriber: Arc<TonSubscriber>, config: ApiConfig) {
    let state = Arc::new(State { subscriber });
    let state = warp::any().map(move || state.clone());

    let routes = warp::path!("account" / String)
        .and(state.clone())
        .and(warp::get())
        .and_then(get_account);

    warp::serve(routes).bind(config.listen_address).await;
}

async fn get_account(address: String, ctx: Arc<State>) -> Result<Box<dyn Reply>, Infallible> {
    let address = match ton_block::MsgAddressInt::from_str(&address) {
        Ok(address) => address,
        Err(e) => {
            return Ok(reply_error(
                StatusCode::BAD_REQUEST,
                e.context("Invalid address").to_string(),
            ))
        }
    };

    Ok(match ctx.subscriber.get_contract_state(&address) {
        Ok(contract) => Box::new(warp::reply::json(&contract)),
        Err(e) => reply_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.context("Failed to get shard account").to_string(),
        ),
    })
}

fn reply_error(status: StatusCode, message: impl AsRef<str>) -> Box<dyn Reply> {
    let message = serde_json::to_string(message.as_ref()).expect("Shouldn't fail");
    Box::new(warp::reply::with_status(
        format!("{{\"error\":{message}}}"),
        status,
    ))
}

struct State {
    subscriber: Arc<TonSubscriber>,
}
