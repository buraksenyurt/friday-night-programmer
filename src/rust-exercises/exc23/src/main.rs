use std::os::raw::{c_int, c_uint};
use std::sync::Mutex;

/*
    C ile yazılmış rand/srand fonksiyonlarını Rust tarafında kullanmak için FFI(Foreign Function Interface) tanımı.
    libc kütüphanesinden srand fonksiyonu ile rastgele sayı üreteci başlatılır ve rand fonksiyonu ile rastgele sayı alınır.
*/
unsafe extern "C" {
    fn rand() -> c_int;
    fn srand(seed: c_uint);
}

// Thread-safety için global mutex oluşturmamızda fayda var
static RAND_MUTEX: Mutex<()> = Mutex::new(());
static mut INITIALIZED: bool = false;

fn main() {
    for i in 0..5 {
        let random_number = generate_random_number();
        println!("#{}: {}", i + 1, random_number);
    }
}
/*
    Güvenli Soyutlamayı yaptığımız metot.
    Öncelikle global mutext ile thread-safety sağlanıyor.
    Daha sonra unsafe blok içinde C'nin srand fonksiyonu ile rastgele sayı üreteci initialize ediliyor.
    Ardından rand fonksiyonu çağrılıyor ve dönen değer güvenli bir şekilde u32'ye çevriliyor.
    Negatif değerler 0'a mapleniyor, böylece u32 overflow riski minimize ediliyor.
    Tüm unsafe kod bu fonksiyon içinde kapsülleniyor, dışarıya güvenli bir API sunuluyor.
*/
fn generate_random_number() -> u32 {
    // Thread safety için lock alıyoruz
    let _guard = RAND_MUTEX.lock().unwrap();

    /*
        unsafe blok için iki C fonksiyonu çağrılıyor.
        ilki srand ile rastgele sayı üretecini initialize ediyor.
        Burada amaç tutarlı rastgele sayılar üretmek.
        İkinci fonksiyon rand ile de gerçekten bir rastgele sayı oluşturuluyor.
    */
    unsafe {
        if !INITIALIZED {
            let seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as c_uint;
            srand(seed);
            INITIALIZED = true;
        }
    }

    let result = unsafe { rand() };

    // max çağrısı ile negatif değerleri 0'a map ederek bir overflow oluşma riskini minimize ediyoruz
    result.max(0) as u32
}
