use aws_config::Region;
use dotenv::dotenv;

#[derive(Clone, Debug)]
pub struct AwsConfig {
    region: Region,
    bucket_name: String,
    silverlining_url: String,
    silverlining_api_key: String,
}

impl AwsConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let aws_region = std::env::var("AWS_REGION").expect("AWS_REGION must be set.");
        let aws_bucket_name =
            std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME must be set.");
        let silverlining_url =
            std::env::var("SILVERLINING_URL").expect("SILVERLINING_URL must be set.");
        let silverlining_api_key =
            std::env::var("SILVERLINING_API_KEY").expect("SILVERLINING_API_KEY must be set.");

        Self {
            region: Region::new(aws_region),
            bucket_name: aws_bucket_name,
            silverlining_url,
            silverlining_api_key,
        }
    }

    pub fn region(&self) -> Region {
        self.region.clone()
    }

    pub fn bucket_name(&self) -> String {
        self.bucket_name.clone()
    }

    pub fn silverlining_url(&self) -> String {
        self.silverlining_url.clone()
    }

    pub fn silverlining_api_key(&self) -> String {
        self.silverlining_api_key.clone()
    }
}
