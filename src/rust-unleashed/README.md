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

## Const ve Non-Mutable Static Değişkenler (adventure_01)

Değerinin değişmeyeceğini varsaydığımız türler için constant kullanmak yaygın bir alışkanlık. Zaten birçok dilde sabit değerler için böyle bir enstrüman bulunuyor. Rust tarafında constant tanımlamak için **const keyword** kullanılıyor. Constant değişkenler ilk değer ataması ile birlikte tanımlanıyor zira bellekte bu değişmez için ne kadar yer ayrılacağının baştan bilinmesi gerekiyor. Aşağıdaki kod parçasını göz önüne alalım.

```rust
const MAX_LEVEL: u32;

fn main() {

}
```

Bu kod derlenmeyecek ve aşağıdaki hata üretilecektir.

![adventure_02](../../images/rust_adventure_02.png)

```text
error: free constant item without body
 --> adventure_01\src\main.rs:8:1
  |
8 | const MAX_LEVEL: u32;
  | ^^^^^^^^^^^^^^^^^^^^-
  |                     |
  |                     help: provide a definition for the constant: `= <expr>;`
```

Şimdi bunu cebimize koyalım. Demek ki bir **constant** tanımlanırken ilk değerinin verilmesi zorunlu. Bir **constant**, **primitive** bir tür değerini taşımak zorunda da değildir. Pekala kendi tasarladığımız bir veri yapısını da constant olarak tanımlayabiliriz.

## Kaynaklar

- [Resmi Rust Kitabı](https://doc.rust-lang.org/reference/introduction.html)
