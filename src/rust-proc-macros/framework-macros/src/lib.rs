use proc_macro::TokenStream;

// function-like macro
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

/*
    Daha işe yarar bir function-like macro
    Şöyle kullanabiliriz örneğin;

    invoice_code!("VEH","BG",1001);

    Tabii gerçek hayat senaryolarında tercih etmeyiz zira parse işlemleri hataya çok müsaittir.
    Bu şekilde string parse yerine syn, quote gibi endüstri standardı haline gelmiş token bazlı çalışan
    crate'ler kullanmamız gerekir. Bu ve önceki örnek TokenStream'in doğasını anlamak için
    yeterlidir.
*/

#[proc_macro]
pub fn invoice_code(input: TokenStream) -> TokenStream {
    let raw = input.to_string(); // gelen token stream'i alalım
    // sonra virgüle göre parçalarını alalım
    let parts = raw.split(',').map(|p| p.trim()).collect::<Vec<&str>>();

    // bir kontrol yapalım
    if parts.len() != 3 {
        return r#"compile_error!("wrong number of arguments!")"#.parse().unwrap();
    }

    let module_code = parts[0];
    let invoice_type = parts[1];
    /*
       Macrolar derleme zamanı için kod üretirler ama çalışma zamanında da harici kütüphaneleri
       ele alabilirler. Mesela üretilen fatura numarasına chrono crate üzerinden güncel
       tarihe ait yil, ay, gün bilgisini ekleyebiliriz.
    */
    let build_date = chrono::Local::now().format("%Y%m%d").to_string();
    let id = parts[2];

    let output = format!(
        r#"
            format!("INV-{{}}-{{}}-{{}}-{{}}",{module_code},{invoice_type},{build_date},{id})
        "#
    );

    output.parse().unwrap()
}

/*
    Basit derive-macro örneği.
    Bu sefer bir veri yapısına #derive ile uygulanabilen bir macro var.
    Bu macro bir struct için uyguladığımızda ona otomatik olarak entity_name
    isimli bir metot dahil ediyoruz.

    Basit kullanımlar için ideal ama struct'ın şuna benzer yazılması lazım.
    struct Game {

    Başa pub gelirse, generic T kullanılırsa, tuple struct ele alınırsa vs
    boşluk karakterine göre ayrıştırıp struct adını bulma yine çuvallar.
    Dolayısıyla token bazlı ayrıştırma yaparak hareket etmek gerekir ki bu bizi tekrardan
    sync, quote kullanımına getirir.
*/
#[proc_macro_derive(TableName)]
pub fn derive_table_name(input: TokenStream) -> TokenStream {
    let raw = input.to_string();
    let struct_name = raw
        .split_whitespace()
        .skip_while(|token| *token != "struct")
        .nth(1)
        .expect("Expected a struct name");

    let generated = format!(
        r#"
            impl {struct_name} {{
                pub fn table_name() -> &'static str {{
                    "tbl_{struct_name}"
                }}
            }}
        "#
    );

    generated.parse().unwrap()
}
