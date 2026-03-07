# .Net ile Bir MCP Server Yazmak

**MCP *(Model Context Protocol)***, yapay zeka araçları için tool desteği sağlamak amacıyla kullanılan bir protokol olarak düşünülebilir. Anthropic tarafından geliştirilmiş bir standarttır ki detaylarına [buradan](https://github.com/modelcontextprotocol) ulaşabilirsiniz. Bu protokolün geliştirilmesinde amaç yapay zeka araçlarına belli bir standart dahilinde harici araç desteği sunabilmektir. Genel senaryoda bir dil modeline gitmeden önce bu protokol üzerinden hizmet veren **MCP Server**`lara gidilerek sağlanan araçlar kullanılabilir. Araçlar da arka planda çoğunlukla REST API hizmetlerini çağırır. MCP, yapay zeka araçları için standart bir protokol sunduğundan tüm MCP server'lar bir yapay zeka aracı tarafından çağırılabilir.

Çok basit ve tek yönlü bir senaryo ile konuyu anlamaya çalışalım: Sisteminizde binlerce servis olduğunu ve bu servislerle ilgili keşif dokümanlarının **yaml** formatlı dosyalarda tutulduğunu düşünelim. Tabii o kadar çok **yaml** içeriği var ki bunlarla alakalı istatistik toplayan, bilgi veren bir de REST Api hizmetimiz var. Bir **MCP server** ile bu API hizmetindeki fonksiyonellikleri birer araç *(tool)* olarak tanımlamak ve çağırmak mümkündür. Senaryoya dahil olan bir MCP istemcisi *(Örneğin VS Code veya farklı bir kod geliştirme ortamındaki AI asistanı, Github CLI veya Claude CLI gibi komut satırı araçları ya da bizim yazacağımız basit bir chatbot)*, MCP server'ımıza bağlanarak araçlarımızı çağırabilir ve bu çıktıları değerlendirerek cevap vermesi için bir dil modeline gidebilir. Söz gelimi bu ortamlarda aşağıdaki gibi sorular sorabiliriz;

- "Sistemimde kaç tane mongodb veritabanı kullanan servis var?"
- "Sipariş yönetimi süreçlerinde kullanılan servisler neler?"
- "Sipariş yönetimi süreçlerinde kullanılan servislerin YAML dokümanlarını getir."
- "5'ten fazla bağımlılığı olan servisleri getir."
- "Müşteri modülüne hizmet eden servislerin listesini getir."

Burada biraz durup düşündüğümüzde, *yahu bu bilgileri bir SQL tablosunda tutup sorgulayan bir servis ve arabirim de yapabilirdik* ya da *bu kadar zahmete ne gerek var, zaten elimizde bir REST API var, onun üzerinden sorgulama yaparız* diye de düşünebiliriz. Bu sık rastlanan bir kafa karışıklığıdır zira MCP server'lar ile REST Api'ler birbirlerine çok benzerler. MCP server'ların REST API'lerden farkı, yapay zeka araçlarının çağırabileceği araç setlerini endüstriyel bir standart üzerinden sunmasıdır. MCP server'lar, yapay zeka araçlarının ihtiyaç duyduğu bilgileri sağlamak için tasarlanmışlardır. MCP standardı, onların kolayca keşfedilmesi *(discover)* ve çağırılmasını da sağlar. Zaten bilinen veya gördüğüm MCP server'lar standart araçların arkasında hep REST API hizmetleri barındırmakta. Ne var ki bu REST API'ler doğrudan yapay zeka araçları tarafından çağrılamazlar, çünkü yapay zeka araçları standart bir protokol üzerinden tanımlanmış araçlara ihtiyaç duyar.

Tabii bu senaryoda tek yönlü bir anlatım söz konusu. Yani, yapay zeka aracı ile kullandığı MCP server daha çok *ask* modunda çalışıyor. Diğer yandan tam ters yönde aksiyonlar gerçekleştirmek de mümkün. Yani bir MCP server aracılığı ile bir görev işletmek, bir süreci tetiklemek, bir uygulamayı başlatmak gibi aksiyonlar da alınabilir. Örneğin, bir **MCP server** üzerinden bir **CI/CD pipeline**'ını tetiklemek ya da projeye yeni bir fonksiyonellik eklemek veya bir board'da task oluşturmak mümkün olabilir. Tüm bunlar MCP server'ın sunduğu araç setine ve arka planda çalışan REST API hizmetlerine bağlıdır.

Konuyu bir örnek üzerinden irdelediğimizde her şey daha kolay anlaşılacaktır diye düşünüyorum. Örneğimizi .Net 10 ile geliştireceğiz.

## Senaryo

Kobay olarak bir todo listesi ile ilintili API hizmetlerini kullanan bir MCP server yazıp kullanmayı deneyelim. Todo listemizi basit bir JSON veri seti olarak düşünebiliriz. Normalde bu veri setinden sorgulama yapmak veya yeni bir todo eklemek için genellikle uygulama bir API hizmeti kullanır. Amacımız bir MCP sunucusu yazıp nasıl kullanıldığını deneyimlemek olduğundan todo listesinin ele alındığı projenin çapı şu an için önemli değil. Aksiyonlarımızı ise şöyle tanımlayalım:

- Soru sorabilelim:
  - "Todo listemdeki tüm görevleri getir?"
  - "Yapılacaklar listemdeki tamamlanmış görevler neler?"
  - "Yapılacaklar listemdeki tamamlanmamış görevler neler?"
- Yeni bir görev ekleyebilelim:
  - "Yapılacaklar listeme 'çalışma odasını temizle' görevini ekle. Bu görev 20 Haziran 2024 tarihine kadar tamamlanmalı. Küçük bir zorluk derecesine sahip"

## Önce Arka Planı Yazalım

Veriyi bir JSON içeriğinde tutacağız ve en azından CRUD operasyonlarını yapabileceğimiz bir REST API hizmeti yazacağız. Bu kısmı rust ile yazalım. Eğlence olsun :D

DEVAM EDECEK...
