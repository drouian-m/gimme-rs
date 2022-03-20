use anyhow::Result;
use clap::Parser;
use std::env;
use std::str;
mod storage_manager;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
  #[clap(short, long)]
  filename: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();

  let bucket_name =
    env::var("GIMME_BUCKET").expect("Please set $GIMME_BUCKET environnement variable");
  let bucket_url =
    env::var("GIMME_BUCKET_URL").expect("Please set $GIMME_BUCKET_URL environnement variable");
  let bucket_region = env::var("GIMME_BUCKET_REGION")
    .expect("Please set $GIMME_BUCKET_REGION environnement variable");
  let access_key =
    env::var("GIMME_ACCESS_KEY").expect("Please set $GIMME_ACCESS_KEY environnement variable");
  let secret_key =
    env::var("GIMME_SECRET_KEY").expect("Please set $GIMME_SECRET_KEY environnement variable");

  let store_manager = storage_manager::StorageManager::new(
    &bucket_name[..],
    &access_key[..],
    &secret_key[..],
    bucket_region,
    bucket_url,
  );

  let data = store_manager.get_object(args.filename).await?;

  let string = str::from_utf8(&data).unwrap();
  println!("File content : {}", string);
  Ok(())
}
