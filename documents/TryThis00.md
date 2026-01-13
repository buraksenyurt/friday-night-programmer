# Metot Parametrelerinden Gelen Değerleri Değiştirmek

C# programlama dilinde sınıf nesne örnekleri *(Class Object Instance)* metotlara varsayılan olarak referans türü *(reference type)* olarak iletilir. Bu, metot içinde yapılan değişikliklerin çağıran tarafı etkileyebileceği anlamına gelir. Bu çalışmada söz konusu senaryonun Rust ve Zig gibi programlama dillerinde nasıl ele alındığını incelemeye çalışıyorum.

> Şu notu düşmek isterim: Bu çalışmada amaç üç dili birbiriyle kıyaslamak değil, her bir dilin bu tip senaryoya yaklaşımını incelemektir. Normal koşullarda bir stok takip programını Rust ve Zig ile geliştirmeyi tercih etmem.

## C# Kod Örneği

İlk olarak C# tarafında bir kurgu hazırlayalım. Bu amaçla basit bir Console uygulaması açıp içerisine aşağıdaki kodları ekleyelim.

```csharp
namespace CSharpSample;
public class Program
{
    static void Main(string[] args)
    {
        Part part = new()
        {
            Id = 1,
            Name = "Widget",
            StockLevel = 100
        };

        Console.WriteLine($"Before update: Part ID: {part.Id}, Name: {part.Name}, Stock Level: {part.StockLevel}");
        UpdateStockLevel(part, 150);
        Console.WriteLine($"After update: Part ID: {part.Id}, Name: {part.Name}, Stock Level: {part.StockLevel}");
    }

    static void UpdateStockLevel(Part part, int newStockLevel)
    {
        part.StockLevel = newStockLevel;
    }
}

public class Part
{
    public uint Id { get; set; }
    public string? Name { get; set; }
    public int StockLevel { get; set; }
}
```

Yukarıdaki gibi bir kodlamayı kuvvetle muhtemel yapmayız. En azından Part isimli veri yapısının state'ini dışarıdan bir fonksiyon ile değiştirmeyi pek tercih etmeyiz. Ancak buradaki amaç için ele alabiliriz. Çalışma zamanındaki çıktı aşağıdaki gibi olacaktır.

![TryThis00_0](../images/TryThis00_0.png)

Senaryoyu kısaca ele alalım. Main metodu içerisinde tanımladığımız Part isimli nesne örneğini, UpdateStockLevel metoduna parametre olarak gönderiyoruz. Bu bir sınıf olduğundan varsayılan olarak referans türü şeklinde iletiliyor. Dolayısıyla UpdateStockLevel metodu içerisinde yapılan değişiklikler, Main metodundaki orijinal nesneyi de doğrudan etkiliyor. Önceden de belirttiğim üzere bu şekilde state değiştirmeyi pek tercih etmem. En azından nesne örneğinin kendi üzerinden yapılmasının ya da değişiklik sonrası yeni bir Part nesnesinin geriye döndürülmesinden yanayım diyebilirim. Neyse neyse. Bunu bir kenara bıraklım. Şimdi aynı senaryoyu Rust dilinde ele almak istediğimizi düşünelim.

> Sevdiğim bir Rust programcısının şöyle bir sözü vardır; "Bu projede dot netçi gibi düşünerek Rust kodlaması yapmak istemedim" :D

## Rust Kod Örneği

Malum Rust ve Zig gibi dillerde class, struct, record gibi sayrımlar yoktur. Veri yapısı genel bir kavramdır ancak bunun taşınma, kullanılma şekli farkılılklar gösterir ve buna göre belleğin farklı konumlarında tutulmaları söz konusudur. Şunu da unutmayalım ki her iki dilde Managed ortamlar içermez. Rust bellek güvenliğini ön planda tutan tedbirleri derleme zamamında ele alırken Zig daha çok C'ye yakın bir yerde konumlanan modern bir dildir. Şimdilik bu derin detayları bir kenara bırakmak istiyorum zira Güney Kore bir Go ustası gibi Rust ve Zig'i kavramam tüm ömrümü alabilir :)

