# Programcı AI Araçları için Örnek Kdolar

Şu anda birçok AI aracına kod yazdırabiliyoruz. Claude, ChatGPT, DeepSeek, Grok ve daha nicelelerine. Hatta Cursor AI ve replit de bunlar arasında sık kullanılan ve kodlama alanında uzmanlaşmış olanlar diyebiliriz. Eğer Microsoft ailesinden gelen ürünler kullanıyorsak sanırım Github Copilot' ta en çok başvurduklarımız arasında. Birçok kaynakta bu araçların başarım oranlarının ölçümü için kendilerini örnek kodlar yazdırıldığından bahsediliyor. Bende çok basit bir deneme yapmak istedim. Github Copilot' da bu notları düştüğüm tarih itibariyle Claude 3.5 Sonnet Preview'dan bana üç farklı faktöryel hesaplama fonksiyonu yazmasını istedim. İlk üretilen kodları aşağıdaki gibi oldu.

```csharp
// 1. Recursive factorial
static long FactorialRecursive(int n)
{
    if (n <= 1) return 1;
    return n * FactorialRecursive(n - 1);
}

// 2. Iterative factorial
static long FactorialIterative(int n)
{
    long result = 1;
    for (int i = 2; i <= n; i++)
    {
        result *= i;
    }
    return result;
}

// 3. LINQ-based factorial
static long FactorialLinq(int n)
{
    return n <= 1 ? 1 : Enumerable.Range(2, n - 1).Aggregate((long)n, (acc, val) => acc * val);
}

// Test the implementations
int number = 5;
Console.WriteLine($"Factorial of {number} (Recursive): {FactorialRecursive(number)}");
Console.WriteLine($"Factorial of {number} (Iterative): {FactorialIterative(number)}");
Console.WriteLine($"Factorial of {number} (LINQ): {FactorialLinq(number)}");
```

Ağırlıklı olarak çoğumuz recursive veya iteratif olan versiyonu yazarız sanıyorum ki _(En azından benim için böyle)_ Esasında ona şöyle bir prompt vermiştim; "Could you write 3 different factorial calculation functions in this file?" Aslında daha farklı bir implementasyon bekliyorum bakalım ona ulaşabilecek miyiz? Sonrasında soruyu değiştirdim ve şöyle dedim; "Would you change some of them with more efficient version?" Buna karşılık fonksiyonları şöyle değiştirdi.

````csharp
// 1. Optimized iterative factorial with overflow checking
static long FactorialFast(int n)
{
    if (n < 0) throw new ArgumentException("Negative numbers are not supported", nameof(n));
    if (n <= 1) return 1;

    checked // Enables overflow checking
    {
        long result = 1;
        for (int i = 2; i <= n; i++)
        {
            result *= i;
        }
        return result;
    }
}

// 2. Span-based factorial for better memory usage
static long FactorialSpan(int n)
{
    if (n < 0) throw new ArgumentException("Negative numbers are not supported", nameof(n));
    if (n <= 1) return 1;

    Span<long> numbers = stackalloc long[n];
    numbers[0] = 1;
    
    for (int i = 1; i < n; i++)
    {
        numbers[i] = numbers[i - 1] * (i + 1);
    }
    
    return numbers[n - 1];
}

// 3. Lookup table factorial for small numbers (very fast for n ≤ 20)
static class FactorialLookup
{
    private static readonly long[] Factorials = {
        1L,                   // 0!
        1L,                   // 1!
        2L,                   // 2!
        6L,                   // 3!
        24L,                  // 4!
        120L,                 // 5!
        720L,                 // 6!
        5040L,               // 7!
        40320L,              // 8!
        362880L,             // 9!
        3628800L,            // 10!
        39916800L,           // 11!
        479001600L,          // 12!
        6227020800L,         // 13!
        87178291200L,        // 14!
        1307674368000L,      // 15!
        20922789888000L,     // 16!
        355687428096000L,    // 17!
        6402373705728000L,   // 18!
        121645100408832000L, // 19!
        2432902008176640000L // 20!
    };

    public static long GetFactorial(int n)
    {
        if (n < 0) throw new ArgumentException("Negative numbers are not supported", nameof(n));
        if (n >= Factorials.Length) 
            throw new ArgumentException($"Factorial too large (n > {Factorials.Length - 1})", nameof(n));
        
        return Factorials[n];
    }
}

