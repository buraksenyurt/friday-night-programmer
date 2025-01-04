# Use Cases

Burada sezon boyunca üzerinde durulabilecek örnek senaryolara ait detaylar yer almaktadır. Senaryolar UC kodları ile işaretlenmiştir.

## UC00 : Çağrı Merkezi Çözümleri

Bir çağrı merkezimiz var. Operatörlerin kullandığı uygulamanın ilk sürümü geliştiriliyor. Tarihler milenyuma bir kalayı göstermekte. Çağrı merkezindeki operatörler windows tabanlı bir uygulama kullanıyorlar. Müşteri verileri farklı firmalardan, farklı zaman dilimlerinde farklı depolama cihazları üzerinden geliyorlar. Örneğin müşteri portföyü nispeten küçük olan bir sigorta firması verisini diskete alıp mektupla şirkete yolluyor. Bunu ayda bir gerçekleştiriyor. Bir başka finans kurumu ise yüksek müşteri portföyünü ayda bir olmak suretiyle CD ile yolluyor. Her kurumsal müşterinin yolladığı bilgiler farklı formatlarda ve farklı şema yapılarına sahip durumda. Program ilk yıllarında büyük müşteri verilerini dBase veri tabanlarında başarılı şekilde kullanabiliyor. Ancak zamanla sistem yeni kurumsal müşteriler dahil oluyor ve veri giderek büyüyor. Sonuçta Oracle veri tabanına geçilmesine karar veriliyor.

Sistem migration planlarından sonra birkaç yıl daha sorunsuz çalışıyor. Çağrı merkezindeki operatör sayısı üç katına çıkıyor ancak tüm sistem aynı kattaki bir network üzerinden yürümeye devam ediyor. Ne yazık ki bir süre sonra korkunç salgın başlıyor ve pandemi ilan ediliyor. İnsanlar evlere kapanırken çağrı merkezi personelinin operasyonu evdeki bilgisayarlarından yürütmesi gerekiyor. Kısa süre içerisinde tüm uygulamanın web tabanlı bir sisteme geçmesi gerekiyor. Çağrı merkezinin anlaşmalı olduğu kurumsal müşteriler modern teknolojilerden yararlanarak portföylerini artık servisler aracılığıyla iletebileceklerini belirtiyor ancak veri boyutları oldukça büyük bu nedenle sadece değişen ve yeni eklenen müşteri verilerinin çekileceği bir sisteme ihtiyaç var.

Zamanla uygulamadaki teknik borç yükünün arttığı fark ediliyor. Şirket oldukça şanslı ki kendilerine ürünü yenilemeleri için iyi bir bütçe ve kaynak ayrılıyor. Var olan sistemi günümüz teknolojileri ile yeniden yazmamız bekleniyor.

En başında itibaren yeni sürüme kadar çıkabilecek sadece ufak değişiklilerle yeni teknolojilere adapte edilebilecek bir çatı _(Framework)_ geliştirilebilir miydi?

## UC01 : ???
