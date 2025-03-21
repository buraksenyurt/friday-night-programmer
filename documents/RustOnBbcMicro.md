# BBC Micro:bit - Rust ile Hello Light

Daha önceden Raspberry PI üzerinde Python dilini kullanarak tekerlek döndürmüştüm ancak Python ve kütüphanelerin sunduğu soyutlamalar ve RasPi' nin yetenekleri işi epeyce kolaylaştırmıştı. Bir süredir de gömülü sistemlerde Rust ile nasıl programlama yapılıyor merak etmekteydim. Rust dilinin çalışma sahası düşünüldüğünde C++ ile hareket edebileceğimiz her yerde geliştirme yapabileceğimizi söylesem yanılmam sanıyorum ki. Ancak burada bare metal programming konsepti sınırlarına giriliyor. Standart kütüphaneyi dışarıda bırakıp core kütüphane üzerinden ilerlemek ve sınırlı kapasiteye sahip ve hatta işletim sistemi dahi olmayan mikro denetleyicilerin farklılıklarını da göze alarak hareket etmek gerekiyor. Bana epey uzak bir konu olduğunu ifade edebilirim ancak en azından bir Hello World yazmak zorundayım ve hatta buna Hello Light desek daha iyi olabilir.

Internette gömülü sistemlerde Rust ile kodlama için sınırsız kaynak var ve hatta birincil kaynak olarak [The Embedded Rustacean](https://www.theembeddedrustacean.com/) sitesini tavsiye ederim. Haftalık bir bültenleri var ve oldukça sıkı makalelere yer veriyorlar. Lakin derli toplu ve kısa yoldan bir giriş yapmak isteyenler için bana göre birincil kaynak Rust Embedded organizasyonun [şu adresteki ücretsiz keşif kitabı](https://docs.rust-embedded.org/discovery/microbit/index.html). Bende bu kitabı baz alarak ilermek istedim diyebilirim. Saf zihnim ilk etapta bir emulator üzerinden hareket edebilirim yönündeydi. Hatta bu konuda oldukça güzel bir [çevrimiçi simülator sitesi](https://wokwi.com/rust) var. Takip etmekde olduğum Discovery kitabı konuyu [BBC micro:bit](https://microbit.org/) üzerinden ele alıyor. Bende heyecanla bu karttan bir tane aldım. ARM tabanlı bu mikro denetleyici için iki fotoğrafı da şöyle bırakayım.

![Micro:bit 00](../images/MicroBit_00.jpg)

ve

![Micro:bit 01](../images/MicroBit_01.jpg)

Bu ufacık kart öğrendiğim kadarıyla STEM müfredatında 7-14 yaş arası çocuklarımızın eğitimlerinde de kullanılıyor. Pyhton, scratch ve Microsoft MakeCode ile üzerinde geliştirme yapılabiliyor. Ben hiçbir ekstra sensör veya genişletme yuvası almadım. Buna rağmen kartın üstünde programlanabilir led'ler _(ki ilk açıldğında kırmızı bir kalp şekline aldılar)_, yerleşik bir hoparlör, iki yön tuşu, bluetooth, hareket sensörü vs yer alıyor. Benim tedarik ettim v2.2 modelinde 512 Kb Flash ve 128 Kb RAM var. nRF52833 kodlu işlemci ise Nordic Semiconductor üretimi. Bilgisayara kolayca USB kablosu ile bağlanabiliyor.

## İşin Zor Kısımları

En çok zorlandığım kısım cihaz üzrerinde gerekli geliştirmeleri yapabilmek için gerekli ortam araçlarını kurmak oldu. Öncelikle rust ile yazılan çıktının ARM tabanlı bu işlemci modeli için build edilmesi, cihaz üzerine bir şekilde aktarılması söz konusu. Ayrıca yer yer debug etmek de gerekebilir ki bunun için de bazı araçlara ve kurulumlara ihtiyaç var. Burada tavsiyem [kitaptaki ilgili bölümü](https://docs.rust-embedded.org/discovery/microbit/03-setup/index.html) harfiyen takip edip kendi ortamınız için gerekli kurulumları yapmanız olacak. Neler neler çektim bi bilseniz :D
