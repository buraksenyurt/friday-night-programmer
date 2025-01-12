# Use Cases

Burada sezon boyunca üzerinde durulabilecek örnek senaryolara ait detaylar yer almaktadır. Senaryolar UC kodları ile işaretlenmiştir.

## UC00 : Çağrı Merkezi Çözümleri

Bir çağrı merkezinin işleyişini düşünelim. Operatörlerin kullandığı uygulamanın ilk sürümü geliştiriliyor. Tarihler milenyuma bir kalayı göstermekte. Çağrı merkezindeki operatörler windows işletim sistemi için yazılmış bir masaüstü uygulama kullanıyorlar. Müşteri verileri farklı firmalardan farklı zaman dilimlerinde ve farklı depolama cihazları üzerinden geliyorlar. Örneğin müşteri portföyü nispeten küçük olan bir sigorta firması verisini diskete alıp mektupla gönderiyor ve bunu ayda bir gerçekleştiriyor. Bir başka finans kurumu ise yüksek müşteri portföyünü ayda bir olmak suretiyle CD ile yolluyor. Hatta farklı bir firma nispeten ilk sisteme aktarılan müşterilerinde gerçekleşen güncellemeleri içeren bir e-postayı firmaya gönderiyor. Her kurumsal müşterinin yolladığı bilgiler farklı formatlarda ve farklı şema yapılarına sahip. Program ilk yıllarında büyük müşteri verilerini dBase, Paradox ve benzeri veri tabanlarında yönetebiliyor ancak zamanla sisteme yeni kurumsal müşteriler dahil oluyor ve kullanılacak veri boyutu giderek büyüyor. Sonuçta Oracle veri tabanına geçilmesine karar veriliyor. Bu senaryoyu 1999 yılı itibariyle aşağıdaki gibi hayal edelim.

![1999 Senaroysu](./images/Scenario1999.png)

Sistem migration planlarından sonra birkaç yıl daha sorunsuz çalışıyor. Çağrı merkezindeki operatör sayısı üç katına çıkıyor ancak tüm sistem aynı kattaki bir network üzerinden yürümeye devam ediyor. Ne yazık ki bir süre sonra korkunç salgın başlıyor ve pandemi ilan ediliyor. İnsanlar evlere kapanırken çağrı merkezi personelinin operasyonu evdeki bilgisayarlarından yürütmesi gerekiyor. Kısa süre içerisinde tüm uygulamanın web tabanlı bir sisteme geçmesine karar veriliyor. Çağrı merkezinin anlaşmalı olduğu kurumsal müşteriler modern teknolojilerden yararlanarak portföylerini artık servisler aracılığıyla iletebileceklerini belirtiyor ancak veri boyutları oldukça büyük ve bu nedenle sadece değişen ve yeni eklenen müşteri verilerinin çekileceği bir sisteme ihtiyaç var gibi duruyor. O vakitlerde firmalar bazı müşteri bilgilerini XML Web servisler ve hatta REST tabanlı api'ler araclığı ile de vermeye başlıyorlar. Arada yine işi farklı şekillerde çözen örneğin iki kurum arasında güvenli olarak kullanılabilen bir FTP sunucusuna dosya bırakanlar da var. Bu yeni duruma göre çağrı merkezinde müşteri verilerinin tutulmasına da gerek olmayabilir lakin bu birazda iş modelinin yürütülüşüne bağlı. Neyse ki biz bu detayları çok fazla kurcalamıyoruz ve senaryomuz aşağıdaki şekline bürünmeye başlıyor.

![2004 Sonrasi Senaryosu](./images/Scenario2004.png)

Zamanla uygulamadaki teknik borç yükünün arttığı fark ediliyor. Şirket oldukça şanslı ki kendilerine ürünü yenilemeleri için iyi bir bütçe ve kaynak ayrılıyor. Var olan sistemi günümüz teknolojileri ile yeniden yazmamız bekleniyor.

En başında itibaren yeni sürüme kadar çıkabilecek sadece ufak değişiklilerle yeni teknolojilere adapte edilebilecek bir çatı _(Framework)_ geliştirilebilir miydi? Müşteri verilerinin çağrı merkezi uygulaması tarafından ele alınmasında ne tür senaryolar ele alınabilirdi?

## UC01 : ???
