# Pi Sayısını Hesaplama Yolunda

Matematiksel yöntemlerden bazıları ele alarak belli bir basamağa kadar pi sayısını hesaplamaya çalışacağım. Doğru bir basamak değerine ulaşmak ve burada yüksek sürate çıkmak hedeflerim arasında. Önce en amele yöntemlerden başlayarak daha sonra daha karmaşık yöntemlere geçmek niyetindeyim.

Kapsam

- Dart oynamayı severiz. Monte Carlo ile başlayalım.
- Chudnovsky algoritması ve ardından Gauss-Legendre algoritmasını deneyelim.
- Paralel hesaplama yöntemlerini kullanarak performansı artırmaya çalışalım.
- Race Condition'ları ve diğer senkronizasyon sorunlarını ele alarak kodumuzu optimize edelim.

## Önce Kısa bir Matematika ve Monte Carlo Seyahati

**Pi *(π)***, bir dairenin çevresinin çapına oranı olarak ifade edilebilir. Yaklaşık olarak ve genelde ezberlediğim değeri *3.14159* olarak bilinir ancak aslında sonsuz bir ondalık dizisine sahiptir ve tam değeri bilinmemektedir. *Pi (π)* sayısını kutlamak için özel bir gün bile vardır: 14 Mart, yani 3/14...

Bir çılgınlık yaparak kodlarımızı deterministik olmayan bir matematikle yazabiliriz. **Monte Carlo** yöntemine göre pi sayısının hesaplanması için bir çembere ve rastgele iki double değere ihtiyacımız vardır. Rastgele değerlerimiz çemberin içine düşerse, **pi *(π)*** sayısının yaklaşık değeri için bir tahmin yapabiliriz. Tabii burada kullanmamız gereken bir formül ve iterasyon sayısı var. Öyleyse vakit kaybeden kodlamaya başlayalım.

## Temiz, Sequential Versiyon

İşte en amatöründen bir **C#** kod örneği.

```csharp
using System.Diagnostics;

public class Program
{
    public static void Main()
    {
        long totalIterations = 100_000_000;
        Stopwatch stopwatch = Stopwatch.StartNew();
        for (int i = 0; i < 10; i++)
        {
            stopwatch.Restart();
            int inCircle = (int)PiEstimatorV1(totalIterations);
            Console.WriteLine($"Estimated value of π: {4.0 * inCircle / totalIterations} in {stopwatch.ElapsedMilliseconds} ms");
        }
    }

    public static long PiEstimatorV1(long iterations)
    {
        long inCircle = 0;
        var random = new Random();

        for (int i = 0; i < iterations; i++)
        {
            double x = random.NextDouble();
            double y = random.NextDouble();
            if (x * x + y * y <= 1.0)
            {
                inCircle++;
            }
        }
        return inCircle;
    }
}
```

Kendi sistemimde normal **dotnet run** koşusuyla elde ettiğim sonuçlar şöyle.

![CalculatePi_00](../images/CalculatePi_00.png)

Yaklaşık 1000 milisaniye civarında bir sürede 3.14 etrafında dolandığımızı söyleyebilirim. Burada gayet senkron bir şekilde tek bir thread kullanarak hesaplama yapıyoruz. Devam etmeden önce rastgele değerlerin çember içerisinde kalıp kalmama formüllerini daha şık yazabilirmiyim diye düşündüm ve **Math** sınıfının **Pow** metodunu kullanarak kodu biraz daha okunabilir hale getirdim.

```csharp
if (Math.Pow(x, 2) + Math.Pow(y, 2) <= 1.0)
{
    inCircle++;
}
```

![CalculatePi_01](../images/CalculatePi_01.png)

Biraz göreceli de olsa **Math** sınıfının statik **Pow** metodunu kullanmak kodun okunabilirliğini artırıp amacını daha şık ifade etse de süreler neredeyse üç kata kadar arttı. Yol yakınken geri dönme vakti.

### Iterasyon Sayısı Artıyor, Paralel Çalışma Geliyor

İlk metodolojimize göre 100 milyon iterasyon bu sistemde kabul edilebilir bir ortalama çalışma süresi yakaladı ancak daha yüksek ve tutarlı bir hesaplama için iterasyon sayısını artırmak gerekir. Bunun üzerine 1 milyar iterasyonu denemeye karar verdim. İşte sonuçlar.

