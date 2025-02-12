# Enum Veri Türünün Rust Tarafında Etkili Bir Kullanımı

Enum veri türü genellikle Algebraic Data Type olarak ifade edilmekte. Özellikle fonksiyonel programlama dillerinden gelenler için bu veri türü oldukça anlamlı. Tuple ve record gibi türler de bu kapsamda ele alınmakta. Yıllardır C# tarafında kodlama yapan birisi olarak enum türünün bu dilde de faydalı amaçlar için kullanıldığını ifade edebilirim. En kötü ihtimalle kafada karışılık yaratacak sayısal değerlerin anlamlı ifadeleri için kullanılabilecek bir değer türü gibi düşünülebilir. Ne var ki Rust dilindeki Enum türü çok daha zengin bir veri modeli sunuyor diyebilirim. Bunu iddia etmiyorum ama gördüğüm bazı örnekler böyle düşündürüyor. Bu kısa yazımızda C# tarafında veya herhangibir nesne yönelimli dilde icra edeceğimiz bir çözümün Rust tarafından struct yerine enum türü ile ele alınırken nasıl fark yaratabileceğini ele almaya çalışacağız. İşe şu basit senaryo ile başlayalım;

Büyük bir dağıtı sistemde yer alan servislerin uygulama içerisinde birçok noktada kullanılmasından mütevellit ortak bir model nesnesine ihtiyacımız var. Söz konusu uygulamanın bir monitoring aracı olduğunu ve servislerin aktif veya pasif _(ya da ayakta veya erişilemez durumda)_ gibi iki farklı durumda olmalarının anlam kazandığını düşünelim. Bu tip bir servis modelini büyük ihtimalle aşağıdaki gibi bir sınıfla ifade edebiliriz.

```csharp
namespace ServiceClassScenario;

public class Service
{
    public string? Name { get; set; }
    public Uri? Url { get; set; }
    public bool Online { get; set; } = false;
    public DateTime? StartTime { get; set; }

    public void Up(DateTime startTime)
    {
        Online = true;
        StartTime = startTime;
    }
    public void Down()
    {
        Online = false;
    }
}
public class Program
{
    static void Main()
    {
        var redis = new Service { Name = "Redis", Url = new Uri("http://localhost:1234"), Online = true };
        redis.Up(DateTime.Now);
        redis.Online = false;
        redis.Down();
        // Peki Online özelliği False olduğunda StartTime bilgisi null'a mı çekilmelidir?
        // Bunların önüne geçmek elbette mümkün hem de bir çok yolla. Factory metot kullanılabilir, constructor'lar private yapılabilir vs
        // Birde benzer veri yapısını rust ile yazmaya çalışalım.

    }
}
```

Yorum satırında da belirtildiği üzere elbette buradaki Service sınıfı çok daha iyi tasarlanabilirdi. Ancak dikkatinizi çekmek istediğim noktada ortada iki farklı Service nesne örneğinin değer kazandığı. Aktif olanlar veya pasif olanlar. Normalde yukarıdaki sınıfta bu nesne durumunu Online özelliği ifade ediyor. Nesnenin bu durumunu değiştirmek içinse Up veya Down gibi fonksiyonelliklerin işleyişleri ele alınıyor. Tabii public olan bu özellik herhangibir servis durumu offline iken true olarak da kalabilir. Peki bu tip bir nesne modelini Rust tarafında yazmak istesek. Büyük ihtimalle Rust dilini ilk öğrenmeye başlayan birisi doğrudan Service sınıfının bir benzerini bir struct veri modeli olarak inşa etmeye çalışır. Ancak enum veri türünün yeteneklerini iyi biliyorsak bu senaryo için rust tarafında aşağıdaki gibi bir tasarıma da gidebiliriz.

```rust
use chrono::{DateTime, Utc};

#[derive(Debug)]
enum Service {
    Offline {
        name: String,
    },
    Online {
        name: String,
        address: String,
        active: bool,
        start_time: DateTime<Utc>,
    },
}

impl Service {
    fn run(&self, address: String, start_time: DateTime<Utc>) -> Result<Self> {
        match self {
            Service::Offline { name } => {
                let created = Service::Online {
                    name: name.clone(),
                    address,
                    active: true,
                    start_time,
                };
                Ok(created)
            }
            Service::Online { .. } => Err(AlreadyOnlineError),
        }
    }
}

#[derive(Debug, Clone)]
struct AlreadyOnlineError;

type Result<T> = std::result::Result<T, AlreadyOnlineError>;

fn main() {
    let redis = Service::Offline {
        name: "Redis".to_string(),
    };
    println!("{:#?}", redis);

    if let Ok(m) = redis.run("https:://127.0.0.1:5326".to_string(), Utc::now()) {
        println!("Redis service is online");
        println!("{:#?}", m);
    }
}
```

Service enum türü tanımlanırken sadece iki duruma sahip olabileceği ifade edilmiştir. Online veya Offline. Her iki durumun sahip olması gereken veriler farklıdır. Online modda iken url, aktivasyon zamanı gibi bilgiler gerekirken Offline modda sadece hangi servisin offline olduğunu belirten bir name özelliğinin olması yeterlidir. Bu veri yapısının enum olarak tasarlanması kullanıldığı yerlerde ele alınırken pattern matching veya if let Ok gibi ifadelerle ele alınmasını şart koşar. Bu daha güçlü bir tür _(strong type)_ kullanımını da garanti eder.
