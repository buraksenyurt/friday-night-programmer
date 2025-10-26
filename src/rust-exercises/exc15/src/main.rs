// Sahipliği devralan fonksiyon
#[allow(dead_code)]
fn validate_with_ownership(input: String) -> bool {
    // Basit bir doğrulama: Şimdilik gelen veri içeriği boş değilse geçerli kabul ediyoruz
    !input.trim().is_empty()
    // input değişkeni fonksiyonun sonunda scope dışına çıktığında bellekten otomatik olarak temizlenecektir
}

// Sahipliği göz ardı eden fonksiyon
fn validate_without_ownership(input: &str) -> bool {
    // Basit bir doğrulama: Şimdilik gelen veri içeriği boş değilse geçerli kabul ediyoruz
    !input.trim().is_empty()
}

fn main() {
    let user_input = String::from("<body><title>Request Form</title></body>");

    // Fonksiyona sahipliği devretmiyoruz, sadece referansını geçiriyoruz
    let is_valid = validate_without_ownership(&user_input);

    if is_valid {
        println!("Request is valid: {}", user_input);
    } else {
        println!("Invalid request.");
    }

    // user_input bu scope içerisinde hala kullanılabilir durumda çünkü sahipliği ilgili fonksiyonuna geçmedik
    println!("Original input is still available: {}", user_input);

    /*
        Aşağıdaki kullanımda owned_input değişkeninin sahipliği validate_with_ownership fonksiyonuna
        devredildiği için, fonksiyon çağrısından sonra owned_input değişkeni geçersiz hale gelir.
        Bu nedenle, fonksiyon çağrısından sonra owned_input değişkenine erişmeye çalışmak
        derleme hatasına neden olur. 

        error[E0382]: borrow of moved value: `owned_input`
        --> exc15\src\main.rs:35:23
        |
        27 |     let owned_input = String::from("<body><title>Owned Request Form</title></body>");
        |         ----------- move occurs because `owned_input` has type `String`, which does not implement the `Copy` trait
        28 |     // Fonksiyona sahipliği devrediyoruz
        29 |     let is_owned_valid = validate_with_ownership(owned_input);
        |                                                  ----------- value moved here
        ...
        35 |     let body_length = owned_input.len(); // Hata: owned_input artık geçerli değil
        |                       ^^^^^^^^^^^ value borrowed here after move
        |
        note: consider changing this parameter type in function `validate_with_ownership` to borrow instead if owning the value isn't necessary
        --> exc15\src\main.rs:1:35
        |
        1  | fn validate_with_ownership(input: String) -> bool {
        |    -----------------------        ^^^^^^ this parameter takes ownership of the value
        |    |
        |    in this function
        help: consider cloning the value if the performance cost is acceptable
        |
        29 |     let is_owned_valid = validate_with_ownership(owned_input.clone());
        |                                                             ++++++++

        For more information about this error, try `rustc --explain E0382`.
        warning: `exc15` (bin "exc15") generated 1 warning

        Burada fonksiyona referans yolu ile sahipliği devrederek ilerlemek daha güvenlidir.
        Ya da maliyetine katlanarak klonlama (clone) yapabiliriz.
        Hatta çağırılan fonksiyondan geriye yeni bir String dönerek sahipliği koruyabiliriz. 
        Ancak bu senaryoda ideal olan referans ile geçiş yapmaktır.
    */
    // let owned_input = String::from("<body><title>Owned Request Form</title></body>");
    // // Fonksiyona sahipliği devrediyoruz
    // let is_owned_valid = validate_with_ownership(owned_input);
    // if is_owned_valid {
    //     println!("Owned request is valid.");
    // } else {
    //     println!("Invalid owned request.");
    // }
    // let body_length = owned_input.len(); // Hata: owned_input artık geçerli değil
}