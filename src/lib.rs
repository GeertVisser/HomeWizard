use reqwest:: { Result  };
use std::time::Duration;
use std::thread::sleep;

mod wifi_p1;
mod solaredge;
mod display_layer;


//Intial Setup, gather variables.
pub fn setup() -> Result <()> {
  let p1_json = match wifi_p1::get_basic_info() {
    Ok(p1_json) => p1_json,
    Err(e) => return Err(e)
  };

  //https://homewizard-energy-api.readthedocs.io/
  println!("We've called HomeWizard P1 API {:#}!!", p1_json);

  let site_id = match solaredge::setup(){
    Ok(site_id) => site_id,
    Err(e) => return Err(e)
  };

  println!("SolarEdge API site id = {}!!", site_id);
  Ok(())
}

//collect data on interval.
pub fn gather_data() -> Result <()> {
  let p1_json = match wifi_p1::gather_data() {
    Ok(p1_json) => p1_json,
    Err(e) => return Err(e)
  };  

  // struct P1DataTotal {
  //   total_power_import_t1_kwh: f64,
  //   total_power_import_t2_kwh: f64,
  //   total_power_export_t1_kwh: f64,
  //   total_power_export_t2_kwh: f64,
  //   total_gas_m3: f64
  // }
  // let p1_total = P1DataTotal { 
  //   total_power_import_t1_kwh: p1_json["total_power_import_t1_kwh"].as_f64().unwrap(),
  //   total_power_import_t2_kwh: p1_json["total_power_import_t2_kwh"].as_f64().unwrap(),
  //   total_power_export_t1_kwh: p1_json["total_power_export_t1_kwh"].as_f64().unwrap(),
  //   total_power_export_t2_kwh: p1_json["total_power_export_t2_kwh"].as_f64().unwrap(),
  //   total_gas_m3: p1_json["total_gas_m3"].as_f64().unwrap()
  // };



  struct P1DataTotal {
      total_power_import_t1_kwh: Vec<f64>,
      total_power_import_t2_kwh: Vec<f64>,
      total_power_export_t1_kwh: Vec<f64>,
      total_power_export_t2_kwh: Vec<f64>,
      total_gas_m3: Vec<f64>
  }
  struct P1DataActual {
    active_power_w: Vec<i32>,
    active_power_l1_w: Vec<i32>,
    active_power_l2_w: Vec<i32>,
    active_power_l3_w: Vec<i32>,
  }
  
  let mut p1_total = P1DataTotal { 
    total_power_import_t1_kwh: vec![], 
    total_power_import_t2_kwh: vec![], 
    total_power_export_t1_kwh: vec![], 
    total_power_export_t2_kwh: vec![], 
    total_gas_m3: vec![]
  };
  let mut p1_actual = P1DataActual { 
    active_power_w: vec![], 
    active_power_l1_w: vec![], 
    active_power_l2_w: vec![], 
    active_power_l3_w: vec![]
  };
  

  for n in 0..9 {
    let p1_json = match wifi_p1::gather_data() {
      Ok(p1_json) => p1_json,
      Err(e) => return Err(e)
    };

    p1_total.total_power_import_t1_kwh.push(p1_json["total_power_import_t1_kwh"].as_f64().unwrap());
    p1_total.total_power_import_t2_kwh.push(p1_json["total_power_import_t2_kwh"].as_f64().unwrap());
    p1_total.total_power_export_t1_kwh.push(p1_json["total_power_export_t1_kwh"].as_f64().unwrap());
    p1_total.total_power_export_t2_kwh.push(p1_json["total_power_export_t2_kwh"].as_f64().unwrap());
    p1_total.total_gas_m3.push(p1_json["total_gas_m3"].as_f64().unwrap());

    p1_actual.active_power_w.push(p1_json["active_power_w"].as_i32().unwrap());
    p1_actual.active_power_l1_w.push(p1_json["active_power_l1_w"].as_i32().unwrap());
    p1_actual.active_power_l2_w.push(p1_json["active_power_l2_w"].as_i32().unwrap());
    p1_actual.active_power_l3_w.push(p1_json["active_power_l3_w"].as_i32().unwrap());

    println!("iteration {}", n);
    sleep(Duration::from_secs(1));
  }

  //collect data on interval.
  
    
  display_layer::plot_graph(
    p1_actual.active_power_w,
    p1_actual.active_power_l1_w,
    p1_actual.active_power_l2_w,
    p1_actual.active_power_l3_w
  );

  

    //   total_power_import_t2_kwh: p1_json["total_power_import_t2_kwh"].as_f64().unwrap(),
    //   total_power_export_t1_kwh: p1_json["total_power_export_t1_kwh"].as_f64().unwrap(),
    //   total_power_export_t2_kwh: p1_json["total_power_export_t2_kwh"].as_f64().unwrap(),
    //   total_gas_m3: p1_json["total_gas_m3"].as_f64().unwrap()


    //   p1_totaltotal_power_import_t1_kwh: p1_json["total_power_import_t1_kwh"].as_f64().unwrap(),
    //   total_power_import_t2_kwh: p1_json["total_power_import_t2_kwh"].as_f64().unwrap(),
    //   total_power_export_t1_kwh: p1_json["total_power_export_t1_kwh"].as_f64().unwrap(),
    //   total_power_export_t2_kwh: p1_json["total_power_export_t2_kwh"].as_f64().unwrap(),
    //   total_gas_m3: p1_json["total_gas_m3"].as_f64().unwrap()


  // Scatter plots expect a list of pairs
  let data1 = vec![
    (-3.0, 2.3),
    (-1.6, 5.3),
    (0.3, 0.7),
    (4.3, -1.4),
    (6.4, 4.3),
    (8.5, 3.7),
  ];

  let data2 = vec![(-1.4, 2.5), (7.2, -0.3)];

  display_layer::scatter_test(data1, data2);

  Ok(())
}





pub async fn hello_world() {
    println!("hello, world!");
}

use std::collections::HashMap;

pub fn get_current_date() -> Result<String> {
    let url = "https://postman-echo.com/time/object";
    let result = reqwest::blocking::get(url);
  
    let response = match result {
      Ok(res) => res,
      Err(err) => return Err(err),
    };
  
    let body = response.json::<HashMap<String, i32>>();
  
    let json = match body {
      Ok(json) => json,
      Err(err) => return Err(err),
    };
  
    let date = json["years"].to_string();
  
    Ok(date)
  }