# .Net ile Bir MCP Server Yazmak

**MCP *(Model Context Protocol)***, yapay zeka araçları için tool desteği sağlamak amacıyla kullanılan bir protokol olarak düşünülebilir. Anthropic tarafından geliştirilmiş bir standarttır ki detaylarına [buradan](https://github.com/modelcontextprotocol) ulaşabilirsiniz. Bu protokolün geliştirilmesinde amaç yapay zeka araçlarına belli bir standart dahilinde harici araç desteği sunabilmektir. Genel senaryoda bir dil modeline gitmeden önce bu protokol üzerinden hizmet veren **MCP Server**`lara gidilerek sağlanan araçlar kullanılabilir. Araçlar da arka planda çoğunlukla **REST API** hizmetlerini çağırır ama bu zorunluluk değildir. Bir başka deyişle **MCP server**'ın sağladığı araç seti arka planda sarmalladığı herhangi bir başka araca da gidebilir. **MCP**, yapay zeka araçları için standart bir protokol sunduğundan tüm **MCP server**'lar bir yapay zeka aracı tarafından çağırılabilir.

Çok basit ve tek yönlü bir senaryo ile konuyu anlamaya çalışalım: Sisteminizde binlerce servis olduğunu ve bu servislerle ilgili keşif dokümanlarının **yaml** formatlı dosyalarda tutulduğunu düşünelim. Tabii o kadar çok **yaml** içeriği var ki bunlarla alakalı istatistik toplayan, bilgi veren bir de REST Api hizmetimiz var. Bir **MCP server** ile bu API hizmetindeki fonksiyonellikleri birer araç *(tool)* olarak tanımlamak ve çağırmak mümkündür. Senaryoya dahil olan bir MCP istemcisi *(Örneğin VS Code veya farklı bir kod geliştirme ortamındaki AI asistanı, Github CLI veya Claude CLI gibi komut satırı araçları ya da bizim yazacağımız basit bir chatbot)*, MCP server'ımıza bağlanarak araçlarımızı çağırabilir ve bu çıktıları değerlendirerek cevap vermesi için bir dil modeline gidebilir. Söz gelimi bu ortamlarda aşağıdaki gibi sorular sorabiliriz;

- "Sistemimde kaç tane mongodb veritabanı kullanan servis var?"
- "Sipariş yönetimi süreçlerinde kullanılan servisler neler?"
- "Sipariş yönetimi süreçlerinde kullanılan servislerin YAML dokümanlarını getir."
- "5'ten fazla bağımlılığı olan servisleri getir."
- "Müşteri modülüne hizmet eden servislerin listesini getir."

Burada biraz durup düşündüğümüzde, *yahu bu bilgileri bir SQL tablosunda tutup sorgulayan bir servis ve arabirim de yapabilirdik* ya da *bu kadar zahmete ne gerek var, zaten elimizde bir REST API var, onun üzerinden sorgulama yaparız* diye de düşünebiliriz. Bu sık rastlanan bir kafa karışıklığıdır zira MCP server'lar ile REST Api'ler birbirlerine çok benzerler. MCP server'ların REST API'lerden farkı, yapay zeka araçlarının çağırabileceği araç setlerini endüstriyel bir standart üzerinden sunmasıdır. MCP server'lar, yapay zeka araçlarının ihtiyaç duyduğu bilgileri sağlamak için tasarlanmışlardır. MCP standardı, onların kolayca keşfedilmesi *(discover)* ve çağırılmasını da sağlar. Zaten bilinen veya gördüğüm MCP server'lar standart araçların arkasında hep REST API hizmetleri barındırmakta. Ne var ki bu REST API'ler doğrudan yapay zeka araçları tarafından çağrılamazlar, çünkü yapay zeka araçları standart bir protokol üzerinden tanımlanmış araçlara ihtiyaç duyar.

Tabii bu senaryoda tek yönlü bir anlatım söz konusu. Yani, yapay zeka aracı ile kullandığı MCP server daha çok *ask* modunda çalışıyor. Diğer yandan tam ters yönde aksiyonlar gerçekleştirmek de mümkün. Yani bir MCP server aracılığı ile bir görev işletmek, bir süreci tetiklemek, bir uygulamayı başlatmak gibi aksiyonlar da alınabilir. Örneğin, bir **MCP server** üzerinden bir **CI/CD pipeline**'ını tetiklemek ya da projeye yeni bir fonksiyonellik eklemek veya bir board'da task oluşturmak mümkün olabilir. Tüm bunlar MCP server'ın sunduğu araç setine ve arka planda çalışan rest api gibi çeşitli hizmetlere bağlıdır.

> Neden MCP Server yazarız? Büyük dil modelleri *(Large Language Models)*, devasa veri setlerine sahiptir ama bunlara görece sabittir ve içeriklerini de çoğunlukla bilmeyiz. Ne zaman güncellendiklerini dahi bilmeyiz ki çoğu internet üzerinden anlık arama da yapabilirler. Eğer bahsettiğimiz senaryolara benzer kurgularda iyi sonuçlar almak istiyorsak, yapay zeka modellerine gitmeden önce ihtiyaç duyacakları bilgileri sağlayacak *MCP server*'lar yazmak ve kullanmak iyi bir fikir olabilir.

## Genel Mimari Konsept

MCP Server kullanılan senaryolarda aşağıdaki unsurlar yer alır.

- **MCP Host:** MCP sunucusuna bağlanan, sağladığı araç setini keşfedebilen ve kullanabilen bir uygulama olarak düşünülebilir. VS Code'daki AI asistanları, Github CLI veya Claude CLI gibi komut satırı araçları, chatbot'lar gibi uygulamalar bu kategoriye girer.
- **MCP Client:** MCP sunucusuna olan bağlantıları yöneten ve host için gerekli *context* bilgilerini sağlayan bir bileşen olarak düşünülebilir. **Context**, LLM'ler için çok önemli bir kavramdır.
- **MCP Server:** Tahmin edileceği üzere istemcilere sahip olduğu araç seti üzerinden *context* sağlama görevini üstlenir.

Kurguyu aşağıdaki çizelge ile de özetleyebiliriz.

![MCP Kroki](../images/MCPServer_00.png)

Konuyu bir örnek üzerinden irdelediğimizde her şey daha kolay anlaşılacaktır diye düşünüyorum.

## Senaryo

Kobay olarak bir **todo** listesi ile ilintili API servisi kullanan bir **MCP server** yazıp kullanmayı deneyelim. Todo listemizi postgresql'da duran bir veritabanı üzerinde düşünelim. Pahalı oldu ama işe yarar :D Normalde bu veri setinden sorgulama yapmak veya yeni bir todo eklemek, güncelleme veya silmek için standart bir yol izleriz. Kullanıcı dostu bir önyüz uygulaması, onun çağırdığı ve bu bahsedilen hizmetleri yerine getiren kuvvetle muhtemel bir rest servisi. Amacımız bir MCP sunucusu yazıp nasıl kullanıldığını deneyimlemek olduğundan todo listesinin ele alındığı projenin çapı şu an için çok da önemli değil. Ancak aşağıdakilere benzer işleri icra edeceğimiz bir senaryo kurgulamaya çalışacağız.

- Soru sorup bilgi alabilelim:
  - "Todo listemdeki tüm görevleri getir?"
  - "Yapılacaklar listemdeki tamamlanmış görevler neler?"
  - "Yapılacaklar listemdeki tamamlanmamış görevler neler?"
- Veri manipülasyonları:
  - "Yapılacaklar listeme 'çalışma odasını temizle' görevini ekle. Bu görev 20 Haziran 2024 tarihine kadar tamamlanmalı. Küçük bir zorluk derecesine sahip"
  - "Süresi geçmiş olan görevleri undone olarak güncelle."
  - "Yapılacaklar listemdeki 'çalışma odasını temizle' görevini yapıldı olarak güncelle."
  - "'MCP Server Nasıl Çalışır?' başlıklı makale yazma görevimi inprogress olarak güncelle."

## Önce Arka Plan Hazırlıkları

Veritabanı tarafı ile olan iletişim **rust** ile yazılmış bir api servisi ile karşılayacağız. Postgresql için aşağıdaki içeriğe sahip bir **docker-compose** kullanılabilir.

```yaml
services:

  postgres:
    image: postgres:latest
    container_name: fnp-postgres
    environment:
      POSTGRES_USER: johndoe
      POSTGRES_PASSWORD: somew0rds
      POSTGRES_DB: postgres
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgres/data
    networks:
      - fnp-network

  pgadmin:
    image: dpage/pgadmin4:latest
    container_name: fnp-pgadmin
    environment:
      PGADMIN_DEFAULT_EMAIL: scoth@tiger.com
      PGADMIN_DEFAULT_PASSWORD: 123456
    ports:
      - "5050:80"
    depends_on:
      - postgres
    networks:
      - fnp-network

volumes:
  postgres_data:

networks:
    fnp-network:
        driver: bridge
```

Bu servisleri ayağa kaldırmak komut satırından aşağıdaki gibi ilerleyebiliriz.

```bash
docker compose up -d
# Eğer docker compose dosyası benimki gibi kalabalık ve sadece bu servisleri ayağa kaldırmak isterseniz
docker compose up -d postgres pgadmin
```

Bu yazımızla çok alakalı olmadığı ve konuyu dağıtacağı için Rust ile yazılmış servis kodlarına burada girmeye gerek yok ancak [şuradan](https://github.com/buraksenyurt/friday-night-programmer/tree/main/src/todo-api) kaynak kodlarını inceleyebilirsiniz.

## MCP Server Tarafı

Gelelim ana mevzuya. Derme çatma da olsa todo listesini yönetebildiğimiz basit bir rest servisimi bulunuyor. Bu servis en azından yukarıdaki senaryoda belirtilen işlevleri sağlamakta *(Gerçek bir saha kurgusunu hazırlarken ilk olarak soruları ve görevleri içerecek bir metin çalışması yapmak, mcp araç setinden sunulacak fonksiyonellikleri doğru şekilde tasarlamak adına önemli olacaktır)*

DEVAM EDECEK...