// Test the implementations
int number = 5;
Console.WriteLine($"Factorial of {number} (Fast): {FactorialFast(number)}");
Console.WriteLine($"Factorial of {number} (Span): {FactorialSpan(number)}");
Console.WriteLine($"Factorial of {number} (Lookup): {FactorialLookup.GetFactorial(number)}");

```

Evet negatif değerler veya taşma hatalarını kontrol altına alıp sonuçları bilenen çok büyük bazı sayılar için tedbirlre almıştı. En nihayetinden asıl meramımı dile getirdim ve şöyle yazdım; "Why didn't use memoization technique?" Yani aslında şu anda kadar ki örneklerden en az birisinde daha önceden hesaplanmış sayıları yeninden hesaplamaya gerek kalmadan aklından getirmesi ile ilgili Memoization pratiğini uygulayacağı bir teknik kullanmasını bekliyordum. Buna istinaden ürettiği fonksiyon da şöyle oldu.

```csharp
// 4. Memoization-based factorial
static class FactorialMemoized
{
    private static readonly Dictionary<int, long> Cache = new()
    {
        { 0, 1 }, // Initialize with base case
        { 1, 1 }  // Initialize with base case
    };

    public static long Calculate(int n)
    {
        if (n < 0) throw new ArgumentException("Negative numbers are not supported", nameof(n));

        if (Cache.TryGetValue(n, out long result))
            return result;

        checked
        {
            result = n * Calculate(n - 1);
            Cache[n] = result;
            return result;
        }
    }
}

// Add this to your test section
Console.WriteLine($"Factorial of {number} (Memoized): {FactorialMemoized.Calculate(number)}");
```

Elbette memoization tekniği hız anlamında avantajlar sağlasa da bazı handikalparı olduğu da bir gerçek. Büyük sayılarda memory tüketimini artırabilir ayrıca daha önceki örnekte verdiği gibi bir lookup table kullanmak memoization'a göre çok daha iyi performans kazanımı sağlayabilir. Kendisinin bu konudaki yorumu da tam olarak şöyle oldu. Neden kullanmadığını hangi durumlarda kullanılması gerektiğini önermekte.

```text
However, there are a few reasons why memoization might not have been included in the initial optimized versions:
1.	Memory overhead: Memoization trades memory for speed by storing all previously calculated values
2.	For single calculations, memoization adds overhead without benefit
3.	The lookup table approach is actually a form of complete pre-computation, which is even faster than memoization for small numbers
4.	Factorial calculations are typically not repeated enough in most applications to justify memoization's overhead
The memoization approach becomes more valuable when:
•	You need to calculate many factorials repeatedly
•	The calculations happen across different method calls
•	Memory usage is less concerning than CPU time
```

Meali;

```text
Ancak, başlangıçta optimize edilmiş sürümlerde belleklemenin dahil edilmemiş olmasının birkaç nedeni olabilir:
1. Bellek yükü: Bellekleme, daha önce hesaplanmış tüm değerleri depolayarak bellekten hız elde eder
2. Tekil hesaplamalar için bellekleme, fayda sağlamadan ek yük ekler
3. Arama tablosu yaklaşımı aslında küçük sayılar için belleklemeden daha hızlı olan tam bir ön hesaplama biçimidir
4. Faktöriyel hesaplamaları, belleklemenin ek yükünü haklı çıkarmak için çoğu uygulamada genellikle yeterince tekrarlanmaz
Bellekleme yaklaşımı şu durumlarda daha değerli hale gelir:
• Birçok faktöriyeli tekrar tekrar hesaplamanız gerektiğinde
• Hesaplamalar farklı yöntem çağrıları arasında gerçekleştiğinde
• Bellek kullanımı, CPU süresinden daha az endişe verici olduğunda
```

Daha karmaşık algoritma veya problemler için çözüm önerileri de istenebilir. Kodlama alanında öne çıkan AI araçlarına genel olarak aşağıdaki görevleri yaptırmayı deneyebiliriz gibi;

- Karmaşık bir kod bloğunun ne yaptığını anlattırmak
- Build hatası veren bir kodun neden bu hatayı verdiğini sormak, düzeltmesini istemek
- Runtime hatalarını çözmesini istemek
- Belli fonksiyonları olan bir kod dosyasının tamamlanmasını istemek
- Kod için dile özel dokümantasyonları çıkarttırmak
- Birim test senaryolarını yazdırmak

Elbette tüm bunları yaptırmak için yazılım konusunda iyi bilgiye sahip olmak önemli. Ayrıca sonuçları da kontrol etmek, sorgulayabilmek ve yorumlayabilmek de gerekiyor.