![CalculatePi_02](../images/CalculatePi_02.png)

"Gerçekten bu kadar süre bekledin mi?" diye sorabilirsiniz. Evet, bekledim :D İterasyon sayısının büyümesi hesaplama süresini çok daha dramatik olarak artırdı. Dolayısıyla bir şeyleri paralel hale getirmenin ve pek tabii bunu da güvenli *(thread-safe)* ve *race condition* sorunlarından kaçınarak yapmanın zamanı geldi. Aslında **for** döngüsünü paralel hale getirebiliriz ve tahminen oluşabilecek **race condition** sorununu da **Interlocked** sınıfını kullanarak çözebiliriz. İşte paralel hale getirilmiş ve güvenli bir şekilde sayaç artırdığını düşündüğüm kod parçası.

```csharp
public static long PiEstimatorV2(long iterations)
{
    long inCircle = 0;
    var random = new Random();

    Parallel.For(0, iterations, i =>
    {
        double x = random.NextDouble();
        double y = random.NextDouble();

        if (x * x + y * y <= 1.0)
        {
            Interlocked.Increment(ref inCircle);
        }
    }
    );
    return inCircle;
}
```

Japon bir kılıç ustasının sakince kata yaparken ki ruh haline bürünüp sabırla beklesem de ilk iki çalışma süresini görünce programın çalışmasını sonlandırdım.

![CalculatePi_03](../images/CalculatePi_03.png)

Bir şeylerin ters gittiği hatalı kodlama yaptığım gün gibi ortada. Hatta işin çok daha enterasan yani paralel for döngüsünü bir kenara bırakıp sadece **Interlocked** nesnesini kullanınca ortaya çıktı.

```csharp
public static long PiEstimatorV3(long iterations)
{
    long inCircle = 0;
    var random = new Random();

    for (int i = 0; i < iterations; i++)
    {
        double x = random.NextDouble();
        double y = random.NextDouble();

        if (x * x + y * y <= 1.0)
        {
            Interlocked.Increment(ref inCircle);
        }
    }
    return inCircle;
}
```

Şaşılacak şey ama sadece **Interlocked** sınıfını kullanarak sayaç artırmaya çalışmak daha iyi sonuçlar verdi. Fakat bu sefer de paralel çalışmanın avantajını tam olarak kullanamdık.

![CalculatePi_04](../images/CalculatePi_04.png)

### Thread-Local Random ve Daha İyi Paralel Çalışma

Sanki doğru yolda ilerliyor gibiyim ama tam olarak değil. **Interlocked** sınıfı thread'ler arası güvenli bir şekilde sayacı artırmamızı sağlasa da, paralel çalışacak her bir iterasyonda bu işlemi yapmak ciddi bir performans kaybına neden oluyor gibi duruyor. Çünkü her bir thread'in sayaç değerini güncellemesi gerektiğinde, diğer thread'lerin de bu değere erişmeye çalışması bir tür bekleme durumunu tetikliyor. Dolayısıyla her bir thread'in kendi sayaç değerini tutması ve en sonunda bu değerleri birleştirmek daha doğru olacak. O zaman birde aşağıdaki kod parçasını deneyelim.

```csharp
public static long PiEstimatorV4(long iterations)
{
    long inCircle = 0;
    using var tlRandom = new ThreadLocal<Random>(() => new Random());

    Parallel.For(
        0L,
        iterations,
        () => 0L,
        (_, _, localCount) =>
        {
            var rng = tlRandom.Value!;
            double x = rng.NextDouble();
            double y = rng.NextDouble();
            return x * x + y * y <= 1.0 ? localCount + 1 : localCount;
        },
        localCount => Interlocked.Add(ref inCircle, localCount)
    );

    return inCircle;
}
```

