use tokio::time::{self, Duration};
use std::thread;

#[tokio::main]
async fn main() {
    call().await;
}

async fn call(){
    let start_time = time::Instant::now();
    println!("Service started...");

    // Bad Practice: CPU yoğun işlemi doğrudan asenkron bağlam içinde ele aldığımızda
    // asıl executor'ı da engeller
    let pwd = decrypt("some hash value");

    // // Good Practice: CPU yoğun işlemi spawn_blocking ile ayrı bir thread pool'a devrediyoruz
    // let pwd_handle = tokio::task::spawn_blocking(|| {
    //     decrypt("some hash value")
    // });

    // Diğer asenkron işlemleri simüle etmek için geçici bir bekleme yapıyoruz
    let io_opt = time::sleep(Duration::from_millis(500));

    // Burada tokio join ile iki asenkron işlemi paralel olarak işletiliyor
    tokio::join!(
        async{
            // Sembolik bir I/O operasyonu icra ettiğimizi düşünelim.
            println!("I/O operations completed");
            io_opt.await;
            println!("I/O wait is over");
        },
        async {
            // Bad Practice :
            println!("Decryption result '{}'",pwd);

            // // Good Practice :
            // let pwd = pwd_handle.await.expect("Blocking task failed.");
            // println!("Decryption result '{}'",pwd);
        }
    );

    /*
        Toplam süreyi raporluyoruz.
        Gözlemlere göre spawn_blocking kullanımı ile asenkron işlemler engellenmeden paralel yürütülüyor.
        Buna göre toplam çalışma süresi yaklaşık olarak 1 saniye civarında oluyor.
        Ancak decrypt fonksiyonunu doğrudan asenkron bağlam içinde çağırıldığında bu süre 1.5 saniye civarına çıkıyor.
        Çünkü, decrypt fonksiyonu asenkron executor'ı bloke ediyor.

        Bad Practice toplam süre: ~1500 ms ve çalışma zamanı çıktısı:

        Service started...
        Starting decryption for 'some hash value'
        I/O operations completed
        Decryption result 'value decrypted'
        I/O wait is over
        Total process duration is 1506

        Good Practice toplam süre: ~1000 ms ve çalışma zamanı çıktısı:

        Service started...
        I/O operations completed
        Starting decryption for 'some hash value'
        I/O wait is over
        Decryption result 'value decrypted'
        Total process duration is 1002
    */

    println!("Total process duration is {}",start_time.elapsed().as_millis());
}

fn decrypt(value:&str) -> String {
    println!("Starting decryption for '{}'",value);
    thread::sleep(Duration::from_millis(1000));
    "value decrypted".to_string()
}
