# Bellek Yönetimi Üzerine Notlar

**Unmanaged** ortamlarda gezinmek birçok yeni veya unutulmuş bilgiyi de karşıma çıkarıyor. Geçtiğimiz günlerde devasa boyutlarda JSON tabanlı logları işlerken **Interning** stratejisi ile belleğe alınan verinin nasıl optimize edilebileceğini öğrendim. Belli senaryolarda _(her zaman da avantajlı olmayabiliyor)_ çok sık tekrar eden string içeriklere heap'de yer ayırılırken gereksiz alan ayırmak yerine, bunları refere eden tekil pointer'lardan yararlanmak ve hatta benzersiz sayısal değerlerle _(örneğin pozitif bir tam sayı ile)_ bir vektör içerisinde tutup _(Intern havuzu olarak da ifade ediliyor)_ erişimi hızlandırmak mümkün. Tam anlamıyla bellek seviyesinde optimizasyon ve performans kazanımı söz konusu. Biraz karışık bir cümle oldu ama refere edeceğim [şu yazının uzunluğu](https://gendignoux.com/blog/2025/03/03/rust-interning-2000x.html) düşünülünce elden bu kadar geldi. Yazıda Paris'in herkese açık otoyol verilerinden yararlanılıyor. Veriler çok büyük ve yazarın iddiasına göre 2bin kata kadar küçültülebiliyor. Örneğin sadece String veriler üzerine yapılan Interning tekniğinin **%47** oranında yer kazanımı sağladığı belirtiliyor. Tabii olay bellek yönetimi, bellek operasyonlarında optimizasyon ve performans işlemleri denince karşımıza daha birçok kavram da çıkıyor. Örneğin **Region-Based Management** konseptinde yer bulan **Area Allocators** ve diğer şeyler... **Copy on Write(CoW)**, **Zero Cost Abstraction**, **Memory/Object Pooling**, **Cache-Aware Programming**, **Enum'larda Padding ve Allignment** kullanımı vs

Rust, zaten sahip olduğu bazı özellikleri ile _(Ownership, borrow-checker- lifetimes vb)_ belleği güvenli noktada tutmak için imkanlar sağlıyor _(Bilindiği üzere Rust'ta bir Garbage Collector mekanizması yok)_ Ne var ki yine de bazı senaryolarda belleği efektif kullanmak için az önce saydığım yaklaşımlara da değinmek gerekiyor. Söz gelimi Interning konusunda yazılmış duruma göre tercih edilebilecek birçok crate mevcut. Bu yazıda amacım diğer konulara ait kısa kısa notlar tutmak.

## CoW _(Copy on Write)_

Sadece gerektiği zaman veri kopyalanmasını öneren bir teknik olduğu ifade ediliyor. Bunu şöyle de ifade edebiliriz; Koypalama işlemini ilk yazma anına kadar geciktirmek. Rust içerisinde bu felsefe ile donatılmış Cow isimli bir enum türü bulunuyor _(ki ayrıca bir Smart Pointer türüdür)_ Diğer yandan resmi kaynaklarda açılımı **Clone on Write** şeklindedir. Bu enum türü içerdiği veriyi ya ödünç alır _(borrow)_ ya da yeni bir kopyası ile sahipliğini alır _(ownership)_ Dolayısıyla veriyi değiştirme ihtiyacı yoksa referansını kullanır ama değişiklik gerekirse de ilk değişiklik gerektiği noktada verinin bir klonu hazırlanıp sahipliği ele alınır. Bu noktada aşağıdaki örnek kod parçasını göz önüne alabiliriz.

```rust
use std::borrow::Cow;

fn main() {
    let user_one = "Super Mario";
    let player_two = "Ready Player One";
    let length = 16;

    println!("{}", padding_end(user_one, length));
    println!("{}", padding_end(player_two, length));
}

fn padding_end<'a>(input: &'a str, target_len: usize) -> Cow<'a, str> {
    if input.len() < target_len {
        // Yeni string oluşturur ve target_len'e göre belli sayıda _ karakteri ekler
        Cow::Owned(format!("{:_<width$}", input, width = target_len))
    } else {
        // Yeterli uzunlukta olduğu için yeni bir versiyon oluşturmadan orijinal referansı döndürür
        Cow::Borrowed(input)
    }
}
```

Burada neler oluyor!? Son derece anlamsız bir fonksiyon olan **padding_end**, parametre olarak gelen literalın sonuna target_len ile belirtiliği kadar _ işareti ekliyor ancak gelen içeriğin uzunluğu zaten o kadar ise eklemiyor. Yalnız bu ekleme ve eklememe arasında önemli bir fark var. İfade sonunda boşluklar kalmışsa yeni bir string oluşturuluyor ve sahipliği fonksiyondan geriye dönüyor _(Owned çağrısı)_ . Diğer durumda ise orjinal referans geri dönüyor _(Borrowed çağrısı)_ 

Copy on Write işletim sistemlerinde ortak page kullanımlarında ya da immutable veritabanı yapılarından yaygın olarak kullanılıyormuş. Gözle görmedim o yüzden ispat edemem ancak Rust'ın Clone on Write özelliğinin daha çok programlama dili ile ilgili olduğu da belirtilmekte. Genel ve yaygın adı **Copy on Write** olsa da Rust tarafındaki **CoW** doğrudan mutasyona izin vermediği için daha çok **Clone on Write** olarak ifade edilmektedir ama özünde benzer bir felsefeye sahip olduklarını ifade edebiliriz diye düşünüyorum.

Farklı bir örnek daha ekleyelim.

```rust
fn remove_ellipsis_dots(input: &str) -> String {
    input.to_string().replace('`', "")
}
```

Yukarıdaki fonksiyon gelen literal içerisindeki üsten virgül işaretlerini bulup kaldırıyor. Burada yeni bir String türünün oluşturulması söz konusu ve bu her seferinde yapılıyor. Bir başka deyişle gelen input içerisinde üstten virgül işaret yoksa bile bir String üretimi söz konusu. İşte tam bu noktada eğer üstten virgül yoksa aynı referansı kullanmaya devam et diyebiliriz. Cow kullanarak tabii. Yani fonksiyon aşağıdaki hale getirilebilir.

```rust
fn remove_ellipsis_dots(input: &str) -> Cow<str> {
    if input.contains('`') {
        Cow::Owned(input.to_string().replace('`', ""))
    } else {
        Cow::Borrowed(input)
    }
}
```

Manupilasyon sadece gerektiği zamanlarda yapılmış olur. Cow kullanımı ile ilgili halen daha kafamı karıştıran birçok örnek var. Bunları iyice araştırmak lazım.

### Dirty Cow Mevzusu _(Pis İnek :P)_

**Dirty Cow** olarak isimlendirilen [bir güvenlik zafiyeti](https://cve.mitre.org/cgi-bin/cvename.cgi?name=cve-2016-5195) var. Copy on Write'ın yanlış kullanımı neticesinde salt okunur veri alanına çeşitli ayrıcalıklar _(privilages)_ eklenmesi sonucu böyle bir zafiyet oluşmuş ve 2018 öncesi sürüme sahip birçok Linux sistemi durumdan etkilenmiş. Aslında durumun vehameti [Wikipedia' da](https://en.wikipedia.org/wiki/Dirty_COW) özetlenmiş. Söz konusu güvenlik zafiyetine konu olan Copy on Write stratejisinin Rust tarafındaki CoW kullanımı ile bir ilişkisi olmayabilir zira Rust’taki Cow, Linux çekirdeği seviyesindeki Copy-on-Write mekanizmasıyla aynı değildir. Rust’taki Cow türü tamamen kullanıcı seviyesi _(user-space)_ işleyen bir veri yapısıdır ve işletim sistemi seviyesinde process bazlı sayfa _(page)_ paylaşımı yapmaz. Tabii söz konusu güvenlik zafiyeti Data Races oluşması halinden, doğru zamanlama ile salt okunur bellek sayfalarına ayrıcalıklar _(privilages)_ eklenmesinden ve uzaktan root kullanıcı ayrıcalıklarına sahip olunabilmesinden bahseder ki bunlara sebebiyet veren durumlar Rust'ın ownership, borrow checker gibi mekanizmaları sayesinde henüz derleme aşamasından engellenirler. Yine de unsafe alanlarda çalışırken veya Arc kullanımlarında dikkatli olmakta yarar var diye düşünüyorum.

## Arena Allocators

## Enum Padding/Alignment

## Memory/Object Pooling

## Cahce Aware Programming

## Zero Cost Abstraction