Öyleyse laf kalabalığını bir kenara bırakıp rust tarafındaki kodları geliştirerek devam edelim. İlk olarak aynı yaklaşımı benimseyelim.

```rust
fn main() {
    let mut part = Part {
        id: 1,
        name: String::from("Widget"),
        stock_level: 10,
    };

    println!("Initial stock level of {}: {}", part.name, part.stock_level);
    change_stock_level(part, 5);
    println!("Stock level after adding 5: {}", part.stock_level);
}

fn change_stock_level(part: Part, change: i32) {
    part.stock_level += change;
}

struct Part {
    id: u32,
    name: String,
    stock_level: i32,
}
```

Rust dilinde değişkenler varsayılan olarak immutable *(yani değeri değiştirilemez)* olarak kabul edilir. Bende **main** metodunun kapsama alanındaki **Part** nesnesini değiştirmek istediğimden onu **mut** anahtar kelimesi ile birlikte tanımladım. DotNetçi gibi düşünmememiz gerekse de velev ki düşündük ve programının beklediğimiz şekilde çalışacağını hayal ettik. Ne mi olur? Neredeyse kod satırı sayısından da fazla satırdan oluşan bir hatalar ve uyarılar listesi ile karşılaşırız. Aynen aşağıdaki ekran görüntüsünde olduğu gibi.

![TryThis00_1](../images/TryThis00_1.png)

Olay main metodu içerisinde değişkenin sadece mutable olarak tanımlayıp her şeyin pürüzsüz bir şekilde çalışacağını zannetmemiz olacaktır. Rust dilinde bir değişkeni mutable olarak tanımlamak, o değişkenin sahip olduğu değerin başka bir metoda aktarılması durumunda orada da **mutable** olarak kabul edileceği anlamına gelmez. Rust dilindeki sahiplik *(ownership)* kavramı, bir değişkenin değerinin başka bir fonksiyona aktarılması durumunda orada yeni bir sahiplik oluşturulmasını gerektirir. Bu durumda, orijinal değişkenin sahipliği kaybolur ve artık orijinal değişkene de erişilemez hale gelir. Yani programcı olarak metoda aktardığım değişkenin referans üzerinden aktarılacağını ve açık bir şekilde bu referansın işaret ettiği değerin değiştirileceğini belirtmem söyleniyor. O halde kodları aşağıdaki gibi revize edelim.

```rust
fn main() {
    let mut part = Part {
        id: 1,
        name: String::from("Widget"),
        stock_level: 10,
    };

    println!("Initial stock level of {}: {}", part.name, part.stock_level);
    change_stock_level(&mut part, 5);
    println!("Stock level after adding 5: {}", part.stock_level);
}

fn change_stock_level(part: &mut Part, change: i32) {
    part.stock_level += change;
}

struct Part {
    id: u32,
    name: String,
    stock_level: i32,
}
```

id alanının hiç kullanmadığım için aldığım bir uyarı mesajı var ama onu şimdilik görmezden geliyorum. Programı tekrar çalıştırdığımızda beklediğimiz davranışın gerçekleştirildiğini görürüz.

![TryThis00_2](../images/TryThis00_2.png)

> Burada ikinci kısımda ele almak üzere bir soru bırakayım; Ya birden fazla Thread bu parçanın stok seviyesini değiştirmek isterse? C# ne yapar, Rust ne kadar zorluk çıkartır, Zig'den ne haber?

## Zig Kod Örneği

Gelelim yeni yeni öğrenmeye çalıştığım Zig diline. Zig dilinde Rust'ta olduğu gibi immutable/mutable ayrımları söz konusu ancak ifade edilme şekilleri biraz daha farklı. Özellikle değerlerinde değişiklik yapılmasını istediğimiz değişkenler **var** anahtar kelimesi ile birlikte tanımlanmak zorundalar. Bunların dışındakiler ise **const** anahtar kelimesi ile tanımlanıyorlar. Üstelik Zig dilinde fonksiyon parametreleri gerçekten de değiştirilemez *(immutable)* olarak kabul edilmekteler. Yine bir DotNetçi gibi düşünerek zig tarafı kodlarını da aşağıdaki gibi geliştirerek ilerleyelim.

```zig
```