**tlRandom** isimli **ThreadLocal** sınıfını kullanarak her bir thread'in kendi rastgele sayı üreteci örneğine sahip olmasını sağlıyoruz. Bu sayede her bir thread'in kendi sayaç değerini tutması ve sonunda bu değerleri güvenli bir şekilde birleştirmesi mümkün hale geliyor. **Paralel for** döngüsü bu örnekte tam beş parametre almakta. Dile kolay tam beş, iki tane daha alsa **Sonarqube**'e takılır herhalde :D İlk iki parametre iterasyon aralığını tanımlarken *(0'dan maksimum iterasyon değerine kadar)*, üçüncü parametre her bir thread için hesaplama değerinin başlangıç değerini belirliyor. Dördüncü parametre her bir iterasyonda çalışacak olan fonksiyonu temsil ediyor ki burada Monte Carlo simülasyonu yapılmakta ve bu fonksiyon, her bir thread'in kendi sayaç değerini güncelliyor. Son parametre ise tüm thread'lerin sayaç değerlerini güvenli bir şekilde birleştirmek için kullanılan fonksiyonu işaret ediyor.

Peki ya çalışma zamanı çıktısı...

![CalculatePi_05](../images/CalculatePi_05.png)

### Gerçek Çekirdek Sayısına Göre Bölme

Not bad, not bad! Hesaplama süreleri daha makul bir noktaya geldi. Ancak daha şık bir tasarıma veya modele gidebilir miyim diye de düşünüyorum. Paralel for döngüsünde kalacağım ama makinedeki çekirdek sayısını da hesaba katarak ilerlemek mantıklı olabilir. Yani her çekirdeğin kendi sayaç değerini tutması ve sonunda bu değerlerin birleştirilmesi gibi bir yaklaşım daha verimli olabilir. Bu amaçla aşağıdaki kod parçasını ele alabiliriz.

```csharp
public static long PiEstimatorV5(long iterations)
{
    int coreCount = Environment.ProcessorCount;
    long chunkSize = iterations / coreCount;
    long inCircle = 0;

    var tasks = Enumerable.Range(0, coreCount).Select(id => Task.Run(() =>
    {
        var rng = new Random();
        long localCount = 0;
        long start = id * chunkSize;
        long end = id == coreCount - 1 ? iterations : start + chunkSize;

        for (long i = start; i < end; i++)
        {
            double x = rng.NextDouble();
            double y = rng.NextDouble();
            if (x * x + y * y <= 1.0)
                localCount++;
        }

        Interlocked.Add(ref inCircle, localCount);
    })).ToArray();

    Task.WaitAll(tasks);
    return inCircle;
}
```

Bu sefer işlemcideki çekirdek sayısına göre iterasyonları bölüyor ve her çekirdek için ayrı bir görev oluşturuyoruz. Her görev kendi rastgele sayı üretecine sahip ve kendi sayaç değerini tutuyor. Görevler tamamlandığında, sayaç değerleri güvenli bir şekilde toplanıyor. Bu, bir önceki versiyona göre performansı biraz daha artırdı diyebilirim. İşte sonuçlar.

![CalculatePi_06](../images/CalculatePi_06.png)

Tabii burada "attığımız taş ürküttüğümüz kurbaya değdi mi?" atasözünü hatırlamakta fayda var. En başta senkron olarak çalışan versiyonu bu son iterasyon sayısı ile tekrar denediğimde aşağıdaki sonuçlara ulaştım. Evet paralel çalışmada sonuçlar daha iyi ama yüksek iterasyon sayıları için geçerli bir durum. Düşük aralıklarda bu maliyete girmeyebiliriz de ve hatta daha yavaş çalışmalar da ortaya çıkabilir.

![CalculatePi_07](../images/CalculatePi_07.png)

## Monte Carlo C# Turunun Değerlendirmesi

Buraya kadarki örnek kod versiyonlarını daha iyi değerlendirmek için aşağıdaki tabloyu ele alabiliriz.

