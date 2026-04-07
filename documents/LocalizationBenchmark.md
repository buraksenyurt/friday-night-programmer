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

Öncelikle birkaç yaklaşımız olduğunu belirtelim. Çoklu dil desteği için dikeyde büyüyen bir tabloyu postgresql üzerinde tutacağız. Bir diğer yaklaşımda dağıtık sistemlerin en karizma caching ürünlerinden olan **redis** ile ilerleyeceğiz. .Net'in dahili bellek kullanımı da yabana atılır türden değil. Dolayısıyla o da işin içerisine giriyor. Bir başka tercih de disk üzerinde bildiğimiz **JSON** tadında tutmak gibi belki bunu yaml ile değiştirebiliriz ya da bir markdown dosyasıyla. Elbette hibrit bir çözüm de deneyeceğiz, all together :D

### Docker Setup

Düzeneğimizi `**.Net 10** platformunda kurgulayabiliriz. **Postgresql** ve **Redis** enstrümanları için her zaman olduğu gibi bir **docker-compose** dosyası iş görecektir. En azından aşağıdaki içeriğe sahip olmasında yarar var.

```yml
services:

  postgres:
    image: postgres:latest
    container_name: locally-postgres
    environment:
      POSTGRES_USER: johndoe
      POSTGRES_PASSWORD: somew0rds
      POSTGRES_DB: postgres
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgres/data
    networks:
      - locally-network

  pgadmin:
    image: dpage/pgadmin4:latest
    container_name: locally-pgadmin
    environment:
      PGADMIN_DEFAULT_EMAIL: scoth@tiger.com
      PGADMIN_DEFAULT_PASSWORD: 123456
    ports:
      - "5050:80"
    depends_on:
      - postgres
    networks:
      - locally-network

  redis:
    image: redis:latest
    container_name: locally-redis
    ports:
      - "6379:6379"
    networks:
      - locally-network

volumes:
  postgres_data:

networks:
  locally-network:
    driver: bridge
```

### Solution İskeleti

Solution içeriğinde birkaç proje yer alacak. Farklı provider türlerimiz olacağı için çok basit soyutlamalar kullanmakta, benchmark ölçümleri için ayrı bir proje kullanmakta ve testleri bir web api üzerinden icra etmekte yarar var. Buna göre solution içeriği ve gerekli nuget paketlerini aşağıdaki script'te olduğu gibi hazırlayabiliriz.

```bash
mkdir LocalizationChallenge
cd LocalizationChallenge

dotnet new sln

dotnet new classlib -n LocalizationChallenge.Core
dotnet sln add LocalizationChallenge.Core/

dotnet new classlib -n LocalizationChallenge.Infrastructure
dotnet add LocalizationChallenge.Infrastructure/LocalizationChallenge.Infrastructure.csproj package Npgsql
dotnet add LocalizationChallenge.Infrastructure/LocalizationChallenge.Infrastructure.csproj package StackExchange.Redis
dotnet add LocalizationChallenge.Infrastructure/LocalizationChallenge.Infrastructure.csproj package Microsoft.Extensions.Hosting.Abstractions
dotnet sln add LocalizationChallenge.Infrastructure/

dotnet new classlib -n LocalizationChallenge.Benchmarks
dotnet add LocalizationChallenge.Benchmarks/LocalizationChallenge.Benchmarks.csproj package BenchmarkDotNet
dotnet add LocalizationChallenge.Benchmarks/LocalizationChallenge.Benchmarks.csproj package Npgsql
dotnet add LocalizationChallenge.Benchmarks/LocalizationChallenge.Benchmarks.csproj package StackExchange.Redis
dotnet sln add LocalizationChallenge.Benchmarks/

dotnet new webapi -n LocalizationChallenge.Api
dotnet add LocalizationChallenge.Api/LocalizationChallenge.Api.csproj package Npgsql
dotnet add LocalizationChallenge.Api/LocalizationChallenge.Api.csproj package StackExchange.Redis  
dotnet sln add LocalizationChallenge.Api/

