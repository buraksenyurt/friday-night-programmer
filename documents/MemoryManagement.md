# Bellek Yönetimi Üzerine Notlar

**Unmanaged** ortamlarda gezinmek birçok yeni veya unutulmuş bilgiyi de karşıma çıkarıyor. Geçtiğimiz günlerde devasa boyutlarda JSON tabanlı logları işlerken **Interning** stratejisi ile belleğe alınan verinin nasıl optimize edilebileceğini öğrendim. Belli senaryolarda _(her zaman da avantajlı olmayabiliyor)_ çok sık tekrar eden string içeriklere heap'de yer ayırılırken gereksiz alan ayırmak yerine, bunları refere eden tekil pointer'lardan yararlanmak ve hatta benzersiz sayısal değerlerle _(örneğin pozitif bir tam sayı ile)_ bir vektör içerisinde tutup _(Intern havuzu olarak da ifade ediliyor)_ erişimi hızlandırmak mümkün. Tam anlamıyla bellek seviyesinde optimizasyon ve performans kazanımı söz konusu. Biraz karışık bir cümle oldu ama refere edeceğim [şu yazının uzunluğu](https://gendignoux.com/blog/2025/03/03/rust-interning-2000x.html) düşünülünce elden bu kadar geldi. Yazıda Paris'in herkese açık otoyol verilerinden yararlanılıyor. Veriler çok büyük ve yazarın iddiasına göre 2bin kata kadar küçültülebiliyor. Örneğin sadece String veriler üzerine yapılan Interning tekniğinin **%47** oranında yer kazanımı sağladığı belirtiliyor. Tabii olay bellek yönetimi, bellek operasyonlarında optimizasyon ve performans işlemleri denince karşımıza daha birçok kavram da çıkıyor. Örneğin **Region-Based Management** konseptinde yer bulan **Area Allocators** ve diğer şeyler... **Copy on Write(CoW)**, **Zero Cost Abstraction**, **Memory/Object Pooling**, **Cache-Aware Programming**, **Enum'larda Padding ve Allignment** kullanımı vs

- [Bellek Yönetimi Üzerine Notlar](#bellek-yönetimi-üzerine-notlar)
    - [Cow - Copy on Write/Clone on Write](#cow-copy-on-write)
        - [Dirty Cow Mevzusu](#dirty-cow-mevzusu-pis-i̇nek-p)
    - [Arena Allocators](#arena-allocators)
        - [AtomicUsize Kullanımı](#atomicusize-kullanımı)
    - [Struct/Enum Türlerinde Padding ve Allignment](#structenum-türlerinde-padding-ve-allignment)
    - [Memory/Object Pooling](#memoryobject-pooling)
    - [Cache Aware Programming](#cache-aware-programming)
    - [Zero Cost Abstraction](#zero-cost-abstraction)

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

![cow_runtime](../images/cow_runtime.png)

Manupilasyon sadece gerektiği zamanlarda yapılmış olur. Cow kullanımı ile ilgili halen daha kafamı karıştıran birçok örnek var. Bunları iyice araştırmak lazım.

### Dirty Cow Mevzusu _(Pis İnek :P)_

**Dirty Cow** olarak isimlendirilen [bir güvenlik zafiyeti](https://cve.mitre.org/cgi-bin/cvename.cgi?name=cve-2016-5195) var. Copy on Write'ın yanlış kullanımı neticesinde salt okunur veri alanına çeşitli ayrıcalıklar _(privilages)_ eklenmesi sonucu böyle bir zafiyet oluşmuş ve 2018 öncesi sürüme sahip birçok Linux sistemi durumdan etkilenmiş. Aslında durumun vehameti [Wikipedia' da](https://en.wikipedia.org/wiki/Dirty_COW) özetlenmiş. Söz konusu güvenlik zafiyetine konu olan Copy on Write stratejisinin Rust tarafındaki CoW kullanımı ile bir ilişkisi olmayabilir zira Rust’taki Cow, Linux çekirdeği seviyesindeki Copy-on-Write mekanizmasıyla aynı değildir. Rust’taki Cow türü tamamen kullanıcı seviyesi _(user-space)_ işleyen bir veri yapısıdır ve işletim sistemi seviyesinde process bazlı sayfa _(page)_ paylaşımı yapmaz. Tabii söz konusu güvenlik zafiyeti Data Races oluşması halinden, doğru zamanlama ile salt okunur bellek sayfalarına ayrıcalıklar _(privilages)_ eklenmesinden ve uzaktan root kullanıcı ayrıcalıklarına sahip olunabilmesinden bahseder ki bunlara sebebiyet veren durumlar Rust'ın ownership, borrow checker gibi mekanizmaları sayesinde henüz derleme aşamasından engellenirler. Yine de unsafe alanlarda çalışırken veya Arc kullanımlarında dikkatli olmakta yarar var diye düşünüyorum.

## Arena Allocators

N sayıda nesne için bellekte baştan bir yer ayırıp bunları bu alanda toplamak ve sonrasında hepsini tek seferde düşürmek istediğimiz senaryolarda kullanılan bir teknik olarak ifade edilmektedir. Temel çalışma prensibine göre program ayağa kalkarken bellekte belli bir bölge bu iş için tahsis edilir ve gerekli nesneler söz konusu alana ardışık olarak dizilir. Bölgenin serbest bırakılması nesnelerin de topluca ve tek seferde bellekten düşürülmesi anlamına gelir. Bir arena oluşturulduğunda işaretçi _(pointer)_ başlangıç konumuna alınır ve diğer nesnelere ardışıl olarak ulaşılması da kolaylaşır ki bunun da performans açısından önemli bir artısı olduğu söylenebilir. Hatta bu alanlar işletim sistemlerinin kullandığı ön belleklere de benzetilir. Bu stratejide tüm bellek bölgesinin tek seferde düşmesi en önemli noktalardan birisidi ancak zamanı geldiğinde tek tek düşürülmesi gereken nesneler söz konusu ise bunları arena içerisinde ele almak mümkün değildir ya da tam tersi nesneler topluca serbest kalırken yaşaması gerekenler varsa bu yöntem kullanışlı olmaz. Bir başka deyişle aynı yaşam ömrüne sahip ya da birlikte sona eren ve çok büyük boyutlu olmayan nesnelerin organizasyonu için daha idealdir. Rust'ta bu amaçla kullanabilecek birçok küfe de bulunuyor. Bunlardan birisi de [bumpalo](https://crates.io/crates/bumpalo) ve işte basit kullanım örneği.

```rust
use bumpalo::Bump;

#[derive(Debug)]
struct Position {
    x_value: u32,
    y_value: u32,
    z_value: u32,
}

pub fn run() {
    let bump = Bump::new();

    let player_one = bump.alloc(Position {
        x_value: 10,
        y_value: 20,
        z_value: 0,
    });
    let player_two = bump.alloc(Position {
        x_value: 15,
        y_value: 5,
        z_value: 30,
    });
    let john_doe = bump.alloc(Position {
        x_value: 3,
        y_value: 5,
        z_value: 8,
    });

    println!("Player One Adresi {:p}", player_one);
    println!("Player Two Adresi {:p}", player_two);
    println!("John Doe Adresi {:p}", john_doe);

    let player_one_addr = player_one as *const _ as usize;
    let player_two_addr = player_two as *const _ as usize;
    let john_doe_addr = john_doe as *const _ as usize;

    println!(
        "Gerçek Player Two - Player One adres farkı: {} byte",
        address_diff(player_two_addr, player_one_addr)
    );
    println!(
        "Gerçek John Doe - Player Two adres farkı: {} byte",
        address_diff(john_doe_addr, player_two_addr)
    );
    
    // Arena burada scope'dan çıkarken içindeki tüm Player nesneleri de tek seferde düşürülecektir
}

fn address_diff(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}
```

Örnekte bump nesnesi oluşturulduktan sonra içerisine üç farklı Position nesne örneği ekleniyor. Sadece bu örnek özelinde bunların kodlama sırası ile olmasa da ardışıl olarak dizildiklerini ispat edebilmek için adres bilgileri arasındaki farklar hesaplanıyor. Position yapısı 4 byte'tan 3 alan içermekte ve dolayısıyla 12 byte yer kaplamakta. Dolayısıyla nesnelerin başlangıç adresleri arasında 12 byte mesafe olmalı. Elbette bunu çok daha büyük boyutlu bir nesne kümesi için kontrol etmek lazım, tam bir ispattır diyemeyiz.

Rust programlama dili çalışma zamanında nesnelerin temizlenmesi için **Resource Acquisition is Initialization _(RAII)_** prensibini kullanır. Buna göre bir değer scope dışında çıktığında düşer ve hatta referans türlü yani heap bazlı bir nesne ise _(Box edilmiş bir tür, Vector gibi)_ drop trait davranışı çalıştırılır. Area Allocator teorisinde ise bu bellek alanının tek seferde düşürülmesi söz konusudur. Dolayısıyla bölgeye atılan nesneler için drop implementasyonlarının çalışmaması gerekir. Bumpalo kütüphanesi açısından bakarsak aşağıdaki örnek kodla durumu özetleyebilir.

```rust
use bumpalo::Bump;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
struct Position {
    id: u32,
    x_value: u32,
    y_value: u32,
    z_value: u32,
}

static DROPPED_COUNT: AtomicUsize = AtomicUsize::new(0); // Threads-Safe olarak veriyi kitlemeden (lock-free) değişikliğe izin vermek için AtomicUsize kullanıldı

impl Drop for Position {
    fn drop(&mut self) {
        println!("{} nolu position için Drop çağrısı", self.id);
        DROPPED_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

pub fn run() {
    let bump = Bump::new();

    { // Kasıtlı olarak scope açıldı
        let player_one = bump.alloc(Position {
            id: 1,
            x_value: 10,
            y_value: 20,
            z_value: 0,
        });
        let player_two = bump.alloc(Position {
            id: 2,
            x_value: 15,
            y_value: 5,
            z_value: 30,
        });
        let john_doe = bump.alloc(Position {
            id: 3,
            x_value: 3,
            y_value: 5,
            z_value: 8,
        });

        println!("Player One Adresi {:p}", player_one);
        println!("Player Two Adresi {:p}", player_two);
        println!("John Doe Adresi {:p}", john_doe);

        let player_one_addr = player_one as *const _ as usize;
        let player_two_addr = player_two as *const _ as usize;
        let john_doe_addr = john_doe as *const _ as usize;

        println!(
            "Gerçek Player Two - Player One adres farkı: {} byte",
            address_diff(player_two_addr, player_one_addr)
        );
        println!(
            "Gerçek John Doe - Player Two adres farkı: {} byte",
            address_diff(john_doe_addr, player_two_addr)
        );
    } // Scope dışındayız ama nesne drop'ları çalışmaz Area Allocation sebebiyle

    println!(
        "Dropped Position nesne sayısı {}",
        DROPPED_COUNT.load(Ordering::SeqCst)
    );

    // Arena burada scope'dan çıkarken içindeki tüm Player nesneleri de tek seferde düşürülecektir
}

fn address_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
```

Var olan örnek kodumuza birkaç ekleme yaptık. En önemlisi Poistion türü için **Drop** trait davranışını eklememiz. Hatta içerisinde **AtomicUsize** türünden bir sayaç kullanıyoruz. Eğer teorimiz doğruysa program sonlanırken scope dışında kalan **Position** nesneleri için Drop trait'inin çalışmaması ve dolayısıyla **DROOPED_COUNT** değişkeninin 0 olarak kalması gerekiyor. Kendi yaptığım çalışmada bu sonuca ulaştığımı söyleyebilirim. 

![bumpalo_runtime](../images/bumpalo_runtime.png)

Dokümantasyona göre Bumpalo söz konusu bellek bölgelerini kendisi oluşturup yönetmekte ve toplu serbest bırakma _(ya da Batch Deallocation)_ işlemi icra etmekte. Yani tek tek nesneleri drop etmek yerine ayrılan tüm bellek bloğu için tek seferde boşaltma işlemi uygulamakta. İşte bu noktada Rust'ın RAII modelini ezdiği düşünülebilir ki bu normaldir zira kütüphane stack yerine kendi bellek bölgesini yönetir. Aslında burada Drop için de bir parantez açmak lazım. Aynı Position veri yapılarını heap üzerinde tahsis ederek deneyelim. İşte örnek kod,

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
struct Velocity {
    id: u32,
    value: u32,
    direction: i32,
}

static DROPPED_COUNT: AtomicUsize = AtomicUsize::new(0); // Threads-Safe olarak veriyi kitlemeden (lock-free) değişikliğe izin vermek için AtomicUsize kullanıldı

impl Drop for Velocity {
    fn drop(&mut self) {
        println!("{} nolu Velocity nesnesi için Drop çağrısı", self.id);
        DROPPED_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

pub fn run() {
    { // Kasıtlı olarak Scope açıldı
        let _p1 = Box::new(Velocity {
            id: 1,
            value: 10,
            direction: 1,
        });
        let _p2 = Box::new(Velocity {
            id: 2,
            value: 20,
            direction: -1,
        });
        let _p3 = Box::new(Velocity {
            id: 3,
            value: 50,
            direction: 1,
        });
    } // Drop'lar çalışır

    println!(
        "Dropped Velocity nesne sayısı {}",
        DROPPED_COUNT.load(Ordering::SeqCst)
    );
}
```

Bu sefer Velocity isimli struct'lardan birkaç nesne örnekliyoruz ancak dikkat edileceği üzere bunların Box ediyoruz yani Heap üzerine yerleştiriyoruz. Buna göre Drop trait'lerinin otomatik olarak çalışması ve sayaç değerimizin de 3 olması gerekiyor. İşte sonuçlar.

![boxing_runtime](../images/boxing_runtime.png)

### AtomicUsize Kullanımı

Yukarıdaki son iki örnek kodda **AtomicUsize** veri türünü kullandık. Normalde statik değişkenler mutable olamazlar zira bunlar Global değişken olarak tanımlanır ve **eşzamanlı _(concurrent)_** erişilme riskleri vardır. AtomicUsize bu tip bir değişkenin çoklu thread'lerde **kilitlemeden _(lock-free)_** ve **thread-safe** olarak değiştirilmesine izin verir. Tabii bunun yerine **Mutex< Usize >** şeklinde bir kullanıma da gidilebilir. Karmaşık atomik işlemler gerektirmediğinden tercih edilebilr ama kilitleme gerektirir ve bunun bir maliyeti vardır, ayrıca **Deadlock** oluşma ihtimali de söz konusudur. Alternatif olarak **RwLock _(Read-Write Lock)_** belki kullanılabilir. Bu eşzamanlı okumları thread-safe icra edereken sadece yazma söz konusu olduğunda kilitleme yapar. Dokümanlara göre bazı durumlarda okuma ve yazma işlemlerinin çakışmasının söz konusu olabileceği belirtiliyor diğer yandan çok fazla yazma işlemi söz konusu ise kilitleme yine bir dezavantaj olarak karşımıza çıkar. Bu arada çoklu thread söz konusu değilse kilitleme mekanizmasına veya atomik işlemlere gerek kalma ve **Cell**, **RefCell** gibi türler kullanılabilir. Çok alternatif var değil mi? :D Kıyaslama için şöyle bir şeyle karalayabiliriz tabii ama güncelliğini kontrol etmekte yarar var. Bir sayaç kullanmak istediğimizi düşünelim;

- Tek bir thread söz konusu ise Cell veya RefCell< Usize > iyi bir çözümdür.
- Çoklu thread kullanımı söz konusu ve basit bir kullanım isteniyorsa Mutex< Usize > pekala iyi bir çözüm olacaktır.
- Çoklu thread'ler tarafında çokça okuma söz konusu ama nadir yazma işlemi varsa RwLock< Usize > ideal çözümdür.
- Çoklu thread erişimi, sıkça yazma ve performans kritik bir işleyiş gerekiyorsa AtomicUsize kullanılmalıdır.

Aslında bu söylediklerimiz ışığında **AtomicUsize** çok daha iyi bir seçim gibi görünebilir ama dezavantajları da vardır. Kilit kullanmaması her zaman hız demek değildir zira işlemci tarafında bellek bariyeri oluşturması söz konusudur. Ayrıca örnekte **Sequentially Consistent** kullandık dolayısıyla işlemci tüm diğer işlemleri durdurabilir sırf sayacı artıracağım diye ki bu da beklenmedik performans kaybına neden olabilir. Tabii bu sık kullanılmayla da ilgilidir. Diğer yandan AtomicUsize basit bir veri türüdür _(ki farkı varsayonları var Atomic kelimesi ile başlayan Bool, I16, I32, I64, I8, Isize, Ptr, U16, U32, U64, U8 gibi)_ ve hatta tek bir değişken üzerinde garanti sonuçlar verebilir. Birden fazla **AtomicUsize** kullanımı **Race Condition** problemini doğurabilir. Örnekte birde Ordering kullandık ki bunu belirtmemiz gerekiyordu. Hatta Drop trait içindeki ile aynı değeri de vermiştik. **Relaxed**, **Acquire**, **AccRel** gibi farklı değerler de verilebilir ve bunların kombinasyonu da çok önemli. Detaylar için [şurada bir tablo var](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.compare_and_swap)

## Struct/Enum Türlerinde Padding ve Allignment

Bir struct belleğe açıldığında alanları _(fields)_ nasıl yerleştiriliyor hiç düşündünüz mü? Ya da bir enum sabitinin alanları. Normal şartlarda alanların düzenli bir sırada hizalanması _(alignment)_ ve alanlar arasında sadece gerektiği kadar boşluk bırakılması _(padding minimizasyonu deniyor)_ bu veri yapısına ulaşan program parçaları için kolay ve hızlı erişilebilirlik anlamına gelir. Rust genellikle bu tip ayarlamaları bizim yerimize zaten yapar ancak bazı hallerde, örneğin FFI _(Foreign Function Interface)_ hattı üzerinde harici C kütüphaneleri ile çalışıldığında belki bu ayarlamaları elle yapmak gerekebilir.

Bu bilgiye ek olarak rust derleyicisinin **niche optimization** _(Friedrich Nietzsche' in nişi değil :P)_ adı verilen bir tekniği kullanarak bazı **enum** türlerini bellek açısından verimli hale getirdiği de ifade ediliyor. Option<u32> türünü ele alalım. Pozitif sayılardan oluşan bu 32bitlik değişken bellekte 8 byte yer kaplar _(4 byte içerdiği değer için + 4 byte None olma hali için)_ Zira u32 için **None** durumunu ifade etmek ek bir flag gerektirmektedir. Lakin **NonZeroU32** da kullanabiliriz. NonZeroU32, 0 hariç tüm 32-bit değerleri taşıyabilir ve 0 değeri **None** durumunu ifade etmek için kullanılır. Bu durumda Option< NonZeroU32 > sadece 4 byte yer kaplar; None değeri altta yatan 0 değeriyle temsil edilir, Some(value) ise value’nun kendi değerleriyle temsil edilir​. Daha fazla etay için [buradaki blog yazısını](https://www.0xatticus.com/posts/understanding_rust_niche/) ziyaret edebilirsiniz. Ben öğrendiklerimle aşağıdaki kodu tatbik etmeye çalıştım.

```rust
use std::num::NonZero;
use std::num::NonZeroU32;

pub fn run() {
    println!("Baştan söyleyelim...");
    println!("u32 {} byte yer tutar", size_of::<u32>());
    println!(
        "Option<u32> ise {} byte yer tutar. Diğer 4 byte None içindir.",
        size_of::<Option<u32>>()
    );
    println!("NonZero32 {} byte yer tutar", size_of::<NonZeroU32>());
    println!(
        "Option<NonZero32> ise yine {} byte yer tutar. Zira 0, None olarak ifade edilir.",
        size_of::<Option<NonZeroU32>>()
    );

    let nan = give_me_a_none();
    match nan {
        None => println!("There is no spoon!"),
        Some(v) => println!("{}", v),
    }

    let transmuted: u32 = unsafe { std::mem::transmute(nan) };
    println!("NonZeroU32 için None : {transmuted:b}");

    let nan = give_me_another_none();
    match nan {
        None => println!("There is no spoon!"),
        Some(v) => println!("{}", v),
    }

    let transmuted: u64 = unsafe { std::mem::transmute(nan) };
    println!("U32 için None : {transmuted:b}");

    let number = give_me_a_number();
    match number {
        None => println!("There is no spoon!"),
        Some(v) => println!("{}", v),
    }

    let transmuted: u32 = unsafe { std::mem::transmute(number) };
    println!("NonZero için Number 23 : {transmuted:b}");

    let number = give_me_another_number();
    match number {
        None => println!("There is no spoon!"),
        Some(v) => println!("{}", v),
    }

    let transmuted: u64 = unsafe { std::mem::transmute(number) };
    println!("U32 için Number 23 : {transmuted:b}");
}

fn give_me_a_none() -> Option<NonZeroU32> {
    NonZero::new(0)
    // None
}

fn give_me_another_none() -> Option<u32> {
    None
}

fn give_me_a_number() -> Option<NonZeroU32> {
    NonZero::new(23)
}

fn give_me_another_number() -> Option<u32> {
    Some(23)
}
```

**NonZeroU32** kullandığımız durumlarda None ve gerçek bir sayının bellekteki binary tutuluş şekilleri çok farklı dikkat edeceğiniz üzere. Örneğin None bilgisi **NonZerou32** için sadece 0 ile ifade edilirken **U32** kullanıldığında çok daha uzun bir içerik söz konusu. Çalışma zamanı çıktısını şöyle ifade edebiliriz.

![Niche Optimization](../images/niche_opt.png)

Elbette hangisinin hangi durumlarda kullanabiliriz sorusu ortaya çıkıyor değil mi? Belki gözden kaçırmış olabiliriz ama şöyle bir durum var. NonZeroU32 adı üstünde 0 değerini taşıyamaz. Sıfır değerini None olarak kabul eder. Bu nedenle yaygın görüş U32'nin kullanıldığı bir senaryoda hiçbir şekilde 0 değerinin kullanılmayacağı garanti ise NonZerou32 tercih edilebilir zira her bir sayısal değer için 8 byte yerine 4 byte ayırabiliriz. 

Niche _(niş)_ optimizasyonu denmesinin bir nedeni de None yerine geçebilecek ayrıcalıklı yani niş bir değeriniz bulunmasıdır. U32 senaryosunda 0'dan feragat edilmesi garanti ise 0 bir niş değer olarak ifade edilir ve None yerine kullanılır ve bu da size büyük bir bellek tasarrufu olarak dönebilir.

Bir örnek daha verelim. Özellikle referans kullanılan senaryolarda niş optimizasyon da ele alınabilir. **None** durumu için **null pointer (0x0)** kullanıldığından tahsis edilen bellek miktarı aynıdır.

```rust
println!("&u32 türü için de {} byte yer ayrılır.", size_of::<&u32>());
println!(
    "ve Option<&u32> içinde {} byte söz konusudur.",
    size_of::<Option<&u32>>()
);
```

![Niche Optimization Two](../images/niche_opt_2.png)

## Memory/Object Pooling

Geliştirmekte olduğumuz uygulamaların heap üzerindeki yerleşimleri hem performans hem de verimlilik açısından önemlidir. Geniş arazilere sahip heap üstünde rastgele nesne yerleşimleri zamanla fragmantasyonu bozabilir, uygulamanın bellekte kapladığı alan şişebilir, reaksiyon süreleri yavaşlayabilir. Esasında Rust, RAII _(Resource Acquisition is Initialization)_ ilkesine bağlı kalarak nesne ömürlerini optimal seviyede yönetmeye. _(Her ne kadar lifetimes mevzusu zaman zaman zorlayıcı olsa da)_ Ancak yine de tekrar tekrar üretilen pahalı nesneler söz konusu olduğunda pekala bunların heap organizasyonu endüstriyel diyebileceğimiz kimi standart metodolojilerle ele alınabilir. Object Pooling' de bunlardan birisidir. Örneğin veri tabanı bağlantıları, oyun motorlarındaki asset yöneticileri sürekli ve tekrar tekrar kullanılan nesneler olarak karşımıza çıkarlar. Bunları belli limitlere sahip bir havuzdan yönetmek pekala ideal olabilir.

Aşağıdaki örnek kod parçasında çok basit bir implemantasyonuna yer verilmektedir.

```rust
use std::sync::Arc;
use std::sync::Mutex;

trait Identifiable {
    fn get_id(&self) -> i16;
}

struct AssetServer {
    id: i16,
}

impl AssetServer {
    fn new(value: i16) -> Self {
        AssetServer { id: value }
    }
}

impl Identifiable for AssetServer {
    fn get_id(&self) -> i16 {
        self.id
    }
}

// Havuzdaki nesneleri tutan veri yapısı
// Generic T türü ile çalışır (T'yi Identifiable olma koşulan zorlayacak generic constraint eklenebilir)
struct ObjectPool<T> {
    objects: Arc<Mutex<Vec<T>>>, // T anında sadece bir thread'in erişimini garanti etmek için Atomic Reference Counted ve Mutex kullanılmıştır.
}

impl<T> ObjectPool<T> {
    pub fn new() -> Self {
        ObjectPool {
            objects: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Ekleme, çekme ve serbest bırakma operasyonlarının tamamında Mutex lock kullanılır
    pub fn add(&mut self, value: T) {
        self.objects.lock().unwrap().push(value);
    }

    pub fn get(&mut self) -> Option<T> {
        let mut objects = self.objects.lock().unwrap(); // Havuzdaki nesneler kilit konularak çekilir
        if objects.len() > 0 {
            // Eğer havuzda nesne varsa
            return objects.pop(); // sonraki nesne verilir
        }
        None
    }

    // Bir nesne ile işimiz bittiğinde onu havuza tekrardan yerleştirmek için kullanılan fonksiyondur
    // Bu arada releae metodu yerine drop trait implementasyonu da tercih edilebilir
    pub fn release(&mut self, value: T) {
        self.objects.lock().unwrap().push(value);
    }
}

pub fn run() {
    let mut asset_pool: ObjectPool<Box<dyn Identifiable>> =
        ObjectPool::<Box<dyn Identifiable>>::new();

    for i in 0..5 {
        asset_pool.add(Box::new(AssetServer::new(i)));
    }

    let server_1 = asset_pool.get().unwrap();
    println!("Server 1 id {}", server_1.get_id());
    let server_2 = asset_pool.get().unwrap();
    println!("Server 2 id {}", server_2.get_id());
    asset_pool.release(server_2);

    for object in asset_pool.objects.lock().unwrap().iter() {
        println!("Asset server id: {}", object.get_id());
    }
}
```

Yukarıdaki kodda çok basit ve ilkel bir object pool mekanizması uygulanmaya çalışılıyor. Teorik olarak heap maliyeti yüksek nesnelerin bir havuzdan karşılanması değerlendirilmekte. **AssetServer** yapısını yüksek maliyetli bir nesne gibi düşünebiliriz. **ObjectPool** isimli veri yapısı **Arc _(Atomic reference counted)_** isimli **smart pointer**'ı ve **Mutex** mekanizmasını kullanarak bu havuzu yönetiyor. Havuza nesneler ekleyebiliyoruz ve release metodunu çağırdığımızda tekrardan havuza dönmelerini sağlayabiliyoruz. Ancak bu çok basit ve ilkel bir implementasyon. release yerine drop trait implementasyonu kullanılabilir ve nesne scope dışında kaldığında tekrardan havuza iade edilebilir. Havuzda hiç nesne yoksa ilk initialize aşamasında bir tane eklenebilir. Bir üst yapı ile havuz yönetimi ele alınabilir. Örneğin nesnelerin belli sayıda ve belli süre boyunca scope' da kalmaları sağlanabilir belki de.

Çok doğal olarak kendi implementasyonumuz dışında kullanabileceğimiz hazır küfeler de _(crate)_ bulunuyor. En bilinenleri [lockfree-object-pool](https://crates.io/crates/lockfree-object-pool) ve [typed_arena](https://crates.io/crates/typed-arena). Aşağıdaki örnekte typed_arena kullanımına ait bir örnek yer almakta.

```rust
use typed_arena::Arena;

#[derive(Debug)]
struct AssetServer {
    assets: Vec<String>,
    id: u32,
}

pub fn run() {
    let arena = Arena::new();

    let server_1 = arena.alloc(AssetServer {
        assets: vec![
            "player.png".to_string(),
            "tileSet.png".to_string(),
            "colors.png".to_string(),
        ],
        id: 1234,
    });

    let server_2 = arena.alloc(AssetServer {
        assets: vec![
            "human.png".to_string(),
            "brick.png".to_string(),
            "block.png".to_string(),
            "juice.jpg".to_string(),
            "intro.wav".to_string(),
        ],
        id: 1255,
    });

    println!(
        "Server {} has {} assets",
        server_1.id,
        server_1.assets.len()
    );
    println!(
        "Server {} has {} assets",
        server_2.id,
        server_2.assets.len(),
    );
}
```

Pek tabii bu soyutlama çok daha basit bir kullanıma sahip. Bu arada object pooling, arena allocator başlığı altında incelediğimiz tek bir tampon yaklaşımını da kullanabilir. Örneğin **typed_arena** küfesi tekil **deallocation** işlevleri yerine kapsam dışına çıkıldığında tüm bloğu düşüren ve stack based yerine heap based çalışan bir kütüphanedir. Tipik bir Arena Allocator taktiği uyguluyor diyebiliriz. 

**Object Pooling** mevzusunda dikkat çeken noktalardan birisi de havuza nesneler eklendikçe heap' in şişmesidir. Bunu yönetmek de bir tasarım gerektirir. typed_arena gibi Arena Allocator stratejisi güden kütüphaneler genellikle tek bir bellek bölgesi ayırmayı tercih eder, tek seferde bellekten düşürür, kapasite dolara bundan bağımsız yeni bir bellek bölgesi daha tahsis eder. Dolayısıyla kendi yazdığımız senaryolarda bu kapasiteyi yönetmemiz de gerekebilir. İlk yazdığımız kodu bu anlamda değerlendirip aşağıdaki gibi yeniden düzenleyebiliriz.

```rust
struct ObjectPool<T> {
    objects: Arc<Mutex<Vec<T>>>,
    capacity: usize,
}

impl<T> ObjectPool<T> {
    pub fn new(capacity: usize) -> Self {
        ObjectPool {
            objects: Arc::new(Mutex::new(Vec::new())),
            capacity,
        }
    }

    pub fn add(&mut self, value: T) {
        if self.objects.lock().unwrap().len() < self.capacity {
            self.objects.lock().unwrap().push(value);
        }
    }

    pub fn get(&mut self) -> Option<T> {
        let mut objects = self.objects.lock().unwrap();
        if objects.len() > 0 {
            return objects.pop();
        }
        None
    }

    pub fn release(&mut self, value: T) {
        if self.objects.lock().unwrap().len() < self.capacity {
            self.objects.lock().unwrap().push(value);
        } else {
            println!("Pool is full");
        }
    }
}
```

ObjectPool yapısına usize türünden capacity alanı eklenmiştir. add ve release fonksiyonlarında kapasite kontrolü yapılır. Bu yöntem havuzdaki nesne sayısını sabitler ve heap'in gereksiz yere şişmesini de engeller ancak dezavantajları da vardır. Örneğin havuzda boş yer yokken bir nesne istersek yeni nesne üretemeyebiliriz. Bu da bizi **Eviction** stratejilerine götürür. **Least Recently Used _(LRU)_**, **Time-Aware Least Recently Used _(TLRU)_**, **Least Frequently Used _(LFU)_**, **Most Recently Used _(MRU)_** gibi. Bunlar sadece object pooling değil cache mekanizmaları içinde ele alınan stratejilerdir. 

- **LRU**' da amaç kullanılmayan nesneleri belli bir sıraya göre havuzdan çıkarmaktır ve genellikle web cache, session cahce, oyunlarda asset yönetimi gibi senaryolarda ele alınır. Burada en az kullanılan nesneye erişmek Big O ölçümlemesine göre nispeten yavaş olabilir ve O(1) durumuna getirilmesi gerekebilir. 
- **TLRU**' da bir zaman damgası kullanılır ve buna göre belli süre kullanılmayan nesnelerin havuzdan çıkartılması sağlanır. Tahmin edileceği üzere amaç belli süre kullanılmayan nesneleri temizlemek bunu da bir zamanlayıcıya göre ayarlamaktır. Örneğin ömrü 60 saniyeden eski olanları temizle demek gibi. Geçici dosya yönetiminde, IoT sistemlerde ele alınabilir. 
- **LFU**' da amaç en az kullanılan ya da daha iyi bir tarifle en az başvurulan nesneyi havuzdan çıkarmaktır. Böylece sık kullanılan nesneler havuzda kalırken az kullanılanlar çıkarılır. Makine öğrenmesi'nde cache kullanılacağı zaman ve DB indeklemede ele alınır. Sık kullanılan nesnelerin havuzda kalması önemli bir avantajdır ancak bu nesnelerin aranması bulunması tam bir O(N) maliyetine eş olabilir.
- **MRU** ise en son kullanılan nesnenin bellekten çıkartılmasını hedefler. Veri sıkıştırma algoritmaları ve büyük dosya sistemlerinde kullanılan bir yöntem olduğu belirtilmektedir. LRU'nun tam tersi olarak da değerlendirilebilir. 

Tabii Object Pooling dedik, sonra havuz kapasitesini nasıl yöneteceğiz dedik ve kendimizi cache stratejilerinin uygulanmasında bulduk. Bu nedenle eğer sıfırdan bir object pool mekanizması tasarlamayacaksak bunu soyutlayan crate'lerden yararlanmak daha iyi olabilir.

## Cache Aware Programming

_"NOT YET IMPLEMENTED"_

## Zero Cost Abstraction

Sıfır maliyetli soyutlamalar olarak çevirebileceğimiz bu kavram Rust'ın güçlü olan özelliklerinden birisidir. Bunu kısaca ifade etmek gerekirse, **iterator** metotları, **generic** tür kullanımları, **trait** implementasyonları gibi yaklaşımların çalışma zamanı maliyetlerinin sıfır olmasıdır/sıfıra yakın olmasıdır diye açabiliriz _(çok kesinlik katamadım ama iddialar bu yönde)_ Hatta idiomatic yazılmış bir rust kodunun sanki **C/C++** dilleri ile yazılmış kadar efektif olduğu belirtilir. Özellikle koleksiyonlar arkasından gelen iterator fonksiyonları için derleme zamanında üretilen kodun çalışma zamanında ek bir maliyet gerektirmeden yüksek performanslı çalıştığını belirten bir kavram olarak da tanıtılır. Generic türler açısından düşündüğümüzde Rust'ın **monomorfizasyon** tekniğiyle somut tiplere özel kodlar ürettiğini söyleyebiliriz. Bu yaklaşım **C++** dilinde **template** enstrümanına benzer biçimde her tip kullanımı için ayrı ve optimize edilmiş kod üretilmesi ile aynıdır. Bu durumda generic bir metot çağrımı ile normal metot çağrımı arasında pek bir performans farkı kalmaz. Iterator fonksiyonlar demişken aşağıdaki basit kod örneği ile bu durumu tarifleyebiliriz.

```rust
pub fn run() {
    let numbers: Vec<u32> = (0..=10).collect();

    let total_sum_1: u32 = numbers.iter().map(|x| x + 1).sum();

    let mut total_sum_2: u32 = 0;
    for x in &numbers {
        total_sum_2 += x + 1;
    }

    println!("Iterator fonksiyonları üzerinden toplam : {}", total_sum_1);
    println!("Klasik for döngüsü ile toplam : {}", total_sum_2);
}
```

Birden ona kadarki sayıların toplanması iki farklı şekilde yapılmaktadır. Birisinde iter() metodu üzerinden ulaşılıp map ve sum fonksiyonları ile hesaplama yapılmaktadır. Diğeri ise bildiğimiz klasik bir for döngüsü yardımıyla yapılandır. Kod okunurluğu açısından iterator kullanımı çok daha ideal duruyor ve aslında birçok fonksiyonel dilde bu tip yüksek seviyeli metotlara rastlıyoruz ancak bu tip bir soyutlamanın çalışma zamanı maliyetleri dile göre tartışılır. **Zero-Cost Abstraction**, klasik for döngüsü ile yazılan ve optimize edilmiş kod çıktısının iterator fonksiyonlar için de söz konusu olduğunu söyler. 

Peki bu nasıl ispat edilebilir? Sanıyorum doğru optimizasyon seviyesinde üretilen kodun **assembly** çıktılarına bakarak bir kıyaslama yapmak lazım. Bu henüz tam olarak gerçekeştiremediğim bir şey ancak yol boyunca öğrendiğim bazı şeyler de oldu. Örneğin **src** klasöründe **zca.rs** isimli yalnız bir rust dosyası var. Bunu **rustc** ile deledikten sonra assembly koduna bakabiliriz.

```bash
# Bu komut zca.s isimli Assembly kodlarından oluşan zca.s dosyasını üretecektir
rustc --emit=asm -C llvm-args="--x86-asm-syntax=intel" zca.rs

# Sadece main fonksiyonunun karşılığını görmek içinse aşağıdaki komutu kullanabiliriz
grep -A 20 "main:" zca.s
```

Aşağıdakine benzer bir çıktı üretmesi muhtemeldir. En azından bende böyle üretti.

```assmebly
main:
        .cfi_startproc
        push    rax
        .cfi_def_cfa_offset 16
        mov     rdx, rsi
        movsxd  rsi, edi
        lea     rdi, [rip + _ZN3zca4main17h95f2e46fab2a6266E]
        xor     ecx, ecx
        call    _ZN3std2rt10lang_start17h9d55bcc39d9e49f1E
        pop     rcx
        .cfi_def_cfa_offset 8
        ret
.Lfunc_end63:
        .size   main, .Lfunc_end63-main
        .cfi_endproc

        .type   .L__unnamed_20,@object
        .section        .rodata..L__unnamed_20,"a",@progbits
.L__unnamed_20:
        .ascii  "capacity overflow"
        .size   .L__unnamed_20, 17
```

Ancak sahip olduğum kıt assembly bilgim ile bir sonuca varamadım diyebilirim _(Henüz)_ Diğer yandan cargo ile üretilmiş projelerde [şöyle bir crate](https://crates.io/crates/cargo-show-asm) kullanabileceğimiz de belirtiliyor. Bu aracı kullanarak kodun assembly çıktılarına bakmak ve okumak daha kolay diyebilirim.
