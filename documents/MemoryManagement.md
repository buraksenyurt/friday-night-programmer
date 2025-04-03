# Bellek Yönetimi Üzerine Notlar

**Unmanaged** ortamlarda gezinmek birçok yeni veya unutulmuş bilgiyi de karşıma çıkarıyor. Geçtiğimiz günlerde devasa boyutlarda **JSON** tabanlı logları işlerken **Interning** stratejisi ile belleğe alınan büyük boyutlu veri kümesinin nasıl optimize edilebileceğini öğrendim. Belli senaryolarda _(her zaman da avantajlı olmayabiliyor)_ çok sık tekrar eden **string** içerikler için heap bölgesinde tahsisat yapılırken gereksiz alan ayırmak yerine, bunları referans eden tekil pointer'lardan yararlanmak ve hatta benzersiz sayısal değerlerle _(örneğin pozitif bir tam sayı ile)_ bir vektör içerisinde tutup _(Intern havuzu olarak da ifade ediliyor)_ erişimi hızlandırmak mümkün. Tam anlamıyla bellek seviyesinde optimizasyon ve performans kazanımı söz konusu. Biraz karışık bir cümle oldu ama kaynak olarak sunacağım [şu yazının uzunluğu](https://gendignoux.com/blog/2025/03/03/rust-interning-2000x.html) düşünülünce elden bu kadar geldi. Yazıda Paris'in herkese açık ulaşım verilerinden yararlanılıyor. Veriler çok büyük ve yazarın iddiasına göre 2bin kata kadar küçültülebiliyor. Örneğin sadece **String** veriler üzerine yapılan **Interning** tekniğinin **%47** oranında yer kazanımı sağladığı ifade ediliyor. Tabii olay bellek yönetimi, bellek operasyonlarında optimizasyon ve performans işlemleri denilince karşımıza çıkan daha birçok konu var. Örneğin **Region-Based Management** konseptinde yer bulan **Area Allocators**, **Copy on Write(CoW)**, **Zero Cost Abstraction**, **Memory/Object Pooling**, **Cache-Aware Programming**, **Enum'larda Padding ve Allignment** kullanımı gibi

Bu yazıda ilgili kavramlar aşağıdaki konu başlıkları çerçevesinde ele alınmakta.

