use bitmex::rest::BitMEXRest;
use bitmex::rest::{GetExecutionRequest, GetExecutionTradeHistoryRequest};
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_execution() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let rt = Runtime::new()?;

    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetExecutionRequest {
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn get_execution_history() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let rt = Runtime::new()?;

    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetExecutionTradeHistoryRequest {
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}
