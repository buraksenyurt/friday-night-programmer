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
    {
        let _id = Identity { value: 1001 };
        println!("Scope is closing...");
    }
    println!("End of the app");
}
```

Identity isimli struct çok basit bir veri yapısı. Ona **Drop** trait'ini implemente ediyoruz ve main fonksiyonu içerisinde açtığımız dahili bir scope içerisinde kullanıyoruz. İşlem sırasını tahmin edebilirsiniz. Aşağıdaki ekran görüntüsündeki gibi çalışacaktır.

![adventure_00.png](../../images/rust_adventure_00.png)
