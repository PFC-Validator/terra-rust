use dotenv::dotenv;
use secp256k1::Secp256k1;

use terra_rust_cli::cli_helpers;

async fn run() -> anyhow::Result<()> {
    let cli = cli_helpers::gen_cli("pubkey", "pubkey");
    let matches = cli.get_matches();
    let secp = Secp256k1::new();
    let private_key = cli_helpers::get_private_key(&secp, &matches)?;

    let msg = "mary had a little lamb. I ate, coz lamb is delicious";
    // let signature = from_key.sign(&secp, &cli.message)?;
    let signature = private_key.sign(&secp, &msg)?;

    println!("PubKeySig={}", signature.pub_key.value);

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    if let Err(ref err) = run().await {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
