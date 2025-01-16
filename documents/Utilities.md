# Utilities

Friday Night Programmer reposunda sene boyunca üzerinde çalışılacak uygulamalar için bazı yardımcı enstrümanlara ihtiyaç olacaktır. Örneğin veri sağlayan servisler, veri tabanları, hazır docker imajları gibi. İhtiyaçlar ortaya çıktıkça bu tip araçlarla ilgili bilgileri bu dokümanda toplayabiliriz.

## The Best Supporter Of Nature Servisi

UC00 kodlu Çağrı Merkezi Çözümleri vakasında farklı segmentten müşteri verilerine ihtiyacımız var. Finans kurumları, sigorta firmaları, oto yedek parça satıcıları gibi ürünleri ile müşterilerine ekstra hizmetler sunabilecek çeşitli şirketler. Vaka çalışmasının asıl amacı bir SDK'nın nasıl yazılabileceğini analiz etmek olduğundan ihtiyaç duyulan çeşitli dummy servisler var. **The Best Supporter Of Nature** servisini de buna hizmet eden bir aracı olarak düşünebiliriz. Pek tabii bu servisin arkasında da bir iş modeli var;

İş modeline göre servisi veren firma aslında elektronik atıkları topluyor ve tekrardan hammaddeye dönüştürüyor ya da sağlıklı ve kalıcı şekilde yok ediyor. Hedef müşteri kitlesi bireysel kullanıcılar. Yani büyük kurumlardan ziyade bireysel olarak evindeki elektronik atıkları getiren kişiler. Her ay sonunda bir çevreye en duyarlı on kişinin belirlendiği bir liste hazırlanıyor ve bu liste çağrı merkezimiz tarafında çekiliyor. Kazananlara birer bilgilendirme mesajı gönderiliyor ve bir ay boyunca Everything Everywhere Şarküteri marketler zincirinden çevrimiçi yapacakları alışverişlere yüzde beş indirim uygulanıyor. Çağrı merkezi personelinin burada aranan taraf olmadığını ifade edebiliriz ama SDK'nin bir notification hizmeti de içereceğini belirtebiliriz. En nihayetinde bize bu tip verileri rastgele değerlerle sağlayacak bir servise ihtiyacmız var. Bu uygulamayı REST tabanlı bir servis olarak tasarlayabiliriz. Hatta kendi sistemlerimizde sıksık kullanacağımız dummy bir servis olarak düşünürsek bir docker container içerisinde saklayabiliriz.

_Servis sayılarının çoğalması durumunda belki bir Service Farm kurabiliriz ve orkestrasyonu için Docker Compose ya da daha üst seviye farklı bir çözüme gidebiliriz._

The Best Support Of Nature servisini csharp klasöründe **EcoFriendlyApi** adıyla bulabilirsiniz.
