use reqwest:: { Result  };
use json::JsonValue;

static BASE_URL: &str = "https://monitoringapi.solaredge.com/";
static API_KEY: &str = your api key;

pub fn call_api(endpoint: &str) -> Result<JsonValue>  {

    println!("1");

    let request_url = format!("{}{}?api_key={}", BASE_URL, endpoint, API_KEY);
    let result = reqwest::blocking::get(&request_url)?.text()?;
    
    let some_json = json::parse(&result);

//    println!("all json data: {:#}", move | some_json.unwrap());
 
    Ok(some_json.unwrap())
}

pub fn setup() -> Result <i32> {
    let se_json = match call_api("sites/list") {
        Ok(se_json) => se_json,
        Err(e) => return Err(e)
      };

      let site_id: i32 = se_json["sites"]["site"][0]["id"].as_i32().unwrap();
      Ok(site_id)
}