# Gerekli proje referansların eklenmesi
dotnet add LocalizationChallenge.Infrastructure/LocalizationChallenge.Infrastructure.csproj reference LocalizationChallenge.Core/LocalizationChallenge.Core.csproj
dotnet add LocalizationChallenge.Api/LocalizationChallenge.Api.csproj reference LocalizationChallenge.Core/LocalizationChallenge.Core.csproj
dotnet add LocalizationChallenge.Api/LocalizationChallenge.Api.csproj reference LocalizationChallenge.Infrastructure/LocalizationChallenge.Infrastructure.csproj
dotnet add LocalizationChallenge.Benchmarks/LocalizationChallenge.Benchmarks.csproj reference LocalizationChallenge.Core/LocalizationChallenge.Core.csproj
dotnet add LocalizationChallenge.Benchmarks/LocalizationChallenge.Benchmarks.csproj reference LocalizationChallenge.Infrastructure/LocalizationChallenge.Infrastructure.csproj
```

DEVAM EDECEK

### Postgresql Tarafı

Veritabanı tarafında bir tabloya ve en azından tohumlamaya ihtiyacımız var. Başlangıç için aşağıdaki script'i kullanabiliriz. *(İlerleyen aşamada belki bir trigger ekleyip olası değişimleri dış dünyaya push'layacağımız bir mekanizmayı da ele alırız ki **cache-invalidation** noktasında gerekli olabilir)*

```sql
CREATE TABLE IF NOT EXISTS localizations (
    id           SERIAL       PRIMARY KEY,
    culture      VARCHAR(10)  NOT NULL,
    resource_key VARCHAR(255) NOT NULL,
    value        TEXT         NOT NULL,
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_culture_key UNIQUE (culture, resource_key)
);

CREATE INDEX IF NOT EXISTS idx_loc_culture_key ON localizations (culture, resource_key);

-- Sample data
INSERT INTO localizations (culture, resource_key, value) VALUES
  ('en-US', 'welcome_message',    'Welcome to our application'),
  ('en-US', 'farewell_message',   'Goodbye, see you soon'),
  ('en-US', 'error_not_found',    'The requested resource was not found'),
  ('en-US', 'error_unauthorized', 'You are not authorized to perform this action'),
  ('en-US', 'button_save',        'Save'),
  ('en-US', 'button_cancel',      'Cancel'),
  ('en-US', 'button_delete',      'Delete'),
  ('tr-TR', 'welcome_message',    'Uygulamamıza hoş geldiniz'),
  ('tr-TR', 'farewell_message',   'Güle güle, yakında görüşürüz'),
  ('tr-TR', 'error_not_found',    'İstenen kaynak bulunamadı'),
  ('tr-TR', 'error_unauthorized', 'Bu işlemi gerçekleştirme yetkiniz yok'),
  ('tr-TR', 'button_save',        'Kaydet'),
  ('tr-TR', 'button_cancel',      'İptal'),
  ('tr-TR', 'button_delete',      'Sil'),
  ('de-DE', 'welcome_message',    'Willkommen in unserer Anwendung'),
  ('de-DE', 'farewell_message',   'Auf Wiedersehen, bis bald'),
  ('de-DE', 'error_not_found',    'Die angeforderte Ressource wurde nicht gefunden'),
  ('de-DE', 'error_unauthorized', 'Sie sind nicht berechtigt, diese Aktion durchzuführen'),
  ('de-DE', 'button_save',        'Speichern'),
  ('de-DE', 'button_cancel',      'Abbrechen'),
  ('de-DE', 'button_delete',      'Löschen')
ON CONFLICT (culture, resource_key) DO NOTHING;
```

## Kod Tarafı

Şimdi adım adım kodlarımızı geliştirmeye başlayalım.

DEVAM EDECEK

## Değişiklikleri Algılama

EKLENECEK

## k6 Benchmark

EKLENECEK

DEVAM EDECEK
