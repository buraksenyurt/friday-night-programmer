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

Malum Rust ve Zig gibi dillerde class, struct, record gibi ayrımlar yoktur. Veri yapısı genel bir kavramdır ancak bunun taşınma, kullanılma şekli farkılılklar gösterir ve buna göre belleğin farklı konumlarında tutulmaları söz konusudur. Şunu da unutmayalım ki her iki dilde Managed ortamlar içermez. Rust bellek güvenliğini ön planda tutan tedbirleri derleme zamamında ele alırken Zig daha çok C'ye yakın bir yerde konumlanan modern bir dildir. Şimdilik bu derin detayları bir kenara bırakmak istiyorum zira Güney Kore bir Go ustası gibi Rust ve Zig'i kavramam tüm ömrümü alabilir :)

Öyleyse laf kalabalığını bir kenara bırakıp rust tarafındaki kodları geliştirerek devam edelim.

```rust

```