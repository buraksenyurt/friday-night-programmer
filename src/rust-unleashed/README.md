# Rust'ı Ne Kadar İyi Biliyoruz?

Bir dili iyi bildiğinize nasıl karar verirsiniz. Çeşitli algoritma sorularını çözerek, zorlu projeler yazarak veya... Rust zaten öğrenme eğrisi oldukça yüksek olan bir dil. En azından başlarda bazı yaklaşımlarını anlamak güç. İlk aklıma gelenler `**Ownership**, **Borrow Checkler**, **Lifetimes**, **Macros** gibi konular. Bu kavramları aştığımız takdirde ilerlemek oldukça kolay ama yine de gerçekten dilin yeteneklerini ne kadar iyi bildiğimize dair bir ölçü değil. Bu nedenle bir süredir çeşitli kaynaklardan *(Internet siteleri olmak üzere)* dilin bizi afallatacak soruları ile ilgili araştırmalar yapıyorum. Çok basit görünen ama bazen dakikalarca baktığım ve çözemediğim sayısız kod parçası ile karşılaştım. Tüm bunların derlenip toplanması ve bir başlık altında kaleme alınması gerekiyordu. İşte bu dokümanın amacı tam olarak bu.

## Scope Kavramı ve Ignore Binding Meselesi (adventure_00)

Rust'ta değişkenlerin yaşam süreleri önemli bir konu. Bir C# programcısı bu yaşam ömrünün .Net dünyasında **Garbage Collector** tarafından yönetildiğini bilir ve hatta değişkenler **scope** dışına çıktıklarında anında olmasa bile GC'nin radarına girerek gerektiğinde bellekten düşürülür. Rust dilinde de benzer şekilde scope kullanımı değişkenlerin yaşam ömürleri için belirleyici kriterlerdendir ancak yine biliriz ki Rust dilinde Garbage Collector gibi bir mekanizma yoktur fakat tüm kurallar belleği en yüksek seviyede güvenli kılabilmemizi garanti eder. Artık referanslar, unutulan referanslar, hayalet referanslar vs oluşmaz.

C# tarafında, **IDisposable** arayüzünü implemente eden bir referans nesne *(ki çoğu eder)* scope dışına çıktığında otomatik olarak **Dispose** metodu işletilir ve bazı kaynak temizleme işleri icra edilir. Rust dilinde buna benzer davranışı tanımlayan **Drop** isimli bir **trait** mevcuttur. Bu trait'i uyarlayan bir referans değişken scope dışında kaldığında uyguladığı **drop** fonksiyonu çağrılır. Zaten birçok dahili veri türü bu trait'i uygular. Ancak değişken atamalarında dikkate değer bir durum söz konusudur. Bazen bir referans türü değere bağlanmaz *(Ignore the Binding terimini nasıl ifade edebilirim bilemedim)* Konuyu biraz daha iyi anlayabilmek için örnek kodlara başvuralım.

İlk olarak temelleri biraz hatırlayalım.

```rust
fn main() {
    {
        let name = "Burak".to_string();
        println!("Hello, {}", name);
    }
    println!("How are you? {}", name);
}
```

Program kodunda yer alan **name** isimli değişken bir scope içinde tanımlanmıştır. Dolayısıyla kodun akan kısmında erişilebilir değildir. Bu yüzden derleme zamanında bir hata alırız.

```text
error[E0425]: cannot find value `name` in this scope
 --> adventure_00\src\main.rs:6:33
  |
6 |     println!("How are you? {}", name);
  |                                 ^^^^
  |
help: the binding `name` is available in a different scope in the same function
 --> adventure_00\src\main.rs:3:13
  |
3 |         let name = "Burak".to_string();
  |             ^^^^
```

Burada stack odaklı bir veri türü kullanmak da bir şeyi değiştirmez.

```rust
fn main() {
    {
        let number = 23;
        println!("The number is, {}", number);
    }
    println!("The number is {}", number);
}
```

Yine derleme zamanında bir hata alırız.

```text
error[E0425]: cannot find value `number` in this scope
 --> adventure_00\src\main.rs:6:34
  |