| Kriter | V0 | V1 | V2 | V3 | V4 | V5 |
| ------- | ---- | ---- | ---- | ---- | ---- | ---- |
| **Paralellik** | Yok | Yok | Var | Yok | Var | Var |
| **Thread-Safe Random** | Var | Var | Yok ve race-condition riski var | Var | ThreadLocal ile Var | Var(kendi instance'ı üzerinden) |
| **Math.Pow Lüksü** | Yok | Çok yavaş | Çok yavaş | Yok | Yok | Yok |
| **Iterasyon başına Interlocked** | Yok | Yok | Var | Var | Yok | Yok |
| **Çekirdek sayısına göre bölme** | Yok | Yok | Yok | Yok | Yok | Var |
| **Lock-free döngü** | Var | Var | Yok | Yok | Var | Var |

ve şu ana kadar ki Monte Carlo uyarlamaları için şunları da söyleyebiliriz:

- **V0:** Basit, hızlı ama tek thread ile çalışıyor. Iterasyon sayısı arttıkça süreler dramatik şekilde artıyor.
- **V1:** Math.Pow kullanımı performansı ciddi şekilde düşürüyor. Okunabilirlik artarken süreler kabul edilemez seviyelere çıkıyor.
- **V2:** Paralel for döngüsü içeriyor ama iki kritik sorunu var. Random paylaşımlı olduğu için *race condition* riski var ve her iterasyonda Interlocked kullanımı ciddi performans kaybına neden oluyor.
- **V3:** Interlocked.Increment ilginç bir şekilde sıralı bir döngüye ekleniyor gibi. Hesaplama V0 gibi hızlı olsa da paralel çalışmanın avantajını tam olarak kullanamıyor.
- **V4:** ThreadLocal kullanarak her thread'in kendi Random örneğine sahip olması ve sayaç değerlerini güvenli bir şekilde birleştirmesi performansı önemli ölçüde artırıyor. Ancak paralel for döngüsünün getirdiği bir yük hala var gibi görünüyor.
- **V5:** Çekirdek sayısına göre iterasyonları bölerek her çekirdeğin kendi görevini yapması ve sonunda güvenli bir şekilde birleştirmesi performansı daha da artırıyor.

## Peki Ya Aynı Paralel Çalışmayı Rust Dili ile Yapsaydık?

Pek tabii bu tip yüksek performans gerektiren hesaplamalar söz konusu olunca aklıma ilk gelen **Garbage Collector** mekanizmasını aradan çıkarmak hatta **managed environment** in de olmadığı bir dilde ilerlemek oluyor. Birkaç yıl olsa da ilgilenme fırsatı bulduğum **Rust** ve çok kısa bir süre baktığım **Zig** ile aynı senaryoyu denemek istiyorum. Önce **rust** tarafı ile başlayalım.

İşlemci çekirdeklerinden maksimum fayda sağlamak adına **rayon** küfesi biçilmiş kaftan. Tabii rastgele sayı üretimi için de **rand** kütüphanesini kullanmayı tercih edeceğim. Bunları projeye **cargo** ile ekleyebiliriz.

```bash
cargo add rayon rand
```

main.rs dosyasına da aşağıdaki kod parçasını ekleyerek devam edebiliriz.

```rust
use rand::rngs::SmallRng;
use rand::RngExt;
use rayon::prelude::*;

fn main() {
    let total_iterations = 1_000_000_000;

    for _ in 0..10 {
        let start_time = std::time::Instant::now();
        let pi_estimate = calculate_pi(total_iterations);
        let elapsed = start_time.elapsed();
        println!("Estimated Pi: {} in {:?}", pi_estimate, elapsed);
    }
}

fn calculate_pi(total_iterations: u64) -> f64 {
    let in_circle: u64 = (0..total_iterations)
        .into_par_iter()
        .map_init(
            || rand::make_rng::<SmallRng>(),
            |rng, _| {
                let (x, y): (f64, f64) = rng.random();

                if x * x + y * y <= 1.0 {
                    1_u64
                } else {
                    0_u64
                }
            },
        )
        .sum();

    4.0 * (in_circle as f64) / (total_iterations as f64)
}
```

**calculate_pi** metodumuzu yine 10 kez çalıştırıp süre ölçümlemesi yapmaktayız. **into_par_iter()** ile paralel bir iterator ve devamında kullanılan **map_init** ile her bir **thread** için ayrı bir rastgele sayı üreteci oluşturuyoruz. map_init kod bloğunda f64 türünden rastgele x ve y değerleri üretiliyor ve çember içinde olup olmadıkları kontrol ediliyor. En sonunda da **sum** yardımıyla **in_circle** sayısı toplanarak **pi** tahmini yapılıyor ve bulunan sonuç döndürülüyor. Öncelikle bu kodu derleme modunda çalıştırarak süreleri gözlemleyelim.

![CalculatePi_08](../images/CalculatePi_08.png)

C# programımıza göre çok daha kötü bir performans sergiliyor gibi duruyor. Aslında bu sonucu normal karşılamak gerekir zira **cargo run** komutu varsayılan olarak **debug** modunda çalışır. Yani **release** modda çalıştırıp bir kıyaslama yapmak daha doğru olur.

```bash
cargo run --release
```

![CalculatePi_09](../images/CalculatePi_09.png)

"Hımmm... Ama hile yapıyorsun hocam oldu mu şimdi?"" :D .Net tarafında da **release** modda çalıştırarak bir kıyaslama yapmamız gerekir esasında. Birde öyle deneyelim.

```bash
dotnet run -c Release
```

![CalculatePi_10](../images/CalculatePi_10.png)

dotnet run ile doğrudan çalıştırdığımıza göre daha iyi sonuçlar aldık ama Rust tarafına nazaran çok kötü. Şartlar yine eşit değil ama. Burada Just-in Time derleyicisinin optimizasyonları devreye girdi ve bu nedenle performans artışı sağlandı. Aslında .Net 8 sonrası gelen **Native AOT** desteği ile Rust tarafına daha yakın sonuçlar elde etmek mümkün olabilir. O zaman birde **Native AOT** ile deneyelim. Bu amaçla ben aşağıdaki komutu denedim. Hem işlemci mimarisi hem de işletim sistemine uygun bir **release** çıktısı hazırlıyoruz. Sonrasında tabii bu exe'yi çalıştırmamız gerekiyor.

```bash
dotnet publish -c Release -r win-x64 --self-contained true /p:PublishAot=true
cd .\bin\release\net10.0\win-x64\publish\
.\CalculatePi.exe
```

![CalculatePi_11](../images/CalculatePi_11.png)

Yani o kadar büyük bir iyileşme olmadı gibi. Hele rust ile release modda çalıştırılan kodun çalışma zamanı süreleri düşünülünce. Atladığım kesin bir şey var ve emin olmak adına C# uygulamasını belki de  **SIMD *(Single Instruction, Multiple Data)*** desteği ekleyerek denemek gerekir. Tabii burada SIMD konusunu kısaca izah etmek lazım. Uzmanı değilim ama okuduğum kadarıyla hikayeyi şöyle ele almak gerekiyor;

Monte Carlo yöntemimizde çembere isabet etme durumunu tespit edereken bir formül kullanıyoruz. `x*x + y*y <= 1.0` şeklinde. İşlemcinin her bir sayı çifti için tek tek bu işlemi yaptığını düşünelim. Ancak SIMD desteği ile işlemcinin bazı register'larını kullanarak aynı anda 4 double veya 8 float işlemin tek bir saat vuruşunda *(clock cycle)* yapılması sağlanabilir. Yani tek bir işlemle 4 veya 8 sayı çifti için `x*x + y*y <= 1.0` kontrolü yapılabilir. İşin matematiği beni şu an için aşıyor ancak .Net'in System.Runtime.Intrinsics kütüphanesinde yer alan `Vector256`, `Vector<T>` gibi türler ile bu tür bir optimizasyonu deneyebiliriz. Tür adlarından da anlaşılacağı üzere burada hesaplamaların vektör karşılıklarının bulunduğu bir senaryo var.

Ancak işimizi zorlaştıracak bir kısım var ki o da rastgele sayı üretimi. **NextDouble** metodunun tekil *(kaynaklarda scalar olarak ifade ediliyor)* çalıştığı ve SIMD ile doğrudan vektör halinde çalışacak bir karşılığının olmadığı görülüyor. Bu, rastgele sayıları bu şekilde kullanırsak donanımsal avantajlardan yararlanamayacağımız anlamına geliyor. Genellikle **XorShift**, **PCG(Permuted Congruential Generator)** gibi algoritmaların kullanılması ya da rastgele sayıları önce devasa bir diziye doldurup SIMD ile bu diziden vektörler halinde çekilmesi gibi yöntemler tercih ediliyor. Bunun kodunu yazmak için biraz daha araştırma yapmam lazım. Tekrardan rust tarafına dönelim.

DEVAM EDECEK...