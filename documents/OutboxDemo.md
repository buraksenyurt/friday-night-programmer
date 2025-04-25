# Outbox Pattern Demo

Bir bayi otomasyon sisteminde servis talep formu sürecinde Outbox Pattern'in nasıl kullanılabileceğine dair basit bir demo hazırlamak istedim. Senaryo gereği bir servis talep formu oluşturulduğunda bunu ele alan modül dışındaki farklı modüllerin de durumdan haberdar olması bekleniyor. Servis talep formunun içeriği bir veri tabanında saklanıyor. İşin içerisinde veritabanı varsa rollback'e neden olabilecek bir transaction hatası da söz konusu olabilir. Böyle bir senaryoda talep formu içeriğinin veri tabanından bir hata nedeniyle oluşamaması ama oluştuğuna dair diğer sistemlere örneğin event yoluyla bilgilendirme geçilmesi doğal olarak veri tutuarsızlığına, sürecin yanlış işletilmesine sebebiyet verecektir. Bu gibi durumlarda kullanılan çözümlerden bir tanesi Outbox Pattern ve bu demodaki amaç en basit haliyle nasıl işletileceğini görmek.

## Akış

Senaryomuza göre servis talep formunun kaydedileceği veritabanında Outbox verisini tutan bir başka tablo daha yer alır. Servis formu verisi ve outbox verisi her iki tabloya aynı transaction içerisinde yazılmaya çalışılır. Aynı veri tabanı üzerinde gerçekleşen bir transaction doğal olarak dağıtık sistem tabanlı bir transaction mekanizmasına göre daha garantidir. Bu sayede veritabanına veri ekleme ve gönderilecek mesajın kaydedilmesi tek bir ACID transaction ile atomik olarak gerçekleşebilir. Eğer transaction işlemi başarılı olursa hem ana veri hem de outbox mesajı kayıt edilmiş olur aksine rollback işlemi söz konusu olursa ikisi de gerçekleşmemiş olur. Sonraki adımda ayrı bir süreç _(arka planda çalışan bir job olabilir)_ Outbox tablosundaki mesajları okur ve RabbitMQ ya da benzeri bir mesajlaşma sistemine servis talep formunun oluşturulduğuna dair bilgi yayınlar. Mesaj başarılı bir şekilde kuyruğa gönderildiğinde ise Outbox tablosunda statü değişikliği yapılır ve hareketin taraflarca ele alındığı bilgisi onaylanmış olur. Sonuç olarak sistemler arası veri tutarlılığı sağlanır ve mesajların en az bir kez iletildiğinden emin olunur.

## Düzenek

Senaryodaki enstrümanları şöyle sıralayabiliriz.

- Servis Talep Formunu oluşturan çok basit bir **Web API** hizmeti
- Veritabanı olarak **Postgresql** 
- **ORM _(Object Relational Mapping)_** aracı olarak **Entity Framework**
- Mesaj kuyruğu olarak **RabbitMQ**
- **Outbox** tablosunu kontrol edecek periyodik iş için Quartz tabanlı bir uygulama
- Postgresql ve Rabbitmq tarafları için Docker-Compose 

Örnek kodları [şu klasörde](../src/OutboxDemo/) bulabilirsiniz.