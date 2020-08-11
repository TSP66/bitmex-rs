use bitmex::rest::BitMEXRest;
use bitmex::rest::{GetAnnouncementRequest, GetAnnouncementUrgentRequest};
use failure::Fallible;
use log::debug;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_announcement() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let bm = BitMEXRest::new();
    let fut = bm.request(GetAnnouncementRequest {
        ..Default::default()
    });

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}

#[test]
fn get_announcement_urgent() -> Fallible<()> {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let bm = BitMEXRest::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.request(GetAnnouncementUrgentRequest {});

    debug!("{:?}", rt.block_on(fut)?);
    Ok(())
}
