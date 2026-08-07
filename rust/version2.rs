use std::env;
use reqwest::blocking::Client;

// Version 2: Consume SOAP service + translate to Spanish
// Usage: cargo run --bin version2 -- <number>
// Example: cargo run --bin version2 -- 10

fn translate_to_spanish(text: &str) -> String {
    // Simple translation mapping for common number words
    // In a real implementation, you would use a translation API or library
    match text {
        "zero" => "cero".to_string(),
        "one" => "uno".to_string(),
        "two" => "dos".to_string(),
        "three" => "tres".to_string(),
        "four" => "cuatro".to_string(),
        "five" => "cinco".to_string(),
        "six" => "seis".to_string(),
        "seven" => "siete".to_string(),
        "eight" => "ocho".to_string(),
        "nine" => "nueve".to_string(),
        "ten" => "diez".to_string(),
        _ => text.to_string(),
    }
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
                    let english_result = &body[start + 21..end];
                    let spanish_result = translate_to_spanish(english_result);
                    println!("{}", spanish_result);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
