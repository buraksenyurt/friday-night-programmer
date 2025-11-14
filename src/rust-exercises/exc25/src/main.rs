use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static REQUEST_COUNTER: Mutex<i32> = Mutex::new(0);
const THRESHOLD_LEVEL: i32 = 100;

fn main() {
    let mut handlers = vec![];

    // Farklı servis isteklerini simüle eden thread'ler oluşturuyoruz
    // İlk thread ProductService isteklerini simüle ediyor
    let handle = thread::spawn(move || {
        for _ in 1..100 {
            handler(
                ServiceId {
                    name: "ProductService",
                    id: 1,
                },
                "api/product/10".to_string(),
            );
        }
    });
    handlers.push(handle);

    // İkinci thread CatalogService isteklerini simüle ediyor
    let handle = thread::spawn(move || {
        for _ in 1..100 {
            handler(
                ServiceId {
                    name: "CatalogService",
                    id: 2,
                },
                "api/catalog/computers/top/10".to_string(),
            );
        }
    });
    handlers.push(handle);

    // Tüm thread'lerin bitmesini bekliyoruz
    for handle in handlers {
        handle.join().unwrap();
    }
}

/*
    Sunucuya gelen servis isteklerini ele alan handler fonksiyon olarak düşünebiliriz.
    Parametrelerin senaryomuz gereği çok bir önemi yok.

*/
fn handler(service: ServiceId, body: String) {
    loop {
        /*
            Sonsuz döngüde iken sayacı hemen 1 artırıyoruz.
            Ardından sembolik olarak gelen isteği işliyoruz.
            Son olarak sayaç eşiği aşıldıysa alarm fonksiyonunu çağırıyoruz.
        */

        // REQUEST_COUNTER değişkenini kullanabilmek için öncelikle kilidini açıyoruz
        let mut counter = REQUEST_COUNTER.lock().unwrap();
        // * operatörü ile Mutex içindeki gerçek değere erişiyoruz
        *counter += 1;

        _ = read_request(&body);

        // Sayaç eşiğini aşıp aşmadığını kontrol ediyoruz
        if *counter > THRESHOLD_LEVEL {
            alert(service);
        }
    }
}

// Simülasyon amaçlı çalışan ve gelen isteği güya işleyen bir fonksiyon
fn read_request(body: &str) -> Result<(), ()> {
    // Sanki gerçekten bir iş yapılıyormuş gibi talep okuma işini belirli bir süre uyutuyoruz
    println!("Processing request body: {}", body);
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

/*
    Uyarı mesajı veren fonksiyon.
    Örneğin basit olması açısından sadece mesaj veriyoruz.
    Aslında buradan dönecek değere göre ana süreç servis çağrılarını başka bir servise yönlendirebilir.
*/
fn alert(service: ServiceId) {
    println!("Alert for {:?}", service);
}

// Sadece servis ile ilgili bilgi taşımak için kullandığımız bir veri yapısı
// Sembolik olarak servisin adını ve sayısal değerini taşıyor
#[derive(Debug, Copy, Clone)]
struct ServiceId<'a> {
    id: u32,
    name: &'a str,
}
