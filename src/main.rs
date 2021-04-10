mod errors;
use crate::errors::Result;

use terra_rust_api::core_types::Coin;
use terra_rust_api::Terra;

async fn run() -> Result<bool> {
    let t = Terra::lcd_client("https://tequila-lcd.terra.dev", "tequila-0004").await?;
    let list = t.staking().validators().await?;
    if !list.result.is_empty() {
        let v1 = list.result.get(0).unwrap();
        println!("{:#?}", v1);
    }
    let v = t
        .staking()
        .validator("terravaloper1usws7c2c6cs7nuc8vma9qzaky5pkgvm2ujy8ny")
        .await?;
    println!("{:#?}", v);
    let coin = Coin::create("ukrw", 12000);
    let sw = t.market().swap(&coin, "uusd").await?;
    println!("{:#?}", sw);
    Ok(true)
}
#[tokio::main]
async fn main() {
    if let Err(ref e) = run().await {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
