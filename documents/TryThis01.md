# Çoklu Thread Ortamlarında Ortak Veriyi Değiştirmek

[Try This 00](/documents/TryThis00.md) dokümanında, bir fonksiyona aktarılan parametre üzerinde değişklik yapmak istediğimizde bunun **C#**, **Rust** ve **Zig** programlama dilleri tarafındaki ele alınış biçimlerini farklı örneklerle incelemeye çalışmıştım. Nihayetinde ulaştığım noktada bir nesnenin kendisini tanımlayan değerlerin değiştirilmesinde programlama dili ve hatta kullanılacak yazılım mimarisi bazında farklı yaklaşımlar olduğunu görmüştük. Merak ettiğim bir başka konu ise, multi-thread *(çoklu iş parçacığı)* ortamlarında ortak veriyi değiştirmek. Aynı veri üzerinde birden fazla **thread**'in okuma/yazma işlemi yapması çok sık karşılaşılan bir durum. Şu anda elimde **managed** ortamı olan, yerleşik framework kütüphanelerinde üst düzey soyutlamalar sunan **C#**, bellek güvenliği konusundaki titiz stratejileri ve zorlamaları ile öne çıkan **Rust** ve **C**'nin modern bir versiyonu olarak gördüğüm düşük seviyeli sistem programlama dili olan **Zig** var.

## Önce C# Tarafı

Konuyu olabildiğince basit bir şekilde ele almak istiyorum. Söz gelimi aynı veri üzerine birden fazla thread'in yazmaya çalıştığı bir örnek düşünelim. Aşağıdaki kod parçası ile ilerleyebiliriz.

```csharp
public class Program
{
    static double CalculationResult = 0;

    static void Main(string[] args)
    {
        Thread threadA = new(PerformCalculationsA);
        Thread threadB = new(PerformCalculationsB);

        threadA.Start();
        threadB.Start();

        threadA.Join();
        threadB.Join();

        Console.WriteLine($"Final Calculation Result: {CalculationResult}");
    }

    static void PerformCalculationsA()
    {
        for (int i = 0; i < 100; i++)
        {
            CalculationResult += Math.Sqrt(i);
            Thread.Sleep(50);
        }
    }

    static void PerformCalculationsB()
    {
        for (int i = 0; i < 100; i++)
        {
            CalculationResult += Math.Log(i + 1);
            Thread.Sleep(50);
        }
    }
}
```

Bu programda **CalculationResult** isimli bir değişkene iki farklı **thread** tarafından yazma işlemi yapılıyor. Thread'ler tarafından işletilen metotlar kendi içlerinde açtıkları döngülerde hesaplama maliyeti yüksek sayılabilecek bir takım işlemler icra ediyor. Karekök alma ve logaritma hesaplamaları sırasında olayı daha da dramatize etmek içinse **Sleep** çağrıları ile gecikmeler oluşturuyoruz. Kendi sistemimde bu örneğin çalışma zamanı çıktıları aşağıdaki gibi oldu.

![TryThis02_00](../images/TryThis02_00.png)

Aslında her seferinde aynı değerin oluşmasını bekliyordunuz değil mi? Ortada herhangi bir hata mesajı yok ama sonuçlar her seferinde farklı çıkıyor. Burada bir tahmin yürütüp bir **thread** söz konusu değişkene yazma işlemi yaparken diğer **thread**'in onu beklemesi sağlayabiliriz. Yani içgüdüsel olarak thread'lerin aynı anda veriyi değiştirmesini engellemeyi tercih edebiliriz. Buna göre kodu aşağıdaki şekilde değiştirerek ilerleyelim.

```csharp
using System.Threading;
public class Program
{
    static double CalculationResult = 0;
    static readonly Lock calculationLock = new();

    static void Main(string[] args)
    {
        Thread threadA = new(PerformCalculationsA);
        Thread threadB = new(PerformCalculationsB);

        threadA.Start();
        threadB.Start();

        threadA.Join();
        threadB.Join();

        Console.WriteLine($"Final Calculation Result: {CalculationResult}");
    }

    static void PerformCalculationsA()
    {
        for (int i = 0; i < 100; i++)
        {
            double partialResult = Math.Sqrt(i);
            lock (calculationLock)
            {
                CalculationResult += partialResult;
            }
            Thread.Sleep(50);
        }
    }

    static void PerformCalculationsB()
    {
        for (int i = 0; i < 100; i++)
        {
            double partialResult = Math.Log(i + 1);
            lock (calculationLock)
            {
                CalculationResult += partialResult;
            }
            Thread.Sleep(50);
        }
    }
}
```