- [Bellek Yönetimi Üzerine Notlar](#bellek-yönetimi-üzerine-notlar)
    - [Cow - Copy on Write/Clone on Write](#cow-copy-on-write)
        - [Dirty Cow Mevzusu](#dirty-cow-mevzusu-pis-i̇nek-p)
    - [Arena Allocators](#arena-allocators)
        - [AtomicUsize Kullanımı](#atomicusize-kullanımı)
    - [Struct/Enum Türlerinde Padding ve Allignment](#structenum-türlerinde-padding-ve-allignment)
    - [Memory/Object Pooling](#memoryobject-pooling)
    - [Cache-Friendly Programming](#cache-friendly-programming)
    - [Zero Cost Abstraction](#zero-cost-abstraction)

Rust, zaten sahip olduğu bazı özellikleri ile _(Ownership, borrow-checker- lifetimes vb)_ belleği güvenli noktada tutmak için .çeşitli imkanlar sağlıyor _(Bilindiği üzere Rust'ta bir Garbage Collector mekanizması yok)_ Yine de bazı senaryolarda belleği efektif kullanmak için az önce saydığım yaklaşımlara da değinmek gerekiyor. Söz gelimi **Interning** konusunda yazılmış duruma göre tercih edilebilecek birçok yardımcı **crate** mevcut.

## CoW _(Copy on Write)_

Sadece gerektiği zaman veri kopyalanmasını öneren bir teknik olduğu ifade ediliyor. Bunu şöyle de ifade edebiliriz; Koypalama işlemini ilk yazma anına kadar ötelemek. Rust içerisinde bu felsefe ile donatılmış **Cow** isimli bir **enum** türü bulunuyor _(ki bu ayrıca bir Smart Pointer türüdür)_ Diğer yandan resmi kaynaklarda açılımı **Clone on Write** şeklindedir. Bu enum türü içerdiği veriyi ya ödünç alır _(borrow)_ ya da yeni bir kopyası ile sahipliğini alır _(ownership)_ Dolayısıyla veriyi değiştirme ihtiyacı yoksa referansını kullanır ama değişiklik gerekirse de ilk değişiklik gerektiği noktada verinin bir klonu hazırlanıp sahipliği ele alınır. Bu noktada aşağıdaki örnek kod parçasını göz önüne alabiliriz.

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

Burada neler oluyor bir değerlendirelim. Son derece anlamsız bir fonksiyon olan **padding_end**, parametre olarak gelen literalın sonuna **target_len** ile belirtildiği kadar **_** işareti ekliyor ancak gelen içeriğin uzunluğu zaten o kadar ise eklemiyor. Yalnız bu ekleme ve eklememe arasında önemli bir fark var. İfade sonunda boşluklar kalmışsa yeni bir **String** değer oluşturuluyor ve sahipliği fonksiyondan geriye döndürülüyor _(Owned çağrısı)_ . Diğer durumda ise orjinal referans döndürülüyor _(Borrowed çağrısı)_ 

**Copy on Write** metodolojisi işletim sistemlerinde ortak **page** kullanımlarında ya da **immutable** veritabanı yapılarında yaygın olarak kullanılmakta. Gözle görmedim o yüzden ispat edemem ancak Rust'ın **Clone on Write** özelliğinin daha çok programlama dili ile ilgili olduğu da belirtilmekte. Genel ve yaygın adı **Copy on Write** olsa da Rust tarafındaki **CoW** doğrudan mutasyona izin vermediği için daha çok **Clone on Write** olarak ifade edilmektedir ama özünde benzer bir felsefeye sahip olduklarını ifade edebiliriz.

Konuyu pekiştirmek amacıyla farklı bir örnekle devam edelim.

```rust
fn remove_ellipsis_dots(input: &str) -> String {
    input.to_string().replace('`', "")
}
```

Yukarıdaki fonksiyon gelen literal içerisindeki üsten virgül işaretlerini bulup kaldırıyor. Burada yeni bir **String** türünün oluşturulması söz konusu ve bu her seferinde yapılıyor. Bir başka deyişle gelen input içerisinde üstten virgül işaret yoksa bile bir **String** üretimi söz konusu. İşte tam bu noktada eğer üstten virgül yoksa aynı referansın kullanılmaya devam edilmesi sağlanabilir. Bunu Cow türünü kullanarak aşağıdaki gibi gerçekleştirebiliriz.

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

Veri manupilasyonu sadece gerektiği zamanlarda yapılmış olur.

### Dirty Cow Mevzusu _(Pis İnek :P)_

**Dirty Cow** olarak isimlendirilen [bir güvenlik zafiyeti](https://cve.mitre.org/cgi-bin/cvename.cgi?name=cve-2016-5195) var. **Copy on Write** 'ın yanlış kullanımı neticesinde salt okunur veri alanına çeşitli ayrıcalıklar _(privilages)_ eklenmesi sonucu böyle bir zafiyet oluşmuş ve 2018 öncesi sürüme sahip birçok **Linux** sistemi durumdan etkilenmiş. Aslında durumun vehameti [Wikipedia' da](https://en.wikipedia.org/wiki/Dirty_COW) özetlenmiş. Söz konusu güvenlik zafiyetine konu olan **Copy on Write** stratejisinin Rust tarafındaki **CoW** kullanımı ile bir ilişkisi olmayabilir zira Rust’taki Cow, Linux çekirdeği seviyesindeki Copy-on-Write mekanizmasıyla aynı değildir. Rust’taki Cow türü tamamen kullanıcı seviyesi _(user-space)_ işleyen bir veri yapısıdır ve işletim sistemi seviyesinde process bazlı sayfa _(page)_ paylaşımı yapmaz. Tabii söz konusu güvenlik zafiyeti **Data Races** oluşması halinden, doğru zamanlama ile salt okunur bellek sayfalarına ayrıcalıklar _(privilages)_ eklenmesinden ve uzaktan root kullanıcı ayrıcalıklarına sahip olunabilmesinden bahseder ki bunlara sebebiyet veren durumlar Rust'ın **ownership**, **borrow checker** gibi mekanizmaları sayesinde henüz derleme aşamasından engellenir. Yine de **unsafe** alanlarda çalışırken veya **Arc** kullanımlarında dikkatli olmakta yarar var.

## Arena Allocators

**N sayıda** nesne için bellekte baştan bir yer ayırıp _(allocate)_ bunları bu alanda toplamak ve sonrasında hepsini tek seferde düşürmek _(deallocate)_ istediğimiz senaryolarda kullanılan bir teknik olarak ifade edilebilir. Temel çalışma prensibine göre program ayağa kalkarken bellekte belli bir bölge bu iş için tahsis edilir ve gerekli nesneler söz konusu alana ardışıl olarak yerleştirilir. Bölgenin serbest bırakılması nesnelerin de topluca ve tek seferde bellekten düşürülmesi anlamına gelir. Bir **arena** oluşturulduğunda işaretçi _(pointer)_ başlangıç konumuna alınır ve diğer nesnelere ardışıl olarak ulaşılması da kolaylaşır ki bunun da performans açısından önemli bir artısı olduğu söylenebilir. Hatta bu alanlar işletim sistemlerinin kullandığı ön belleklere de benzetilir. 

Bu stratejide tüm bellek bölgesinin tek seferde düşmesi en önemli noktalardan birisidir ancak zamanı geldiğinde tek tek düşürülmesi gereken nesneler söz konusu ise bunları **arena** içerisinde ele almak mümkün değildir ya da tam tersi nesneler topluca serbest kalırken yaşaması gerekenler varsa bu yöntem kullanışlı olmayacaktr. Bir başka deyişle aynı yaşam ömrüne _(life-time)_ sahip ya da birlikte sona eren ve çok büyük boyutlu olmayan nesnelerin organizasyonu için daha idealdir. Rust'ta bu amaçla kullanabilecek birçok **crate** mevcut. Bunlardan birisi de [bumpalo](https://crates.io/crates/bumpalo) ve işte basit bir kullanım örneği.

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

Örnekte **bump** nesnesi oluşturulduktan sonra içerisine üç farklı **Position** değişkeni ekleniyor. Sadece bu örnek özelinde değişkenleri kodlama sırası ile olmasa da ardışıl olarak dizildiklerini ispat edebilmek için bellek adresleri arasındaki farklar hesaplanıyor. **Position** yapısı 4 byte'tan oluşan 3 alan içermekte ve dolayısıyla 12 byte yer kaplamakta. Buna göre nesnelerin başlangıç adresleri arasında 12 byte mesafe olmalı. Elbette bunu çok daha büyük boyutlu bir nesne kümesi için kontrol etmek lazım, tam bir ispattır diyemeyiz.

 Bilindiği üzere **rust** programlama dili çalışma zamanında nesnelerin temizlenmesi için **Resource Acquisition is Initialization _(RAII)_** prensibini kullanır. Buna göre bir değer **scope** dışına çıktığında bellekten otomatik olarak düşer ve hatta referans türlü yani **heap** bazlı bir nesne ise _(Box edilmiş bir tür, Vector gibi)_ **drop trait** davranışı çalıştırılır. **Area Allocator** teorisinde ise bu bellek alanının tek seferde düşürülmesi söz konusu. Dolayısıyla bölgeye atılan nesneler için **drop** implementasyonlarının çalışmaması beklenir. **Bumpalo** kütüphanesi açısından bakarsak aşağıdaki örnek kodla durumu özetleyebilir.

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

Var olan örnek kodumuza birkaç ekleme yaptık. En önemlisi **Poistion** türü için **Drop** trait davranışını eklememiz. Hatta içerisinde **AtomicUsize** türünden bir sayaç da kullanıyoruz. Eğer teorimiz doğruysa program sonlanırken **scope** dışında kalan **Position** değerleri için **drop trait** 'inin çalışmaması ve dolayısıyla **DROOPED_COUNT** değişkeninin **0** olarak kalması gerekiyor. Kendi yaptığım çalışmada bu sonuca ulaştığımı söyleyebilirim. 

![bumpalo_runtime](../images/bumpalo_runtime.png)

Dokümantasyona göre **Bumpalo** kütüphanesi söz konusu bellek bölgelerini kendisi oluşturup yönetmekte ve toplu serbest bırakma _(ya da Batch Deallocation)_ işlemi icra etmekte. Yani nesneleri tek tek **drop** etmek yerine ayrılan tüm bellek bloğu için tek seferde boşaltma işlemi uygulamakta. İşte bu noktada Rust'ın **RAII** modelini ezdiği düşünülebilir ki bu normaldir zira kütüphane **stack** yerine kendi bellek bölgesini yönetir. 

Aslında burada **Drop** davranışı için de bir parantez açmak lazım. Aynı **Position** veri yapılarını **heap** üzerinde tahsis ederek ilerleyelim.

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

Bu sefer **Velocity** isimli **struct** 'lardan birkaç değişken tanımlıyoruz ancak dikkat edileceği üzere bunları **Box** ile kullanıyoruz. Bir başka deyişle kasıtlı olarak **Heap** üzerinde yerleşmelerini sağlıyoruz. Bu yeni durumda **Drop trait** 'lerinin otomatik olarak çalışması ve sayaç değerimizin de **3** olması gerekiyor. İşte sonuçlar.

![boxing_runtime](../images/boxing_runtime.png)

### AtomicUsize Kullanımı

Yukarıdaki son iki örnek kodda **AtomicUsize** veri türünü kullandık. Normalde **statik** değişkenler **mutable** olamazlar zira bunlar **global** değişken olarak tanımlanır ve **eşzamanlı _(concurrent)_** erişilme riskleri vardır. **AtomicUsize** tipi bir değişkenin çoklu thread'lerde **kilitlemeye gerek olmadan _(lock-free)_** ve **thread-safe** olarak değiştirilmesine izin verir. Tabii bunun yerine **Mutex< Usize >** şeklinde bir kullanıma da gidilebilir. Karmaşık atomik işlemler gerektirmediğinden tercih edilebilir ama kilitleme gerektirir ve bunun bir maliyeti vardır, ayrıca **Deadlock** oluşma ihtimali de söz konusudur. Alternatif olarak **RwLock _(Read-Write Lock)_** kullanılabilir. Bu tür eşzamanlı okumları **thread-safe** icra ederken sadece yazma söz konusu olduğunda kitleme yapar. Dokümanlara göre bazı durumlarda okuma ve yazma işlemlerinin çakışmasının söz konusu olabileceği belirtiliyor ancak diğer yandan çok fazla yazma işlemi söz konusu ise kilitleme yine bir dezavantaj olarak karşımıza çıkıyor. Bu arada birden fazla thread söz konusu değilse kilitleme mekanizmasına veya atomik işlemlere gerek yoktur ve **Cell**, **RefCell** gibi türler kullanılabilir. Çok alternatif var değil mi? :D Kıyaslama için şöyle bir şeyler karalayabiliriz tabii ama güncelliğini kontrol etmekte yarar var. Bir sayaç kullanmak istediğimizi düşünelim;

- Tek bir thread söz konusu ise **Cell** veya **RefCell< Usize >** iyi bir çözümdür.
- Çoklu thread söz konusu ve basit bir kullanım yeterliyse **Mutex< Usize >** pekala iyi bir çözüm olacaktır.
- Çoklu thread'ler tarafında bolca okuma ama nadir yazma işlemi söz konusuysa **RwLock< Usize >** ideal çözümdür.
- Çoklu thread erişimi, sık yazma operasyonu ve performans kritik bir işleyiş gerekiyorsa **AtomicUsize** kullanılması daha iyi olacaktır.

Aslında bu söylediklerimiz ışığında **AtomicUsize** çok daha iyi bir seçim gibi görünebilir ama dezavantajları da vardır. Kilit kullanılmaması her zaman hız avantajı sağlamaz zira işlemci tarafında bellek bariyeri oluşması söz konusudur. Ayrıca örnekte **Sequentially Consistent** kullandık, dolayısıyla işlemci sırf sayacı arttıracağım diye diğer işlemleri durdurabilir ki bu da beklenmedik performans kaybına neden olabilir _(Tabii bu sık kullanılmayla da ilgilidir)_ 

Diğer yandan **AtomicUsize** basit bir veri türüdür _(ki Atomic Atomic kelimesi ile başlayan Bool, I16, I32, I64, I8, Isize, Ptr, U16, U32, U64, U8 gibi farklı türler var)_ ve hatta tek bir değişken üzerinde garanti sonuçlar verebilir. Birden fazla **AtomicUsize** kullanımı **Race Condition** problemini doğurabilir. Örnekte birde Ordering kullandık ki bunu belirtmemiz gerekiyordu. Hatta **Drop trait** içindeki ile aynı değeri de vermiştik. **Relaxed**, **Acquire**, **AccRel** gibi farklı değerler de verilebilir ve bunların kombinasyonu önemlidir. Detaylar için [şurada bir tablo var](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.compare_and_swap)

## Struct/Enum Türlerinde Padding ve Allignment

Bir **struct** türü belleğe açıldığında alanları _(fields)_ nasıl yerleştiriliyor hiç düşündünüz mü? Ya da bir **enum** sabitinin alanları. Normal şartlarda alanların düzenli bir sırada hizalanması _(alignment)_ ve alanlar arasında sadece gerektiği kadar boşluk bırakılması _(padding minimizasyonu deniyor)_ bu veri yapısına ulaşan program parçaları için kolay ve hızlı erişilebilirlik anlamına gelir. **Rust** genellikle bu tip ayarlamaları bizim yerimize zaten yapar ancak bazı hallerde, örneğin FFI _(Foreign Function Interface)_ hattı üzerinde **harici C kütüphaneleri** ile çalışıldığında belki bu ayarlamaları elle yapmak gerekebilir.

Bu bilgiye ek olarak rust derleyicisinin **niche optimization** _(Friedrich Nietzsche' in nişi değil :P)_ adı verilen bir tekniği kullanarak bazı **enum** türlerini bellek açısından verimli hale getirdiği de belirtilir. **Option<u32>** türünü ele alalım. Pozitif sayılardan oluşan bu 32 bitlik değişken bellekte **8 byte** yer kaplar _(4 byte içerdiği değer için + 4 byte None olma hali için)_ Zira u32 için **None** durumunu ifade etmek ek bir **flag** gerektirmektedir. Lakin **NonZeroU32** türünü de kullanabiliriz. **NonZeroU32**, 0 hariç tüm 32-bit değerleri taşıyabilir ve 0 değeri **None** durumunu ifade etmek için kullanılır. Bu durumda **Option< NonZeroU32 > sadece 4 byte** yer kaplar; **None** değeri altta yatan **0** değeriyle temsil edilir, **Some(value)** ise value’nun kendi değerleriyle temsil edilir​. Daha fazla etay için [buradaki blog yazısını](https://www.0xatticus.com/posts/understanding_rust_niche/) ziyaret edebilirsiniz. Konuyu pekiştirmek için aşağıdaki örnekle devam edelim.

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

Dikkat edileceği üzere **NonZeroU32** kullandığımız durumlarda **None** ve gerçek bir sayının bellekteki **binary** formatlı saklanma şekilleri çok farklı. Örneğin None bilgisi **NonZerou32** için sadece **0** ile ifade edilirken **U32** kullanıldığında çok daha uzun bir içerik söz konusu.

![Niche Optimization](../images/niche_opt.png)

Doğal olarak hangisini hangi durumlarda kullanabiliriz sorusu ortaya çıkıyor? Belki gözden kaçırmış olabiliriz ama şöyle bir durum var. **NonZeroU32** adı üstünde **0** değerini taşıyamaz. Sıfır değerini **None** olarak kabul eder. Bu nedenle yaygın görüş U32'nin kullanıldığı bir senaryoda hiçbir şekilde 0 değerinin kullanılmayacağı garanti ise **NonZerou32** tercih edilebilir zira her bir sayısal değer için 8 byte yerine 4 byte ayırabiliriz. 

Niche _(niş)_ optimizasyonu denmesinin bir nedeni de **None** yerine geçebilecek ayrıcalıklı yani niş bir değerin bulunmasıdır. **U32** senaryosunda 0 değerinden feragat edilmesi garanti ise 0 bir niş değer olarak ifade edilir ve None yerine kullanılır ve bu da bize büyük bir bellek tasarrufu olarak döner.

Bir örnek daha verelim. Referans kullanılan senaryolarda da niş optimizasyon ele alınabilir. **None** durumu için **null pointer (0x0)** kullanıldığından tahsis edilen bellek miktarı aynıdır.

```rust
println!("&u32 türü için de {} byte yer ayrılır.", size_of::<&u32>());
println!(
    "ve Option<&u32> içinde {} byte söz konusudur.",
    size_of::<Option<&u32>>()
);
```

![Niche Optimization Two](../images/niche_opt_2.png)

## Memory/Object Pooling

Geliştirmekte olduğumuz uygulamaların heap üzerindeki yerleşimleri hem performans hem de verimlilik açısından önemlidir. Geniş arazilere sahip **heap** üstünde rastgele nesne yerleşimleri zamanla fragmantasyonu bozabilir, uygulamanın bellekte kapladığı alan şişebilir, tepki süreleri yavaşlayabilir. Esasında Rust, RAII _(Resource Acquisition is Initialization)_ ilkesine bağlı kalarak nesne ömürlerini optimal seviyede yönetir. _(Her ne kadar lifetimes mevzusu zaman zaman zorlayıcı olsa da)_ Ancak yine de tekrar tekrar üretilen pahalı nesneler söz konusu olduğunda pekala bunların heap organizasyonu endüstriyel diyebileceğimiz kimi standart metodolojilerle ele alınabilir. **Object Pooling** ' de bunlardan birisidir. Örneğin veri tabanı bağlantıları, oyun motorlarındaki asset yöneticileri sürekli ve tekrar tekrar kullanılan nesneler olarak karşımıza çıkarlar. Bunları belli limitlere sahip bir havuzdan yönetmek pekala ideal olabilir.

Aşağıdaki örnek kod parçasında çok basit bir uygulaması görülmektedir.

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

Yukarıdaki kodda ilkel bir **object pool** mekanizması uygulanmaya çalışılıyor. Teorik olarak **heap** maliyeti yüksek nesnelerin bir havuzdan karşılanması değerlendirilmekte. **AssetServer** yapısını yüksek maliyetli bir nesne gibi düşünebiliriz. **ObjectPool** isimli veri yapısı **Arc _(Atomic reference counted)_** isimli **smart pointer**'ı ve **Mutex** mekanizmasını kullanarak bu havuzu yönetiyor. Havuza nesneler eklenbiliyor ve release fonkisyonu çağırıldığında tekrardan havuza dönmelerini sağlanıyor. Örnek daha da geliştirilebilir. Söz gelimi **release** yerine **drop trait** implementasyonu kullanılabilir ve nesne scope dışında kaldığında tekrardan havuza iade edilmesi sağlanabilir. Havuzda hiç nesne yoksa ilk oluşturulma aşamasında en az bir tane eklenebilir. Bir üst yapı ile havuz yönetimi daha sağlıklı da ele alınabilir. Örneğin nesnelerin belli sayıda ve belli süre boyunca scope' da kalmaları sağlanabilir.

Çok doğal olarak kendi implementasyonumuz dışında kullanabileceğimiz hazır küfeler de _(crate)_ bulunuyor. En bilinenleri [lockfree-object-pool](https://crates.io/crates/lockfree-object-pool) ve [typed_arena](https://crates.io/crates/typed-arena). Aşağıdaki örnekte **typed_arena** kullanımına ait bir örnek yer almakta.

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

**Object pooling**, **arena allocator** başlığı altında incelediğimiz tek bir tampon yaklaşımını da kullanabilir. Örneğin **typed_arena** küfesi tekil **deallocation** işlevleri yerine kapsam dışına çıkıldığında tüm bloğu düşüren ve stack based yerine heap based çalışan bir kütüphanedir. Tipik bir **Arena Allocator** taktiği uyguluyor diyebiliriz. 

**Object Pooling** mevzusunda dikkat çeken noktalardan birisi de havuza nesneler eklendikçe heap' in şişmesidir. Bunu yönetmek de bir tasarım gerektirir. **typed_arena** gibi Arena Allocator stratejisini benimseyen kütüphaneler genellikle tek bir bellek bölgesi ayırmayı tercih eder, tek seferde bellekten düşürür, kapasite dolarsa var olandan bağımsız yeni bir bellek bölgesi daha tahsis eder. Dolayısıyla kendi yazdığımız senaryolarda bu kapasiteyi yönetmemiz de gerekebilir. İlk yazdığımız kodu bu anlamda değerlendirip aşağıdaki gibi yeniden düzenleyebiliriz.

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

**ObjectPool** yapısına **usize** türünden **capacity** alanı eklenmiştir. **add** ve **release** fonksiyonlarında kapasite kontrolü yapılır. Bu yöntem havuzdaki nesne sayısını sabitler ve **heap** bölgesinin gereksiz yere şişmesini engeller ancak dezavantajları da vardır. Örneğin havuzda boş yer yokken bir nesne istersek yeni nesne üretilemeyebilir. Bu da bizi **Eviction** stratejilerine götürür. **Least Recently Used _(LRU)_**, **Time-Aware Least Recently Used _(TLRU)_**, **Least Frequently Used _(LFU)_**, **Most Recently Used _(MRU)_** gibi daha birçok yaklaşım da mevcuttur. Bu teknikler sadece **object pooling** değil **cache mekanizmaları** için de ele alınan stratejilerdir. 

- **LRU**' da amaç kullanılmayan nesneleri belli bir sıraya göre havuzdan çıkarmaktır ve genellikle **web cache**, **session cahce**, oyunlarda asset yönetimi gibi senaryolarda ele alınır. Burada en az kullanılan nesneye erişmek **Big O** ölçümlemesine göre nispeten yavaş olabilir ve O(1) durumuna getirilmesi gerekebilir. 
- **TLRU**' da bir zaman damgası kullanılır ve buna göre belli süre kullanılmayan nesnelerin havuzdan çıkartılması sağlanır. Tahmin edileceği üzere amaç belli süre kullanılmayan nesneleri temizlemek bunu da bir zamanlayıcıya göre ayarlamaktır. Örneğin ömrü 60 saniyeden eski olanların temizlenmesi gibi. Geçici dosya yönetiminde, IoT sistemlerde ele alınabilir. 
- **LFU**' da amaç en az kullanılan ya da daha iyi bir tarifle en az başvurulan nesneyi havuzdan çıkarmaktır. Böylece sık kullanılan nesneler havuzda kalırken az kullanılanlar çıkarılır. Makine öğrenme modellerinde cache kullanılacağı zaman veya DB indeksleme işlemlerinde ele alınır. Sık kullanılan nesnelerin havuzda kalması önemli bir avantajdır ancak bu nesnelerin aranması, bulunması tam bir **O(N)** maliyetine eş olabilir.
- **MRU** ise en son kullanılan nesnenin bellekten çıkartılmasını hedefler. Veri sıkıştırma algoritmaları ve büyük dosya sistemlerinde kullanılan bir yöntem olduğu belirtilmektedir. LRU'nun tam tersi olarak da değerlendirilebilir. 

Tabii Object Pooling dedik, sonra havuz kapasitesini nasıl yöneteceğiz dedik ve kendimizi cache stratejilerinin uygulanmasında bulduk. Bu nedenle eğer sıfırdan bir object pool mekanizması tasarlamayacaksak bunu soyutlayan crate'lerden yararlanmak daha iyi olabilir.

## Cache Friendly Programming

Yüksek performanslı kod işletiminde programın çalıştığı sistemin donanımsal avantajlarından yararlanmak da gerekir. Bazı hallerde tampon bellek hassasiyeti olan programlama teknikleri kullanılabilir. Bu pratiklerde genellikle işlemcinin **L1, L2, L3** gibi farklı seviyelerdeki tampon bellek noktaları önemli rol oynar. [Alder Lake](https://en.wikipedia.org/wiki/Alder_Lake) kod adlı 12nci nesil **intel i7** işlemcilerini düşünelim. **L1** cache' de çekirdek başına **80 ila 96Kb**, **L2** cache'de **1.25** ila **2 Mb**, **L3** cache'de ise **30 Mb**'a kadar alan söz konusudur. Dizilim olarak kabaca şöyle düşünebiliriz.

```text
+------------------+
+   L1 - 96 Kb     +
+------------------+
         |
+------------------+
+   L2 - 2 Mb      +
+------------------+
         |
+------------------+
+   L3 - 30 Mb     +
+------------------+ 
         |
+------------------+
+                  +
+                  +
+                  +
+     Main Ram     +
+                  +
+                  +
+                  +
+                  +
+------------------+     
```   

**L** seviye cache'ler çekirdeğin en hızlı erişim yaptığı alanları içerir. L1'den ana belleğe gelirken **bandwidth** daralır ve gecikmeler _(Latency)_ artar. Ancak görüldüğü üzere kullanılabilecek kapasite ana bellekten L1'e gelirken epeyce azalır. Yani hızlanmak için her şeyi L seviye tampon bölgelerinde konumlandırmamız pek mümkün olmayabilir. Çoğu programlama dili bu yönetim için belli metodolojileri benimser. Bu alanda en sık verilen örnek iki boyutlu bir diziyle de ifade edilebilen matrislerdir. **Rust** bir çok dilde olduğu gibi bir matrisi ele alırken **satır öncelikli _(row-major order)_** bir yaklaşımı baz alır. Tahmin edileceği üzere birde **sütun öncelikli _(column-major order)** söz konusudur. Her iki yaklaşım dizi elemanlarının bellekte farklı biçimlerde yerleştirilmesi ile alakalıdır. Biraz daha detay için [Wikipedia](https://en.wikipedia.org/wiki/Row-_and_column-major_order) yazısına bakılabilir. 

Eğer iki boyutlu bir dizinin elemanlarını dolaşırken **row-major order** yaklaşımına uygun kod yazarsak elemanlar belleğe ardışıl dizileceğinden _(Kuvvetle muhtemel L1, L2 seviyesinde)_ erişiminde hızlı olacağını söyleyebiliriz. Konuyu biraz daha net anlamak için aşağıdaki örnek kod parçası ile devam edelim. Bu örnekte **criterion** küfesini de kullanarak benchmark sonuçlarını değerlendireceğiz. _(cache-friendly isimli örnek)_

```rust
pub const MAX_SIZE: usize = 1024;

pub fn row_major_call(matrix: &[[u8; MAX_SIZE]; MAX_SIZE]) -> usize {
    let mut sum = 0;
    for row in 0..MAX_SIZE {
        for col in 0..MAX_SIZE {
            sum += matrix[row][col] as usize;
        }
    }
    sum
}

pub fn column_major_call(matrix: &[[u8; MAX_SIZE]; MAX_SIZE]) -> usize {
    let mut sum = 0;
    for col in 0..MAX_SIZE {
        for row in 0..MAX_SIZE {
            sum += matrix[row][col] as usize;
        }
    }
    sum
}
```

Buradaki fonksiyonlar iki boyutlu bir matris üzerinden toplama işlemini icra etmektedir. İlk etapta kodlar aynı görünebilir, nitekim içiçe iki for döngüsünün işletilmesi söz konusudur. Ancak **row_major_calc** fonksiyonu diğerinin aksine en dış döngüde satırları sayarak ilerlemektedir. Bu iterasyon sırasında her bir satır için kolonlara geçilir. Rust'ın derleyicisi satır öncelikli bir dizilimi benimsediğinden bellek yerleşimi de buna göre ardışıl yapılır. Durumu daha iyi analiz etmek için criterion kütüphanesini kullanarak **benchmark** ölçümlemelerini değerlendirebiliriz. Tabii öncesinde projeye **criterion** küfesinin eklenmesi gerekir.

```bash
cargo add criterion
```

Buna bağlı olarak **toml** içeriğini de aşağıdaki gibi değiştirebiliriz.

```toml
[package]
name = "cache-friendly"
version = "0.1.0"
edition = "2021"

[dev-dependencies]
criterion = { version = "0.5.1", features = ["html_reports"] }

[[bench]]
name = "matrix-calc"
harness = false
```

Projenin root klasöründe benches isimli başka bir klasör açıp aşağıdaki içeriğe sahip **matrix-calc.rs** isimli dosyayı ekleyebiliriz.

```rust
use cache_friendly::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_access_patterns(c: &mut Criterion) {
    let matrix = [[1u8; MAX_SIZE]; MAX_SIZE];

    c.bench_function("Row Major Access", |b| {
        b.iter(|| row_major_call(black_box(&matrix)))
    });

    c.bench_function("Column Major Access", |b| {
        b.iter(|| column_major_call(black_box(&matrix)))
    });
}

criterion_group!(benches, benchmark_access_patterns);
criterion_main!(benches);
```

**Benchmark** testlerini çalıştırmak içinse aşağıdaki komutla ilerlemek yeterli.

```bash
cargo bench
```

// SONUÇLAR EKLENECEK

## Zero Cost Abstraction

Sıfır maliyetli soyutlamalar olarak çevirebileceğimiz bu kavram Rust'ın güçlü olan özelliklerinden birisidir. Bunu kısaca ifade etmek gerekirse, **iterator** metotları, **generic** tür kullanımları, **trait** implementasyonları gibi yaklaşımların çalışma zamanı maliyetlerinin sıfır olmasıdır/sıfıra yakın olmasıdır diye açabiliriz _(çok kesinlik katamadım ama iddialar bu yönde)_ Hatta idiomatic yazılmış bir rust kodunun sanki **C/C++** dilleri ile yazılmış kadar efektif olduğu belirtilir. Özellikle koleksiyonlar arkasından gelen iterator fonksiyonları için derleme zamanında üretilen kodun çalışma zamanında ek bir maliyet gerektirmeden yüksek performanslı çalıştığını belirten bir kavram olarak da tanıtılır. Generic türler açısından düşündüğümüzde Rust'ın **monomorfizasyon** tekniğiyle somut tiplere özel kodlar ürettiğini ifade edelim. Bu yaklaşım **C++** dilinde **template** enstrümanına benzer biçimde her tip kullanımı için ayrıca optimize edilmiş kod üretilmesi ile aynıdır. Bu durumda generic bir metot çağrımı ile normal metot çağrımı arasında pek bir performans farkı kalmaz. **Iterator** fonksiyonlar demişken aşağıdaki basit kod örneği ile bu durumu tarifleyebiliriz.

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

Birden ona kadar olan sayıların toplanması iki farklı şekilde yapılmaktadır. Birisinde iter() metodu üzerinden ulaşılıp map ve sum fonksiyonları ile hesaplama yapılmaktadır. Diğerinde ise bildiğimiz klasik bir **for** döngüsü kullanılmıştır. Kod okunurluğu açısından iterator kullanımı çok daha idealdir ve aslında birçok fonksiyonel dilde bu tip yüksek seviyeli işlevlere rastlanır ancak bu tip bir soyutlamanın çalışma zamanı maliyetleri dile göre tartışılır. **Zero-Cost Abstraction**, klasik for döngüsü ile yazılan ve optimize edilmiş kod çıktısının iterator fonksiyonlar için de söz konusu olduğunu söyler. 

Peki bu nasıl ispat edilebilir? Sanıyorum doğru optimizasyon seviyesinde üretilen kodun **assembly** çıktılarına bakarak bir kıyaslama yapmak mümkün. Bu henüz tam olarak gerçekeştiremediğim bir şey ancak yol boyunca öğrendiğim bazı şeyler de oldu. Örneğin **src** klasöründe **zca.rs** isimli yalnız başına takılan bir rust dosyası var. Bunu **rustc** ile deledikten sonra assembly koduna bakabiliriz.

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