6 |     println!("The number is {}", number);
  |                                  ^^^^^^
  |
help: the binding `number` is available in a different scope in the same function
 --> adventure_00\src\main.rs:3:13
  |
3 |         let number = 23;
  |             ^^^^^^
```

Şimdi bu bilgileri cebimizde tutalım. En basit anlamda bir değişkenin tanımlandığı scope içerisinde kullanılabildiğini ifade edebiliriz. Şimdi Drop trait'inin davranışını incelemek üzere aşağıdaki kod parçasını ele alarak devam edelim.

```rust
#![allow(dead_code)]

struct Identity {
    value: u32,
}

impl Drop for Identity {
    fn drop(&mut self) {
        println!("Buckle your seat belt, dorothy, because kansas is going bye-bye");
    }
}
fn main() {
    case_one();
    println!("End of the app");
}

fn case_one() {
    let _id = &Identity { value: 1001 };
    println!("Scope is closing...");
}
```

**Identity** isimli struct çok basit bir veri yapısı. Ona **Drop** trait'ini implemente ediyoruz case_one isimli fonksiyon içerisinde kullanıyoruz. Bu fonksiyonu da main içerisinden çağırıyoruz. **_id**, **case_one** içerisinde tanımlı bir değişken olduğu için doğal olarak fonksiyon çağrısı sonlandığı yerde bellekten düşürülecek. İşlem sırasını tahmin edebilirsiniz. Aşağıdaki ekran görüntüsündeki gibi çalışacaktır.

![adventure_00.png](../../images/rust_adventure_00.png)

İşte şimdi beyin yakan kod parçasının tam zamanı. İkinci bir fonksiyon daha ekleyerek ortamı şenlendirelim.

```rust
#![allow(dead_code)]

struct Identity {
    value: u32,
}

impl Drop for Identity {
    fn drop(&mut self) {
        println!("Buckle your seat belt, dorothy, because kansas is going bye-bye");
    }
}
fn main() {
    // case_one();
    case_two();
    println!("End of the app");
}

fn case_one() {
    let _id = &Identity { value: 1001 };
    println!("Scope is closing...");
}

fn case_two() {
    _ = &Identity { value: 1001 };
    println!("Scope is closing...");
}
```

Bu kodun çalışma zamanı çıktısı ise aşağıdaki gibi olacaktır.

![adventure_01.png](../../images/rust_adventure_01.png)

Aradaki farkı görebildiniz mi? İnanın bu konuya istinaden yazılmış örnek kodlara dakikalarca baktım ve farkı göremedim. Dikkat edileceği üzere **drop** fonksiyonu **case_two** isimli blok sonlanmadan önce ve hatta **_** atamasının yapıldığı satırın hemen ardından çağırılmış görünüyor. Dolayısıyla elimizde tanımlandığı kapsam *(case_two fonksiyonunun kapsamı)* sonuna kadar yaşamamış bir değişken var. İşte burada çok temel bazı bilgileri gözden kaçırmış olduğumu fark ettim.

Aslında her iki metot özelinde düşündüğümüzde **Identity** türünden bir değer oluşturduğumuzda *(eşitliklerin sağ tarafları)* veri bellekte geçici bir alana *(temporary memory location)* alınıyor. Bunda olağanüstü bir durum var. Ta ki biz onu gerçekten kullanacağımızı belirttiğimiz bir değişkene atayıncaya kadar ki bu da **let** anahtar kelimesi ile yapılan bir atama işlemi demek. let ile yapılan atamada bu geçici bellek bölgesi aslında bir değişken ile bağlanırken *(binding)* kendi yaşam süresi de uzatılıyor. Ancak **_** atamasında değişkene bağlama işlemi kasıtlı olarak atlanmakta. Buna ignore the binding deniyor. Yani geçici bellek bölgesi bir değişkene bağlanmadığı için yaşam süresi de uzatılmıyor ve hemen ardından **drop** fonksiyonu çağrılarak bellekten düşürülüyor. Ben bir yaşıma girdim. Belki hiç önemsenmeyecek bir detay gibi görünebilir ama dile olan hakimiyetimiz açısından önemli başlıklardan birisi.

Maceralarım devam edecek :D

## Her Const Kullanımı Yeni Bir Geçici Kopya Demek mi? (adventure_01)

Değerinin değişmeyeceğini varsaydığımız türler için constant kullanmak yaygın bir alışkanlık. Zaten birçok dilde sabit değerler için böyle bir enstrüman bulunuyor. Rust tarafında constant tanımlamak için **const keyword** kullanılıyor. Constant değişkenler ilk değer ataması ile birlikte tanımlanıyor zira bellekte bu değişmez için ne kadar yer ayrılacağının baştan bilinmesi gerekiyor. Aşağıdaki kod parçasını göz önüne alalım.

```rust
const MAX_LEVEL: u32;

