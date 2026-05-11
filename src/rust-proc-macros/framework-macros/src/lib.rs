use proc_macro::TokenStream;

#[proc_macro]
pub fn greetings(input: TokenStream) -> TokenStream {
    dbg!(input);

    r#"
     {
        println!("Greetings from proc macro");
     }
     "#
    .parse()
    .unwrap()
}
