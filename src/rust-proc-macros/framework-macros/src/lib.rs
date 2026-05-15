use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Data, Expr, Fields, Token, Type};

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
    Şimdi invoice_code makrosunun daha güvenli bir versiyonunu yazalım.
    Bu makro girdilere göre bir fatura numarası formatı oluşturuyor. Ancak girdileri
    string olarak parse etmek yerine token bazlı parse işlemi yapıyoruz.

    Bunun en büyük avantajı pek takii string parse işlemi yapmıyor oluşumuz. Örneğin
    klasik invoice_code makrosunu aşağıdaki gibi çağırsak;

    invoice_code!("VEH", get_type("VIP,User"), 1000);

    şuna benzer bir derleme hatası alırız;

    error: wrong number of arguments!
 --> app\src\main.rs:9:5
  |
9 |     invoice_code!("VEH", get_type("VIP,User"), 1000);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
*/
struct InvoiceArgs {
    module_code: Expr,
    invoice_type: Expr,
    id: Expr,
}

impl Parse for InvoiceArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // İlk parça modül kodu onu doğrudan alıyoruz
        let module_code: Expr = input.parse()?;
        input.parse::<Token![,]>()?; // Bir virgül atlıyoruz
        let invoice_type: Expr = input.parse()?; // İkinci parça fatura tipi
        input.parse::<Token![,]>()?; // Bir virgül daha atlıyoruz
        let id: Expr = input.parse()?; // Son parça id

        Ok(InvoiceArgs {
            module_code,
            invoice_type,
            id,
        })
    }
}

#[proc_macro]
pub fn invoice_code_safe(input: TokenStream) -> TokenStream {
    // Öncelikle gelen TokenStream'i InvoiceArgs yapısına parse ediyoruz
    // Bunu yaparken de başka bir macro kullandığımıza dikkat edelim :)
    let args = parse_macro_input!(input as InvoiceArgs);

    let module = args.module_code;
    let inv_type = args.invoice_type;
    let id = args.id;

    // Burası zaten klasik fatura için tarih damgasını aldığımız yer
    let build_date = chrono::Local::now().format("%Y%m%d").to_string();

    // Çıktıyı da quote makrosu ile oluşturuyoruz. Burada #args.module_code gibi ifadelerle
    // parse ettiğimiz argümanlara erişebiliyoruz.
    let output = quote! {
        format!("INV-{}-{}-{}-{}", #module, #inv_type, #build_date, #id)
    };

    TokenStream::from(output)
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

/*
TableName derive macro'sunun quote ve syn crate'lerini kullanarak daha güvenli ve esnek
bir versiyonunu aşağıdaki gibi ele alabiliriz.
*/
#[proc_macro_derive(TableNameSafe)]
pub fn derive_table_name_safe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput); // Token Stream'i alıp anlamlı bir Abstract Syntax Tree'ye dönüştürür.
    let type_name = input.ident;
    let table_name = type_name.to_string().to_lowercase();

    let expanded = quote! {
        impl #type_name {
            pub fn table_name() -> &'static str {
                concat!("tbl_", #table_name)
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(CreatedEvent)]
pub fn derive_created_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let type_name = input.ident;
    let event_name = format!("{}_created", type_name.to_string().to_lowercase());

    let expanded = quote! {
        impl CreatedEvent for #type_name {
            fn created_event(&self) -> String {
                #event_name.to_string()
                // Burada belki de başka bir dış bağımlılığı ele alıp event'i oraya göndereceğiz
            }
        }
    };
    TokenStream::from(expanded)
}

/*
    Peki bir struct'ın alanlarını okuyarak kod üretebilir miyiz?
    Örneğin bir struct içindeki alanlar üzerinde basit validasyon işlemleri yapan kodları oluşturacak
    bir derive macro yazalım. Böylece her struct için manuel olarak validasyon kodu yazmak yerine,
    bu işlemi otomatikleştirebiliriz.
*/
#[proc_macro_derive(Validator)]
pub fn derive_validator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    // tip adını alalım
    let type_name = input.ident;

    // alanları alıyoruz. Sadece named fields yani struct { field: type } şeklinde olanları desteklemekte
    // Bunu yaparken data'yı bir pattern match ile kontrol ediyoruz.
    // Eğer struct değilse veya named fields değilse compile error üretiyoruz.
    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => {
                return quote! {
                    compile_error!("Validator only supports structs with named fields");
                }
                .into();
            }
        },
        _ => {
            return quote! {
                compile_error!("Validator only supports structs");
            }
            .into();
        }
    };

    // Her bir alan içi çok basit bir validasyon yapıyoruz.
    // Aslında sadece String türündeki alanların boş olup olmadığını kontrol ediyoruz.
    // Diğer türler için herhangi bir validasyon yapmıyoruz.
    // Gerçek hayat senaryolarında elbette daha fazla tip için kontrol yapabiliriz.
    // Bu kadarı bile karışık aslında :D
    let validations = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;

        let is_string = match &field.ty {
            Type::Path(type_path) => type_path
                .path
                .segments
                .last()
                .map(|segment| segment.ident == "String")
                .unwrap_or(false),
            _ => false,
        };

        if is_string {
            let message = format!("{} is required", field_name);
            Some(quote! {
                if self.#field_name.trim().is_empty() {
                    return Err(#message.to_string());
                }
            })
        } else {
            None
        }
    });

    // Son olarak validasyon kodlarını içeren bir impl bloğu oluşturuyoruz.
    // Dolayısıyla bir struct için bu derive macro'yu kullandığımızda,
    // o struct'a validate isimli bir metot otomatik olarak ekleniyor olacak.
    let expanded = quote! {
        impl #type_name {
            pub fn validate(&self) -> Result<(), String> {
                #(#validations)*

                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
