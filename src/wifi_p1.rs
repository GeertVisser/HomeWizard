use reqwest:: { Result  };
use json::JsonValue;

static BASE_URL: &str = "http://192.168.3.202/api";

pub fn get_basic_info() -> Result<JsonValue>  { 
    return call_api(BASE_URL);
}

pub fn gather_data() -> Result<JsonValue>  {
    let request_url = format!("{}{}", BASE_URL, "/v1/data");
    return call_api(&request_url);
}

pub fn call_api(request_url: &str) -> Result<JsonValue>  {

    let result = reqwest::blocking::get(request_url)?.text()?;
    let some_json = json::parse(&result); 

    Ok(some_json.unwrap())
}