fn main() {

}
```

Bu kod derlenmeyecek ve aşağıdaki hata üretilecektir.

![rust_adventure_02.png](../../images/rust_adventure_02.png)

```text
error: free constant item without body
 --> adventure_01\src\main.rs:8:1
  |
8 | const MAX_LEVEL: u32;
  | ^^^^^^^^^^^^^^^^^^^^-
  |                     |
  |                     help: provide a definition for the constant: `= <expr>;`
```

Şimdi bunu cebimize koyalım. Demek ki bir **constant** tanımlanırken ilk değerinin verilmesi zorunlu. Bir **constant**, **primitive** bir tür değerini taşımak zorunda değil. Pekala kendi tasarladığımız bir veri yapısını da constant olarak tanımlayabiliriz. Hem kendi veri yapımızı kullanmak hem de farklı bir durumu değerlendirmek için aşağıdaki kod parçasını ele alalım.

```rust
#![allow(dead_code)]

struct BackgroundColor {
    name: &'static str,
    id: u32,
}

impl Drop for BackgroundColor {
    fn drop(&mut self) {
        println!("Dropping constant. State: {},{}", self.name, self.id);
    }
}

const BACKGROUND_COLOR: BackgroundColor = BackgroundColor {
    name: "Lightsaber",
    id: 1,
};

fn main() {
    let value = &mut BACKGROUND_COLOR;
    value.name = "Black pearl";
    value.id = 2;
    println!("Value Name: {} and ID: {}", value.name, value.id);

    BACKGROUND_COLOR.name = "Red wine";
    BACKGROUND_COLOR.id = 2;
    println!(
        "Background Color Name: {} and Id {}",
        BACKGROUND_COLOR.name, BACKGROUND_COLOR.id
    );
}
```

**BackgroundColor**, **statik** yaşam ömürlü literal string *(&str)* ve stack odaklı 32 bit işaretsiz tamsayı *(u32)* taşıyan bir veri yapısı. Özellikle yaşam döngüsünü izlemek istediğimiz için **Drop** trait'ini de implement ettik. Sonrasında **BACKGROUND_COLOR** isimli bir **constant** tanımlıyoruz ve buna ilk değerini verirken **name** ile **id** alanlarına da birer değer atıyoruz. main fonksiyonu içerisinde ise dikkate değer işler söz konusu. İlk olarak **constant** 'ın **mutable** bir referansını **value** isimli bir değişkene bağlıyoruz. Ardından **value** üzerinden **name** ve **id** alanlarında değişiklik yapıyoruz. Bundan herhangi bir sorun yok zira constant yeni bir temporary alan halinde yeniden oluşturulup bind ediliyor. Sonrasında **BACKGROUND_COLOR** constant'ının **name** alanını değiştiriyoruz ve tekrardan ekrana yazdırıyoruz. Öncelikle kodun çalışma zamanı çıktısını bir inceleyelim.

![rust_adventure_03.png](../../images/rust_adventure_03.png)

İlk dikkat etmemiz ve okumamız gereken yer uyarı mesajı.

```text
warning: taking a mutable reference to a `const` item
  --> adventure_01\src\main.rs:22:17
   |
