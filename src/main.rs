use rho_circles::chips::normalize;
use serde_json::json;

fn main() {
    println!("Rho Circles - Chip Registry System");
    println!("===================================\n");

    // Example: Test rho.normalize
    let test_input = json!({"b": 2, "a": 1, "null_field": null});
    println!("Testing rho.normalize with input: {}", test_input);
    
    match normalize(test_input) {
        Ok(output) => {
            println!("✓ Normalization successful!");
            println!("  CID: {}", output.cid);
            println!("  Bytes (base64): {}...", &output.bytes[..20.min(output.bytes.len())]);
        }
        Err(e) => {
            println!("✗ Normalization failed: {}", e);
        }
    }
}

