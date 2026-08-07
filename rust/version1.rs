use std::env;
use reqwest::blocking::Client;
use serde::Deserialize;

// Version 1: Consume SOAP service directly
// Usage: cargo run --bin version1 -- <number>
// Example: cargo run --bin version1 -- 10

#[derive(Debug, Deserialize)]
struct SoapResponse {
    #[serde(rename = "NumberToWordsResult")]
    number_to_words_result: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let number = args.get(1).unwrap_or(&"10".to_string()).clone();
    
    let client = Client::new();
    let url = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";
    
    let soap_body = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
        <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
            <soap:Body>
                <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
                    <ubiNum>{}</ubiNum>
                </NumberToWords>
            </soap:Body>
        </soap:Envelope>"#,
        number
    );
    
    let response = client
        .post(url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "http://www.dataaccess.com/webservicesserver/NumberToWords")
        .body(soap_body)
        .send();
    
    match response {
        Ok(resp) => {
            let body = resp.text().unwrap_or_default();
            // Parse XML response (simplified - in production use proper XML parser)
            if let Some(start) = body.find("<NumberToWordsResult>") {
                if let Some(end) = body.find("</NumberToWordsResult>") {
                    let result = &body[start + 21..end];
                    println!("{}", result);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
