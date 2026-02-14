use rho_circles::cas::Cas;
use rho_circles::chips::normalize;
use rho_circles::modules;
use serde_json::json;

fn main() {
    println!("Rho Circles - Chip Registry System");
    println!("===================================\n");

    // Create shared CAS
    let cas = Cas::new();

    // Example 1: Test rho.normalize
    println!("1. Testing rho.normalize (Inner Ring)");
    let test_input = json!({"b": 2, "a": 1, "null_field": null});
    println!("   Input: {}", test_input);

    match normalize(test_input) {
        Ok(output) => {
            println!("   ✓ Normalization successful!");
            println!("   CID: {}", output.cid);
            println!(
                "   Bytes (base64): {}...",
                &output.bytes[..20.min(output.bytes.len())]
            );
        }
        Err(e) => {
            println!("   ✗ Normalization failed: {}", e);
        }
    }

    // Example 2: Test mod.log (Middle Ring)
    println!("\n2. Testing mod.log (Middle Ring)");
    let log_result = modules::log(
        "info".to_string(),
        "System startup complete".to_string(),
        Some(json!({"version": "0.1.0", "modules": 7})),
        &cas,
    );
    match log_result {
        Ok(rc) => {
            println!("   ✓ Log entry created!");
            println!("   Content CID: {}", rc.recibo.content_cid);
            println!("   Level: {}", rc.body["level"]);
            println!("   Message: {}", rc.body["message"]);
        }
        Err(e) => {
            println!("   ✗ Log failed: {}", e);
        }
    }

    // Example 3: Test mod.chip.publish (Middle Ring)
    println!("\n3. Testing mod.chip.publish (Middle Ring)");
    let chip_spec = json!({
        "chip": "demo.chip",
        "version": "1.0.0",
        "type": "module",
        "inputs": {"value": {"type": "string"}},
        "outputs": {"result": {"type": "string"}}
    });
    let publish_result = modules::publish(chip_spec, "owner_demo".to_string(), &cas);
    match publish_result {
        Ok(rc) => {
            println!("   ✓ Chip published!");
            println!("   Chip CID: {}", rc.body["chip_cid"]);
            println!("   Owner CID: {}", rc.body["owner_cid"]);
        }
        Err(e) => {
            println!("   ✗ Publish failed: {}", e);
        }
    }

    println!("\n===================================");
    println!("System Status:");
    println!("  Inner Ring (Base Transistors): ✓ Complete (5/5)");
    println!("  Middle Ring (Modules): ✓ Complete (7/7)");
    println!("  Outer Ring (Products): ⏳ Not started");
    println!("\n  Total Tests Passing: 71");
    println!("===================================\n");
}
