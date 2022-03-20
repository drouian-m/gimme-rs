use anyhow::{Error, Result};
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use simple_error::SimpleError;
use std::error;
use std::str;

pub struct StorageManager {
  bucket: Bucket,
}

impl StorageManager {
  pub fn new(
    bucket_name: &str,
    access_key: &str,
    secret_key: &str,
    region: String,
    endpoint: String,
  ) -> StorageManager {
    let credentials = Self::create_credentials(access_key, secret_key).unwrap();
    let bucket = Self::create_bucket(bucket_name, region, endpoint, credentials).unwrap();

    StorageManager { bucket }
  }

  pub async fn get_object(&self, object_name: String) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let (data, code) = self.bucket.get_object(object_name).await?;
    // println!("{}", code);
    if code != 200 {
      bail!("Package not found");
    }
    Ok(data)
  }

  pub async fn get_object_as_stream(&self, object_name: String) {
    println!("Not implemented.");
  }

  pub async fn create_object(&self) {
    println!("Not implemented.");
  }

  fn create_credentials(access_key: &str, secret_key: &str) -> Result<Credentials> {
    let credentials: Credentials =
      Credentials::new(Some(access_key), Some(secret_key), None, None, None)?;
    Ok(credentials)
  }

  fn create_bucket(
    name: &str,
    region: String,
    endpoint: String, // use Option<endpoint> in the future to manage aws case
    credentials: Credentials,
  ) -> Result<Bucket> {
    let bucket = Bucket::new(
      name,
      Region::Custom {
        region: region,
        endpoint: endpoint,
      },
      credentials,
    )?;
    Ok(bucket)
  }
}
