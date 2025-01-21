use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn analyze_stats(json_data: &str) -> JsValue {
    let data: Vec<serde_json::Value> = serde_json::from_str(json_data).unwrap();

    let memory_usages: Vec<u64> = data
        .iter()
        .map(|entry| entry["memory_used"].as_u64().unwrap())
        .collect();
    let avg_memory_used = memory_usages.iter().sum::<u64>() / memory_usages.len() as u64;

    let cpu_usages: Vec<f32> = data
        .iter()
        .map(|entry| entry["cpu_usage"].as_f64().unwrap() as f32)
        .collect();
    let avg_cpu_usage = cpu_usages.iter().sum::<f32>() / cpu_usages.len() as f32;

    let result = serde_json::json!({
        "avg_cpu_usage": avg_cpu_usage,
        "avg_memory_used": avg_memory_used
    }).to_string();

    JsValue::from(&result)
}
