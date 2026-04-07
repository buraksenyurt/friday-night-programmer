# Hangi Localization Tekniği

Tartışmanın konusu çooooook uzun zamandır dünyamızda var alan çoklu dil desteği. Kimi zaman veritabanı üzerinden kimi zaman fiziki dosyalardan *(resx gibi)* yönetmeye çalıştığımız bir mevzu. Sürekli değişip genişleyebilenler bir yana nadiren değişip genellikle statik kalanlar da işin bir başka yanı. Aslında temel amaç bir program arayüzünün veya kullanıcı ile etkileşimde olan taraflarının farklı dillere de destek vermesini sağlamak. Teori basit; değişmez sabit bir kavram *(key diyelim)* karşılığında kullanılan dile göre farklı değerler tutulmasını sağlamak. 

Örneğin müşteri bilgilerini kaydetme ekranında kullandığımız **button** kontrolünün başlığını **save_button** şeklinde bir **key** ile sabitleyip değerlerini ana dilimizde *kaydet*, İngilizce'de *save*, İspanyolca' da *Ahorrar* şeklinde tutabiliriz. Bunun için ister tablo tasarımı kullanalım ister bir **key:value** koleksiyon verinin okunması, değiştirilmesi, aynı anda erişilmesi gibi konular başka soruları da gündeme getirir. Nerede tutsak iyi olur, hangi teknik bizi ne kadar yavaşlatır/hızlandırır, eş zamanlı *(concurrent)* çağrılarda değişenlerin güncelliğini nasıl koruruz, **race-condition** oluşur mu, ön belleğe *(cache)* alsak ne zaman tazelemek gerekir, koca veriyi ön belleğe alma maliyeti nedir vb. 

Çoklu dil desteği aslında çözülmemiş bir problem değil. Birçok yazılım firması zaten çoktandır ideal çözümler üzerinden ilerlemekte. Bu çalışmadaki amacım veritabanı *(kuvvetle muhtemel Postgresql)*, redis, in-memory cache veya hibrit çözümler arasında bir benchmark ölçümü yapmak. Neticede aşağıdaki tabloda belirtilen sonuçları ispatlamaya ya da gerçeği yansıtıp yansıtmadığını bulmaya çalışacağız.

| **Yaklaşım** | **Read** | **Cold Start** Maliyeti | **Hot Read** Maliyeti |
| --- | --- | --- | --- |
| **PostgreSQL** *(doğrudan kullanım)* | Network + Disk | High | High |
| **Redis** | Network + RAM | Medium | Low |
| **IMemoryCache** *(in-process)* | RAM | Low (after warmup) | Lowest |
| **JSON** *(resx gibi dosyalarda)* | Disk | Low | Very Low |
| **Hybrid** *(DB → Redis → MemCache)* | Layered | Warm on demand | Lowest after L1 |

## Hazırlıklar

DEVAM EDECEK