Bu sefer sonuçlar tam da beklediğimiz gibi. Her seferinde aynı toplam değerini yakalıyoruz.

![TryThis02_01](../images/TryThis02_01.png)

Yeni örneğimizde en basit kilit mekanizması olarak gördüğüm **Lock** nesnesini kullanıyorum. Her iki **thread**'in de ortak veriye erişmeden önce bu kilidi alması gerekiyor. Bir thread kilidi aldıktan sonra diğer thread'in kilit serbest bırakılana kadar da beklemesi gerekiyor. Bu sayede aynı anda sadece bir **thread** ortak veriye erişip üzerinde değişiklik yapabiliyor. Bu da veri tutarlılığını sağlıyor. Bu zaten **Mutual Exclusion *(Karşılıklı Dışlama)*** prensibinin temel çalışma mantığı.

Elbette farklı kilit mekanizmaları ve stratejileri var. Örneğin, [Reader-Writer Lock](https://learn.microsoft.com/en-us/dotnet/api/system.threading.readerwriterlock?view=net-10.0) mekanizması ile birden fazla thread'in aynı anda okuma yapmasına izin verilip, yazma işlemi yapılacağı zaman tüm okuma işlemlerinin bitmesi beklenebilir. Bu tür stratejiler performans açısından önemli olabilir.

Ancak burada temel prensip olarak ortak veriye erişim sırasında veri tutarlılığını sağlamak için bir tür senkronizasyon mekanizması kullanılması gerekliliği. **Interlocked** sınıfı gibi daha düşük seviyeli atomik işlemler sunan yapılar da var. Aynı örneği birde **Interlocked** sınıfı kullanarak ele alalım.

```csharp
using System.Threading;
public class Program
{
    static double CalculationResult = 0;

    static void Main(string[] args)
    {
        Thread threadA = new(PerformCalculationsA);
        Thread threadB = new(PerformCalculationsB);

        threadA.Start();
        threadB.Start();

        threadA.Join();
        threadB.Join();

        Console.WriteLine($"Final Calculation Result: {CalculationResult}");
    }

    static void PerformCalculationsA()
    {
        for (int i = 0; i < 100; i++)
        {
            double partialResult = Math.Sqrt(i);
            double initialValue, computedValue;
            do
            {
                initialValue = CalculationResult;
                computedValue = initialValue + partialResult;
            } while (Interlocked.CompareExchange(ref CalculationResult, computedValue, initialValue) != initialValue);
            Thread.Sleep(50);
        }
    }

    static void PerformCalculationsB()
    {
        for (int i = 0; i < 100; i++)
        {
            double partialResult = Math.Log(i + 1);
            double initialValue, computedValue;
            do
            {
                initialValue = CalculationResult;
                computedValue = initialValue + partialResult;
            } while (Interlocked.CompareExchange(ref CalculationResult, computedValue, initialValue) != initialValue);
            Thread.Sleep(50);
        }
    }
}
```

## Peki ya Rust tarafı?

**Rust**, sahiplik *(ownership)* ve borçlanma *(borrowing)* ilkeleri ile zaten bellek güvenliğini sıkı sıkıya denetlemeye çalışıyor. Benzer şekilde **multi-thread** ortamlarda veri yarışlarını önlemek için güçlü mekanizmalar da sunuyor. Ancak senaryoya ve ortak değişkene erişim şekline bağlı olarak *(sadece okuma amaçlı, çok okuyucu tek yazıcı vb.)* farklı stratejiler izlemek gerekebilir. Burada karşımıza çıkan en yaygın çözümlerden birisi **Arc *(Atomic Reference Counting)*** ve **Mutex *(Mutual Exclusion)*** kombinasyonu. İlk olarak önceki yazımızda da vurguladığımız gibi dotNetçi kafası ile düşünerek hareket edelim. Bu noktada kodu pekala aşağıdaki gibi yazsak olur.

```rust
use std::thread;

fn main() {
    let mut calculation_result: f64 = 0.0;

    let handle_1 = std::thread::spawn(|| {
        for i in 1..=100 {
            calculation_result += (i as f64).sqrt();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let handle_2 = std::thread::spawn(|| {
        for i in 1..=100 {
            calculation_result += (i as f64 + 1.0).ln();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("Calculation result {}", calculation_result);
}
```

Yazım stili farklılaşsa da benzer bir işleyiş olduğunu söyleyebilirim. **handle_1** ve **handle_2** isimliye yumurtlanan **thread**'ler ortak değişken olan **calculation_result** üzerinde yazma işlemi yapıyor. Ne varki bu kodu çalıştıramayacağız zira derleyici bizi aşağıdaki gibi üzecek.

![TryThis02_02](../images/TryThis02_02.png)

Bu son derece doğal, zira oluşturulan thread'ler kendi kapsamlarında bir üst thread olan main'deki bir değişkeni kullanma niyetindeler. Bunu açıkça ifade etmemiz bekleniyor ve bunun yolu da **move** operatörünün kullanılmasından geçiyor. Dolayısıyla söz konusu kodları aşağıdaki gibi değiştirerek ilerlememiz lazım.

```rust
use std::thread;

fn main() {
    let mut calculation_result: f64 = 0.0;

    let handle_1 = std::thread::spawn(move || {
        for i in 1..=100 {
            calculation_result += (i as f64).sqrt();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let handle_2 = std::thread::spawn(move || {
        for i in 1..=100 {
            calculation_result += (i as f64 + 1.0).ln();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("Calculation result {}", calculation_result);
}
```

Bu sefer program sorunsuz şekilde çalışacaktır ancak sonuçlar hiç de beklediğimiz gibi olmayacaktır. Programı kaç kez çalıştırırsak çalıştıralım her seferinde **0** *(yazıyla sıfır)* sonucunu elde ederiz :D Üstelik **Rust** derleyicisi **calculation_result** değişkeninin hiç kullanılmadığını bile iddia eder.

![TryThis02_03](../images/TryThis02_03.png)

Bunu bir açıklayalım. **move** operatörü ile **main thread** içerisindeki **calculation_result** değişkeni açılan thread scope'larına kopyalanarak taşınır. Yani her **thread** kendi **calculation_result** kopyasına sahip olarak çalışır. Dolayısıyla **+=** operatörünü **main thread** içerisindeki **calculation_result** değişkenine uyguluyor gibi görünsek de bu aslında o **scope** içerisine alınmış kopya üzerinde yapılan bir işlem. Gelin bu teoriyi ispat etmeye çalışalım. Bu amaçla kodu aşağıdaki gibi değiştirelim.

```rust
use std::thread;

fn main() {
    let mut calculation_result: f64 = 0.0;

    let handle_1 = std::thread::spawn(move || {
        for i in 1..=100 {
            calculation_result += (i as f64).sqrt();
            thread::sleep(std::time::Duration::from_millis(50));
            println!("Intermediate result from Handle 1 {}", calculation_result);
        }
    });

    let handle_2 = std::thread::spawn(move || {
        for i in 1..=100 {
            calculation_result += (i as f64 + 1.0).ln();
            thread::sleep(std::time::Duration::from_millis(50));
            println!("Intermediate result from Handle 2 {}", calculation_result);
        }
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("Calculation result {}", calculation_result);
}
```

![TryThis02_04](../images/TryThis02_04.png)

Tabii bu benim bakış açımdan bir ispat. Açılan her **handle scope**'u kopyalayarak aldığı **calculation_result** değişkeni üzerinde işlemler yapıyor ve kendi ara sonuçlarını yazdırıyor. Ama aynı şey **main thread** içindeki **calculation_result** değeri için geçerli değil. Burada **calculation_result** değişkeninin **f64** türünden olduğunu ve **Copy trait**'ini implemente ettiğini hatırlatayım. O halde bu işi **Rust** tarafında istediğimiz şekilde ele almak için de birşeyler yapmamız gerekiyor. Her şeyden önce ortak verinin iki thread içerisinde de kullanılabiliyor olması lazım. Bunun için genellikle **Atomic Reference Counting *(Arc)*** isimli akıllı işaretçi *(smart pointer)* kullanılmakta.

**Arc** enstrümanı bir verinin birden fazla sahibi olmasına olanak tanır ve bunun için referans sayımı yaparak belleğin doğru şekilde yönetilmesini sağlar. Ancak **Arc** tek başına yeterli değildir çünkü aynı anda birden fazla thread'in veriyi değiştirmesine izin verilmez. Bu nedenle **Mutex *(Mutual Exclusion)*** yapısını da kullanmamız gerekir. **Mutex**, aynı anda sadece bir thread'in veriye erişmesine izin verir ve böylece **data race** durumu önlenir. Aynen .net tarafında Lock ile yaptığımız gibi. Bu kombinasyonun uygulanışından önce söz konusu senaryoda Arc bileşenini Mutex olmadan kullanmayı deneyelim. Aynen aşağıdaki kod parçasında olduğu gibi.

```rust
use std::thread;

// CASE02: Bu senaryoda Arc'ı tek başına Mutex olmadan kullanırsak.
use std::sync::Arc;

fn main(){
    let calculation_result = Arc::new(0.0_f64);

    let calc_res_clone_1 = Arc::clone(&calculation_result);
    let handle_1 = thread::spawn(move || {
        for i in 1..=100 {
            *calc_res_clone_1 += (i as f64).sqrt();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let calc_res_clone_2 = Arc::clone(&calculation_result);
    let handle_2 = thread::spawn(move || {
        for i in 1..=100 {
            *calc_res_clone_2 += (i as f64 + 1.0).ln();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("Calculation result {}", *calculation_result);
}
```

Programı bu şekilde çalıştırdığımızda derleyicini kırıcı mesajları ile başbaşa kalırız.

![TryThis02_06](../images/TryThis02_06.png)

Bu mesajlar ilk etapta anlamsız gibi görünse de tam olarak konuyla ilgilidir. **cannot assign to data in an `Arc`** ifadesini takiben söylenen **trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<f64>`** mesajı Arc ile sarmalanmış bir veriyi değiştirmek için gerekli olan **DerefMut** trait'inin implemente edilmediğini söyler. Yani Arc tek başına ortak veriyi değiştirmek için yeterli değildir, zira o sadece **Deref trait**'ini implemente etmektedir. Bize gerekli olan şeyse DerefMut implementasyonu. Bunu da **Mutex** kullanarak sağlayabiliriz.

```rust
use std::thread;
use std::sync::Arc;

fn main(){
    let calculation_result = Arc::new(std::sync::Mutex::new(0.0_f64));

    let calc_res_clone_1 = Arc::clone(&calculation_result);
    let handle_1 = thread::spawn(move || {
        for i in 1..=100 {
            let mut result = calc_res_clone_1.lock().unwrap();
            *result += (i as f64).sqrt();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let calc_res_clone_2 = Arc::clone(&calculation_result);
    let handle_2 = thread::spawn(move || {
        for i in 1..=100 {
            let mut result = calc_res_clone_2.lock().unwrap();
            *result += (i as f64 + 1.0).ln();
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("Calculation result {}", *calculation_result.lock().unwrap());
}
```

Tabii kilitleri açık bir şekilde serbest bırakmak için bir kod kullanmadık ancak scope sonları zaten bu işin otomatik yapılmasını sağlayacaktır.

İşte çalışma zamanı çıktısı.

![TryThis02_05](../images/TryThis02_05.png)

Ay now, ay now... Tuhaf bir durum söz konusu. Yüksek hassasiyetli **64 bit floating point** değer her deneme için %100 tutarlı değil gibi görünüyor. Örneğin ikinci denemede son küsürat bir üst değere yuvarlanmış gibi. Alt alta yazınca fark daha net görülebilir.

```text
1039.8174431755524
1039.817443175553
```

Üstelik .Net tarafındaki gibi iki thread ve her birisinde 100er iterasyon içeren döngülerde çalışırken... Buradaki farkın karekök ve logaritma işlemlerinden kaynaklanan küçük yuvarlama hatalarından mı yoksa **Mutex** kilitlenmesi sırasında ortaya çıkan performans etkilerinden mi kaynaklandığını bilmiyorum. Belki de her ikisi birden. Ancak sonuçların her seferinde aynı olmaması dikkat çekici. Bir nevi thread'lerin farklı sıralarda çalışmasından kaynaklı bir durum da söz konusu olabilir. Sadece üstünkörü bir bakış açısıyla Rust tarafındaki hesaplamaların daha yüksek rakamlara ulaştığını ifade edebiliriz.

## Zig Tarafı

Gelelim yeni yeni öğrenemye çalıştığım Zig tarafına.

> DEVAM EDECEK AMA ÖNCE ZİG'DE THREAD KONUSUNU ÖĞRENMEM LAZIM :)
