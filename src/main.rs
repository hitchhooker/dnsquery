use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let [_, domain, txt] = args.as_slice() else {
        eprintln!("usage: dnsquery <domain> <txt-record>");
        std::process::exit(1);
    };

    if dnsquery::verify(domain, txt).await {
        println!("verified");
    } else {
        eprintln!("failed");
        std::process::exit(1);
    }
}
