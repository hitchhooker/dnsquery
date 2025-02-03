use hickory_resolver::{AsyncResolver, config::{ResolverConfig, ResolverOpts}};
use hickory_resolver::proto::rr::{RecordType, RData};

pub async fn verify(domain: &str, txt: &str) -> bool {
    let resolver = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    match resolver.lookup(domain, RecordType::TXT).await {
        Ok(response) => {
            let records: Vec<String> = response
                .iter()
                .filter_map(|record| match record {
                    RData::TXT(txt_data) => Some(txt_data),
                    _ => None,
                })
                .flat_map(|txt_data| txt_data.iter())
                .map(|bytes| String::from_utf8_lossy(bytes).to_string())
                .collect();

            for record in &records {
                println!("Record: {}", record);
            }

            println!("Challenge: {}", txt);

            if records.contains(&txt.to_string()) {
                return true;
            }
        }
        Err(err) => {
            println!("Lookup failed for {}: {}", domain, err);
        }
    }

    false
}
