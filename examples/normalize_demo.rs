use rho_circles::cas::Cas;
use rho_circles::chips::normalize;
use serde_json::json;
use std::fs;

fn main() {
    println!("=== Rho Circles Example: Normalize and Store ===\n");

    // Initialize CAS
    let cas = Cas::new();

    // Example 1: Normalize a simple object
    println!("Example 1: Normalize with key sorting");
    let input1 = json!({"z": 3, "a": 1, "m": 2});
    match normalize(input1.clone()) {
        Ok(output) => {
            println!("  Input:  {}", input1);
            let decoded =
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &output.bytes)
                    .unwrap();
            println!("  Output: {}", String::from_utf8(decoded).unwrap());
            println!("  CID:    {}", output.cid);

            // Store in CAS
            let stored_cid = cas.put(output.bytes.into_bytes()).unwrap();
            println!("  Stored: {}\n", stored_cid);
        }
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 2: Null removal
    println!("Example 2: Null removal");
    let input2 = json!({
        "name": "Alice",
        "age": null,
        "city": "Boston",
        "country": null
    });
    match normalize(input2.clone()) {
        Ok(output) => {
            println!("  Input:  {}", input2);
            let decoded =
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &output.bytes)
                    .unwrap();
            println!("  Output: {}", String::from_utf8(decoded).unwrap());
            println!("  CID:    {}\n", output.cid);
        }
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 3: Nested objects
    println!("Example 3: Nested object normalization");
    let input3 = json!({
        "user": {
            "z_last": "Smith",
            "a_first": "John"
        },
        "metadata": {
            "created": 1234567890,
            "tags": ["rust", "blockchain"]
        }
    });
    match normalize(input3.clone()) {
        Ok(output) => {
            println!("  Input:  {}", input3);
            let decoded =
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &output.bytes)
                    .unwrap();
            println!("  Output: {}", String::from_utf8(decoded).unwrap());
            println!("  CID:    {}\n", output.cid);
        }
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 4: Determinism test
    println!("Example 4: Determinism - same content, different order");
    let input4a = json!({"b": 2, "a": 1, "c": 3});
    let input4b = json!({"c": 3, "a": 1, "b": 2});

    match (normalize(input4a.clone()), normalize(input4b.clone())) {
        (Ok(out_a), Ok(out_b)) => {
            println!("  Input A: {}", input4a);
            println!("  Input B: {}", input4b);
            println!("  CID A:   {}", out_a.cid);
            println!("  CID B:   {}", out_b.cid);
            println!("  Match:   {}\n", out_a.cid == out_b.cid);
        }
        _ => println!("  Error in normalization\n"),
    }

    // Example 5: Float rejection
    println!("Example 5: Float rejection (expected error)");
    let input5 = json!({"value": 3.14});
    match normalize(input5.clone()) {
        Ok(_) => println!("  Unexpected success!\n"),
        Err(e) => println!("  Input:  {}\n  Error:  {} ✓\n", input5, e),
    }

    // Load chip spec example
    println!("Example 6: Load chip spec from file");
    if let Ok(spec_str) = fs::read_to_string("examples/chip_specs/rho.normalize.json") {
        if let Ok(spec_json) = serde_json::from_str::<serde_json::Value>(&spec_str) {
            match normalize(spec_json.clone()) {
                Ok(output) => {
                    println!("  Chip:   {}", spec_json["chip"]);
                    println!("  Type:   {}", spec_json["type"]);
                    println!("  CID:    {}", output.cid);
                }
                Err(e) => println!("  Error: {}", e),
            }
        }
    } else {
        println!("  (chip spec file not found, skipping)");
    }

    println!("\n=== Done ===");
}
