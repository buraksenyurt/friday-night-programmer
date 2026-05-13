use framework_macros::{greetings, invoice_code};
fn main() {
    greetings!("Input..."); // function-like macro
    let invoice = invoice_code!("VEH","BG",1001);
    println!("Created invoice number: {}", invoice);
}
