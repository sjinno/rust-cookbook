use form_urlencoded::{byte_serialize, parse, Serializer};

const BASE_URL: &str = "https://api.exchangeratesapi.io/latest?"; // latest?base=USD

pub fn encode_form() {
    let parameters = Serializer::new(String::new())
        .append_pair("base", "USD")
        .finish();
    let url = format!("{}{}", BASE_URL, parameters);
    println!("{}", url);

    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}