22 |     let value = &mut BACKGROUND_COLOR;
   |                 ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
  --> adventure_01\src\main.rs:16:1
   |
16 | const BACKGROUND_COLOR: BackgroundColor = BackgroundColor {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(const_item_mutation)]` on by default

warning: `adventure_01` (bin "adventure_01") generated 1 warning
```

Uyarı mesajı **const** bir öğeye mutable referans alındığını belirtmekte. Nitekim her **const** kullanımı yeni bir geçici alan oluşturulması demek. Örnek kodda bu yeni alan referans olsa dahi **value** isimli yepyeni bir değişkene bağlanıyor *(binding)*. Bir başka ifadeyle **const** tanımlanan bir öğe bellekte tek bir yerde durmuyor ve her kullanıldığında yeni bir kopyası oluşturuluyor. Dolayısıyla biz değiştirilebilir referans ile bu yeni kopyaya erişiyoruz. Farklı bir kopya üzerinden çalışmamızda bir sakınca yok zira aynı verinin sahipliğini taşımıyorlar. Dolayısıyla **value** üzerinden **name** ve **id** gibi alanları değiştirmemizde bir problem yok ve bu değişiklikler orjinal constant değerini de etkilemiyor. Diğer yandan kodun devam eden kısmında doğrudan **BACKGROUND_COLOR** constant değişkeni üzerinde işlemler yapıyoruz. Sırasıyla name ve id alanlarının içeriğini değiştiriyoruz. Lakin her bir atama işlemi yeni bir constant kopyasının oluşturulması ve satır sonlandığı anda *(yani ; ile ifade tamamlandığında)* da derhal **drop** edilmesi demek. Bu nedenle constant değişkeni üzerinden name ve id alanlarına müdahale etsek dahi asıl constant içeriği sabit kalmaya devam ediyor. Birden fazla constant kopyası oluşmasının ispatı da program sonunda çalıştırılan drop çağrıları ile anlaşılabiliyor.

## Zero Sized Types (adventure_02)

Rust'ın yetenekli bir dil olduğunu biliyoruz. Bu nedenle birçok iyi dilden esinlenip adapte ettiği türlü özellikleri var. Söz gelimi fonksiyonel dil paradigmasında Option ve Result türlerini alması ya da Haskell'den Type Class kavramını alıp Trait enstrümanını kullanması gibi. Bu ve başka özellikler dilini gücünü de artırıyor. Çok fazla söz edilmeyen bir başka güçlü kavram ise Zero Sized Types. Hatta şu anki kod macerasını kavrayabilmek için öncelikle sıfır boyutlu bir veri türü söz konusu olabilir mi, olursa hangi senaryolarda işe yarar bakmam gerekti.

Zero Sized Type *(ZST)* türünden veri yapıları bellekte yer kaplamayan türlerdir. Bir başka deyişle 0 byte uzunluğundadırlar. Örneğin herhangi bir alan içermeyen bir struct, unit türü *(() ile ifade edilir)* ve bazı **enum türleri *(örneğin Empty gibi)*** ZST olarak kabul edilir. İhtiyaca göre kendi ZST veri yapılarımızı da tanımlayabiliriz. Bu konuda daha çok **State Machine** senaryoları gösteriliyor. Böyle ifade edince de aklıma gelen ilk senaryo bir oyundaki ana döngünün yönetilmesi. Hatta bu tip bir senaryoda **PhantomData** kullanımı da söz konusu olabilir. Bu konuya açıklık getirmek için aşağıdaki kod parçasını ele alalım.

```rust
```

## Kaynaklar

- [Resmi Rust Kitabı](https://doc.rust-lang.org/reference/introduction.html)
