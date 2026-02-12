# RAG - Retrieval Augmented Generation İçin Hazırlıklar

Senaryom şöyle; Mimari tasarımı, domain yapısı belli olan bir çözümde Copilot ajanlarını kullanırken yapılan geliştirme/analiz taleplerinde genel dil modelinin RAG ile çizilen çerçeve içerisinde kalmasını sağlamak ve beklentilere en yakın sonuçları yakalamak. Örneğin github üzerinde geliştirdiğimiz büyük çaplı DDD *(Domain Driven Design)* ile uyumlu bir projeye Senior Software Developer/Business Analyst gibi rolleri tanımlanmış vekil botlar tanımladığımızı düşünelim. Bu botlardan Copilot yardımıyla bir şeyler yapmasını istediğimizde bizim belirlediğimiz kapsam içerisinde hareket etmesini sağlamak için bir RAG düzeneğine ihtiyacımız olacak. Bu düzenek içinse bazı ön hazırlıklara ihtiyacımız var.

En önemli kısımlardan birisi terimler arası ilişkilerin tespiti, matematiksel olarak ifade edilebilmesi ve yakınlıklarının tespiti. Özellikle markdown tabanlı içeriklerin olduğu *spec-oriented* temelli içeriklerde Text Embedding mekanizması ile - [ki şurada bir girizgah yapmıştım](Embedding.md) bu ilişkiler kolayca hesaplanabiliyor. Bu çalışmadaki amacım mimari tasarım, domain dili ve semantik bağlamların bulunduğu JSON tabanlı veri içeriklerini embedding hesaplamalarını da yaptırarak vektörel bir veritabanına almak. Bu RAG tesisatı için ilk adım olarak da düşünülebilir.

## Qdrant Veritabanı

Vektör tabanlı veritabanı olarak **Rust** ile yazılmış [Qdrant](https://qdrant.tech/documentation/overview/)'ı tercih ediyorum *(Github reposu da [şurada](https://github.com/qdrant/qdrant)* Rust ile yazılmış olması yüksek performans ve güvenilir bir bellek kullanımı getiriyor. Ayrıca **REST** ve **gRPC** protokollerini destekleyen API hizmeti ve kullanışlı bir arayüzü var. *(Alternatif olarak **Memcached** veritabanı da tercih edilebilir)* Pek tabii sistemimi kirletmek istemediğimden bir docker imajı kullanacağım. Bu yüzden hali hazırda bu repoya tahsis ettiğim [docker-compose](../docker-compose.yml) dosyasına aşağıdaki servis tanımını ekledim.

```yaml
services:

# Diğer servisler...

  qdrant:
    image: qdrant/qdrant
    ports:
      - "6333:6333" # REST API Portu
      - "6334:6334" # GRPC Portu
    volumes:
      - qdrant_storage:/qdrant/storage
    restart: always
    networks:
      - fnp-network

volumes:
  qdrant_storage:
  # Diğer volume tanımları...
networks:
  fnp-network:
    driver: bridge
```

Hemen kendime bir not düşeyim. FNP docker dosyasyında birçok servis tanımlı ama şu anda sadece qdrant servisini çalıştırmak istiyorum. Bunun için aşağıdaki terminal koutunu kullanabiliriz.

```bash
docker compose up -d qdrant
```

## Text Embedding için Local Dil Modeli

Normal şartlarda büyük dil modelleri ile çalışırken API'ler üzerinden ilerliyoruz. Ücretli modellerde API key'leri kullanarak hareket ediyoruz. Ancak bu deneysel ve öğrenme amaçlı çalışmada ücretsiz dil modellerini yerel bilgisayarda çalıştırarak da ilereleyebiliriz. Senaryoda bize **text embedding** için kullanabileceğimiz bir dil modeli lazım. Ben kişisel bilgisayarımda [LM Studio](https://lmstudio.ai/) arabirimini kullanıyorum. Model olarak da **Nomic Embed Text** dil modelini tercih ettim. 85 Megabyte'tan küçük bir dil modeli. Tabii bunu kullanabilmek için LM Studio'da Local Server'ı başlatmak ve Nomic dil modelini yüklemek gerekiyor. Kabaca aşağıdaki gibi bir arabirime ulaşmamız lazım.

![LM Studio IDE](../images/LM-studio.png)

## Örnek Veriler

Normal şartlarda projenin daha önceden yazılmış markdown dokümanları yine bir AI modelinden yararlanarak embedding için gerekli girdiye dönüştürülebilir. Yani, Claude Sonnet'den Opus'tan veya Gemini'dan ya da muadillerinden yararlarank dokümanları taraması ve mimari, domain dili ve diğer semantik bağlamlara göre bölümlere ayırarak embedding için hazır hale getirilmesi istenebilir. Ben bu öğreti çerçevesinde örnek üç json dokümanı ile ilerleyeceğim.

ubiquities-language.json: Projede kullanılan terimlerin tanımlarını ve birbirleriyle ilişkilerini içeren bir json dosyası. Aşağıdaki gibi basit bir içerik düşünebiliriz.

```json

```