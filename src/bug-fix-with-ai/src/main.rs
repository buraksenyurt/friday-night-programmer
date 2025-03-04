/*
    Dil modelleri için çok basit benchmark' lar içeren bir uygulama.

    Daha gelişmiş testler için code challenge siteleri de kullanılabilir.

    ## HATA DÜZELTME (Bug Fixing  - bug_fixing.rs)
    bug_fix modülünde kasıtlı olarak bırakılmı 'Value used after being moved' hatası.
    Bunu kodlamada uzmanlaşmış bir dil modelinin başarısını ölçmek için kullanabiliriz.
    Modelden jatayı tespit edip doğru düzeltmeyi yapması istenir.

    ## Kod Tamamlama (Code Completion - code_completion.rs)
    code_completion modülünde kasıtlı olarak bırakılmış bir kod tamamlama sorunu var.
    Modelden kodun tamamlanması istenir.

    ## Birim Test Yazma (Unit Test Writing - write_test.rs)
    write_test modülündeki basit fonksiyon için birim testler yazdırılması istenir.

    ## Kod İyileştirme (Refactoring - refactoring.rs)
    refactoring modülündeki kodun daha okunabilir ve daha performanslı hale getirilmesi istenir.

    ## Dokümantasyon Oluşturma (Documentation - documentation.rs)
    Faktoryel hesaplayan fonksiyon için dokümantasyon oluşturulması istenir.

    ## Hata Mesajlarına Göre Debug İşlemleri (Debugging - debugging.rs)
    debugging modülündeki kodun çalıştırılmasında oluşabilecek hataları analiz etmesi ve kod düzeltmesi istenebilir.

*/
mod bug_fixing;

fn main() {
    bug_fixing::run(); // build hatası içeriyor
    debugging::run(); //runtime hatası içeriyor
}
