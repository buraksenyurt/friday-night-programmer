# Friday Night Programmer

Bu sene boyunca yapacağımız Friday Night Programmer çalışmalarını toplayacağımız genel repodur.

- [Friday Night Programmer](#friday-night-programmer)
  - [Önsöz](#önsöz)
  - [Oyun Sahası Hakkında](#oyun-sahası-hakkında)
  - [Yardımcı Dokümanlar](#yardımcı-dokümanlar)
  - [Chapter 00 - Hello World](#chapter-00---hello-world-3-0cak-2025-cuma-2130---2230)
  - [Chapter 01 - A New Hope](#chapter-01---a-new-hope-10-ocak-2025-cuma-2130---2230)
  - [Chapter 02 - War Games](#chapter-02---war-games-17-ocak-2025-cuma-2130---2230)
  - [Chapter 03 - Ready, Player One](#chapter-03---ready-player-one-31-ocak-2025-cuma-2130---2230)
  - [Chapter 04 - The Usual Suspects](#chapter-04---the-usual-suspects-7-şubat-2025-cuma-2130---2230)
  - [Chapter 05 - Dark City](#chapter-05---dark-city)
  - [Chapter 06 - Memento](#chapter-06---memento)
  - [Chapter 07 - Johnny Mnemonic](#chapter-07---johnny-mnemonic)
  - [Chapter 08 - Return of the King](#chapter-08---return-of-the-king)

## Önsöz

Yeni yıl planlarım arasında yer alan ve çok uzun zamandır gerçekleştirmek istediğim bir canlı yayın serisi var. Her cuma gecesi kendimi geliştirmek, tecrübelerimi aktarmak, bilgilerimi pekiştirmek ve tartışmak için çalışma masamın başında olmayı planlıyorum. Programın misyonu ve içeriği aşağıdaki gibidir.

![image](https://github.com/user-attachments/assets/ae1e7a98-b7ba-45e2-85ce-5910012b601a)

## Oyun Sahası Hakkında

Friday Night Programmer uzun soluklu bir çalışma alanı. Tek bir proje değil de birden fazla konuyu ele almak için, çeşitli pratikler ve kod antrenmanları yapmak için kullandığım bir alan. Bu oyun sahasında zaman içerisinde birçok eklenti, yardımcı araç veya fikir gündeme gelecek. Bu eklemeler yeni ihtiyaçları da beraberinde getirecek. Örneğin sisteme eklediğim bağımsız servis sayısı arttıkça bunların kolayca keşfedilmesini sağlamak için _(Service Discovery)_ Consul aracına ihtiyaç olacak. Ya da genel bir ftp ortamı için bir docker imajı gerekecek. Dolayısıyla oyun sahası zamanla genişleyecek. İçeriye alınan enstrümanları göz önüne aldığımızda aşağıdaki gibi bir High Level Diagram göz önüne alabiliriz.

![High Level Diagram of Playground](./images/HldOfPlayground.png)

Genel Ortamlar başlığı altında yer alan enstrümanlar docker-compose üzerinden kullanılır. [docker-compose.yml](docker-compose.yml) dosyasını inceleyebilirsiniz. Docker içeriklerini ayağa kaldırmak için aşağıdaki komutu çalıştırmak yeterlidir. Elbette sistemde docker'ın yüklü olduğunu varsayıyorum.

```shell
docker-compose up -d
```

Bununla birlikte servislerin sayısı arttıkça onları nasıl çağırdığımızı da unutabiliriz. Var olan ve ilerleyen zamanlarda eklenemsi düşünülen servisler için bir Postman koleksiyonundan yararlanılması iyi olacaktır. İlgili dosyaya [buradan](Friday%20Night%20Programmer.postman_collection.json) ulaşabilirsiniz. Bu arada yer yer api adreslerini Environment değişkenleri ile de tutmak yararlı olabilir. Örneğin project_api servisi için docker adresi environment değişkeni olarak eklenmiştir. Bu anlamda [Environment](Docker%20Environment.postman_environment.json) dosyasını Postman ortamına aktarmakta yarar var.

## Yardımcı Dokümanlar

Sezon boyunca yardımcı olabilecek bazı dokümanlar, araçlar, gereçler...

- Bölümlerde ele alınacak ana senaryolar [Use Cases](./documents/UseCases.md) isimli dokümanda yer almaktadır.
- Sezon boyunca birçok dummy uygulamaya ihtiyacımız olacak. Dummy servisler, veritabanları, docker imajları vs Bunlara ait özet bilgileri [Utilities](./documents/Utilities.md) dokümanında bulabilirsiniz.

Diğer yandan bazı işlerimizi kolaylaştıracak rehber niteliğindeki how to dokümanları da aşağıdaki listede toplanabilir.

- [Rust ve Bellek Güvenliği](./documents/CvsRust.md)
- [Enum Veri Türünün Rust Tarafında Etkili Kullanımı](./documents/UsefulEnumInRust.md)
- [Ollama Yardımıyla Deepseek Dil Modelini .Net Platformunda Kullanmak](./documents/OllamaWithDotNet.md)
- [Pub/Sub mesajlaşma için Nats Kullanımı(Rust ile)](./documents/HowToNats.md)
- [Rust ile WASM Kullanımında İkinci Round](./documents/RustAndWasmRoundTwo.md)
- [Rust ile WASM Kullanımı](./documents/RustAndWasm.md)
- Popüler **git branch** stratejilerinden olan git flow hakkında bilgi almak için [GitFlowIntroduction](./documents/GitFlowIntroduction.md) isimli dokümana bakabilirsiniz.

## Chapter 00 - Hello World _(3 0cak 2025 Cuma, 21:30 - 22:30)_

[Yayın Linki](https://www.youtube.com/live/K8ygZKn5zGQ?si=YqwmemtXDX4JOxp_)

Bu ilk yayınımız tanışma ile geçti. Yayın sırasında aşağıdaki başlıklara değindiğimiz oldu.

- [Northwind veritabanı](https://support.content.office.net/en-us/media/559a04f2-11b2-44b8-ae4a-92284d1576bd.png) Hatta Microsoft github reposunda çalışabileceğimiz popüler veritabanları için script'ler de yer alıyor. [Northwind](https://github.com/microsoft/sql-server-samples/blob/master/samples/databases/northwind-pubs/instnwnd.sql), [Contoso](https://github.com/microsoft/sql-server-samples/blob/master/samples/databases/contoso-data-warehouse/load-contoso-data-warehouse-to-sql-data-warehouse.sql), [pubs](https://github.com/microsoft/sql-server-samples/blob/master/samples/databases/northwind-pubs/instpubs.sql)
- İlk paramı kazandığım zamanının ötesinde olan geliştirme aracı [Delphi](https://winworldpc.com/product/delphi/2x)
- Yayın sırasında önerdiğim [Mark J.Price'ın Tools and Skills for .NET 8 kitabı](https://www.amazon.com/Tools-Skills-NET-practices-solutions/dp/183763520X)

Yayının ilerleyen kısımlarında şu konu üzerinde durmaya çalıştık. Managed/Unmanaged ortamlar ve burada Rust'ın konumu. C# ve Java gibi managed environment'ler üzerinde koşan diller esasında line of business diye ifade edebileceğimiz iş çözümlerinin hızlı, az hatayla geliştirilmesinde endüstriyel olarak kendilerini kanıtlamış diller. Bu ortamlar belleği bizim için yönetir ve GC _(Garbage Collector)_ gibi mekanizmalar kullanılmayan referansların bellekten düşürülmesini unutmamızı engeller _(en basit anlamda)_ Java ve C# kodları derlendiğinde bir ara kod çıktısı oluşur ve bir çalışma zamanı _(runtime)_ bu kodları yürütür. Bu çalışma zamanı olası program çökmelerine karşı istisnaları _(exception)_ yönetmemizi de sağlar. Tür güvenliği _(type safety)_ söz konusudur, geniş kütüphane desteği ile birçok işlevi kolayca gerçekleştirmemizi sağlarlar.

![image](https://github.com/user-attachments/assets/0e9a98df-ae54-46e4-82a2-541b23b550e3)

Ancak uzaya gönderdiğimiz bir mekiğin yörünge hesaplama modülü ve bağlı sistemlerinde, ya da az enerji tüketimi ile yüksek hesaplama sürelerine çıkmamız beklenen akıllı bir donanım modülünde, gömülü sistemlerin çoğunda, makinelerin programlamasında pek fazla tercih edilmezler. Burada devreye unmanaged ortamlarda koşan C, C++ gibi diller giler. Ne varki bu dillerde kodlama yapmak epey külfetli olabilir. Memory optimazasyonunu çok ileri seviyede yapabilsekde referansların yönetimi kolay değildir. Kullanılmayan bir referansı bellekte düşürmeyi unutmaya görün veya boşaltılmış bir bellek alanına referans eden başka bir atamayı gözden kaçırmaya görün... Olaylar karışabilir ve aşağıdaki gibi bir çok sorun tarihe altın harflerle kazınabilir.

![image](https://github.com/user-attachments/assets/02245767-aad3-453a-b0ed-c62f2e9b441e)

Görsel DevNot Summit 2023'te yaptığım Rust dili sunumuna aittir. İşte tam bu notkada hızlı geliştirme yapmamızı sağlayan güvenli ortamlarla, yüksek hız ve performansa kavuşabildiğimiz ama geliştirmenin görece zor olduğu ortamlar arasında kalırız. Rust tam orta noktada her iki tarafın ihtiyaçlarını giderebilecek kadar güçlü çözümlerle gelir. Unmanaged ortamlarda karşılaşabileceğimiz ve yukarıdaki görselde ifade edilen birçok hatayı henüz derleme _(compile time) aşamasında engeller. Rust dilinin Sustainability Software Engineering tarafında da önemli bir yeri vardır hatta bu konuyu ilk olarak Amazon [şu makalesinde](https://aws.amazon.com/blogs/opensource/sustainability-with-rust/) ele almıştır. Diğer yandan rust dilinin yazılım güvenliği açısından birçok açığa karşı etkili bir çözüm olduğu ifade edilebilir.

_Yayın sırasında Rust programlama dilinin bu kadar çok reklamını yapmadım elbette._

İlk programımız bu şekilde sonlandı diyebilirim. Yayın sırasında her ne kadar sürçü lisan ettiysem affola diyeyim. Bir sonraki canlı yayında görüşmek ümidiyle.

## Chapter 01 - A New Hope _(10 Ocak 2025 Cuma, 21:30 - 22:30)_

[Yayın Linki](https://youtube.com/live/h5H11RjS298)

Bu programda aşağıdaki konulardan bahsettik.

- 20 Nisan 2025'te Rust Türkiye konferansı gerçekleştirilecek. Uluslararası diğer konferansları da takip etmek için [bu adrese](https://corrode.dev/blog/rust-conferences-2025/#rust-konf-turkiye) bakabilirsiniz.
- Özellikle yurt dışında Rust diliyle ilgili farklı ve heves uyandıran iş ilanları var. [Burada uydu sistemleri için](https://lynk.world/careers/?ashby_jid=9e9385e1-a6e5-4ccb-8cb7-6d3b9af1cd88), [şurada NASA'nın çekirdek uçuş yazılım çatısı için](https://stemgateway.nasa.gov/s/course-offering/a0BSJ000000KS9p2AG/flight-software-in-rust) ilgili ilanlar bulunuyor. Ayrıca [MIT tarafında](https://careers.ll.mit.edu/search/?createNewAlert=false&q=rust) da epeyce Rust geliştiricisi aranmakta. _(Yalnız şunu belirtmek isterim. İlanlar güncelliğini yitirebilir ve sayfalara ilerleyen zamanlarda erişilemeyebilir)_
- [Rust In Space! How Rust is powering next-generation space mission simulators](https://www.howtocodeit.com/articles/rust-in-space) isimli oturum hakkında bilgiler verildi.
- Doom oyununu uzayda bir uyduda çalıştırdılar :) [Bu keyifli sunumu](https://www.youtube.com/watch?v=GPHDbVPlmMk) kaçırmayın derim.
- Bu bölüm tavsiye ettiğim kitap Karel Çapek'ten Rossum'un Uluslararası Robotları.
- Akışın kalan kısmında Git Flow stratejisinden bahsettik. Bu konu ile ilgili [Git Flow Introduction](GitFlowIntroduction.md) dokümanına bakabilirsiniz.
- Son bölümde Çağrı Merkezi vakasını ele almaya başladık. Geliştirmeleri genel olarak call-me-sdk isimli feature üstünde yapacağız.

## Chapter 02 - War Games _(17 Ocak 2025 Cuma, 21:30 - 22:30)_

[Yayın Linki](https://youtube.com/live/Pe0iihvA6QE)

Bu programda aşağıdaki konulardan bahsettik.

- Tavsiye edeceğim kitap Harry Harrison'dan Yer Açın! Yer Açın! Hatta 1973 yılında [Soylent Green](https://www.imdb.com/title/tt0070723/) olarak da sinemaya uyarlanmış.
- Bahsettiğimiz 1984 Eylül tarihli Sinclair Programs dergisi için [şu adrese](https://ia601006.us.archive.org/2/items/sinclair-programs-23/SinclairPrograms23-Sep84.pdf) uğrayabilir ve derginin tamamına ulaşabilirsiniz.
- Dotty The Kangroo isimli oyun üzerine güzel bir video ve Ian McTavish'i bulma macerası için [şu videoya](https://www.youtube.com/watch?v=EbFN5dS_iuU) bakılabilir. İlgili oyunu oynamak isterseniz de [buradaki](https://spectrumcomputing.co.uk/entry/40761/ZX-Spectrum/Dotty_the_Kangaroo) emulatorlerden yararlanabilirsiniz.
- Rust'ı 100 örnek ile öğrenmek isterseniz güzel bir doküman var. [İşte burada](https://rust-exercises.com/100-exercises/)
- Google'ın Android takımı için hazırladığı [Rust eğitim dokümanı](https://google.github.io/comprehensive-rust/bare-metal.html) oldukça doyurucu.
- Google'ın kapattığı projeleri öğrenmek isterseniz bir [mezarlık](https://killedbygoogle.com/) var.
- Web sitelerinde kullanıcıların en sevmediği şey sanırım 404 hatası almak. Ancak bunu da [kullanıcı dostu hale](https://www.creativebloq.com/web-design/best-404-pages-812505) getiriyorlar.
- Readme dokümanlarını hazırlarken yazı stillerini düzenlemekte kullanabileceğimiz kavramların özetine [buradan](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax) ulaşabilirsiniz.
- .Net platformu için kullanılabilecek araçların [derlenmiş bir listesi](https://github.com/quozd/awesome-dotnet?tab=readme-ov-file#tools) Arada sırada güncellense de topluca türlü bilgilere ulaşabileceğimiz bir alan.

## Chapter 03 - Ready, Player One _(31 Ocak 2025 Cuma, 21:30 - 22:30)_

[Yayın Linki](https://www.youtube.com/live/4jxHRzlo77E)

Bu programda aşağıdaki konulardan bahsettik.

- Futbol ekonomisinin büyüklüğünü gözler önüne seren Deloitte analizi ile başladık. Detaylar [burada](https://www.deloitte.com/uk/en/services/financial-advisory/analysis/deloitte-football-money-league.html). Ayrıca La Liga'nın [Microsoft CoPilot](https://unlocked.microsoft.com/laliga-beyond-stats/) ile olan işbirlikteliğine baktık.
- Dünyanın ilk ev içi oyun konsolu olan Magnavox'u ve 1972 yılı yapımı Tennis oyununa baktık. [Youtube](https://www.youtube.com/watch?v=vB5fE9eTVmk) videosu burada. Oyun konsolunun kullanım kılavuzuna da [Internet Arşivinden](https://archive.org/details/Odyssey_Installation_and_Game_Rules_1972_Magnavox_US/mode/2up) bakılabilir. Ayrıca diğer oyun konsolları ile ilgili detaylı bir sitede var. Teknoloji tarihçesini sevenler için tam bir hazine, [Video Game Console Library](https://www.videogameconsolelibrary.com/index.html)
- Bu bölüm değindiğim iki kitap var. Birisi [The Nostalgia Nerd's Retro Tech](https://amzn.eu/d/bn7aPWd) diğeri ise [System Programming with Rust](https://www.amazon.com.tr/dp/B0DP21NGJY)
- Rust'ın Linux çekirdeğindeki kullanımı da artıyor. Bilgilendirme yazısı [şurada](https://www.phoronix.com/news/Linux-6.14-Rust)
- Eğer Rust ile bir işletim sistemi yazmayı düşünüyorsanız hangi yollardan ilerleyebileceğinizi anlatan dolu dolu bir blog var. İlk bölümde bare metal programming'e giriş yapıyor ardından VGA kartının belleğine doğrudan erişerek işletim sisteminin terminal ekranını tasarlıyorsunuz. [Kaçırmayın](https://os.phil-opp.com/)
- Rust ile C# ın belli noktalarda kıyaslandığı bir [yazıya](https://woodruff.dev/exploring-programming-paradigms-c-and-rust-side-by-side/?amp=1) da değindik. Hatta sevgili Salih Cantekin hocanın blog yazısına da uğradık, [In place string reverse in Rust](https://salihcantekin.com/in-place-string-reverse-in-rust/) Bora Kaşmer hocamızı unutur muyuz? [Microsoft Teams’e bir Worker Üzerinden Mesaj Atmak](https://www.borakasmer.com/microsoft-teamse-bir-worker-uzerinden-mesaj-atmak/)
- Eğer 2025 yılında bir yazılım geliştirici olarak kendinize çeşitli challenge'lar arıyorsanız [bu yazı](https://www.upgrad.com/blog/software-engineering-challenges/) size birkaç fikir verebilir.
- Sıklıkla ThoughtWorks firmasının teknoloji radarını takip etmenizi öneriyorum. Bu son yazıda en çok dikkatimi çeken [Replacing pair programming with AI](https://www.thoughtworks.com/radar/techniques/summary/replacing-pair-programming-with-ai) tekniğinin uzun vadede zarar vereceğini ve kullanılmamasının tavsiye edildiğini belirten kısım oldu.
- Repodaki gelişmelere de değindik. Yeni bir Use Case' imiz var. [Use Case](./documents/UseCases.md) dokümanındaki UC01 kodlu vaka. Projemizde src klasöründe yer alıyor. İlk etapta servis tarafının geliştirilmesi söz konusu. projects_api isimli rust uygulamasına bakabilirsiniz. Test için gerekli postman dosyası ise [burada](./Friday%20Night%20Programmer.postman_collection.json). Ayrıca Rust ile WASM kullanımının ele alındığı basit bir senaryodan da bahsettik. Detaylara [bu yazıdan](https://www.buraksenyurt.com/post/sunucu-metriklerini-izleme-rust-ve-wasm-ile) ulaşabilirsiniz.
- Gelen bir soru üzerine ufakta olsa CAP teoremi üzerine konuştuk. Kısaca bir dağıtık sistemde Consistency, Availability ve Partition Tolerance kavramlarının üçü bir arada olamaz üzerine dayalı.

Sonraki yayında yazılım mimarilerine de değineceğiz.

## Chapter 04 - The Usual Suspects _(7 Şubat 2025 Cuma, 21:30 - 22:30)_

[Yayın Linki](https://www.youtube.com/live/1VNXT3lckns)

Bu programa geçen bölümden gelen bir soru üzerine yazılım mimarisi seçiminde yaparken nelere dikkat ederiz ile başlıyoruz. Ben bu konu ile ilgili referans kitap olarak Richards & Ford’ un [Fundamentals of Software Architecture](https://www.oreilly.com/library/view/fundamentals-of-software/9781492043447/) kitabını öneriyorum.

- Mimariler, servisler vs demişken kullandığımız çatıların her zaman kontrol altında tutulması da gerekiyor. Bu haftanın gündemine düşen bir diğer konuda güvenlik açıkları ile ilgili. [CISA tags Microsoft .NET and Apache OFBiz bugs as exploited in attacks](https://www.bleepingcomputer.com/news/security/cisa-tags-microsoft-net-and-apache-ofbiz-bugs-as-exploited-in-attacks/) yazısında belirtildiğine göre eski dostumuz .Net Remoting ile ilgili bir açıkda varmış. Diğeride Apache'nin Open For Business ürününe ait.
- Haftanın oyunu _(Airborne Empire)_ Oyunla alakalı [Steamdb](https://steamdb.info/app/2438680/) üstünden de bilgi alınabilir. Oyun Unity ile geliştirilmiş bir koloni simülasyon oyunu. Diğerlerinden farklı olarak koloniniz havada uçan bir şehir ve sizi sık sık bela olan korsan pilotlar mevcut.
- O'Reilly, kendi öğrenim platformundaki bilgilerden yararlanarak yıllık bir rapor yayınlamış ve 2025 yılı teknoloji trendlerinden bahsetmiş. Programda yazının detayları _(Özellikle programlama dilleri bölümü)_ üzerinde durduk. [Buradan okuyabilirsiniz](https://www.oreilly.com/radar/technology-trends-for-2025/).
- The Angry Dev'in C# geliştiricilerinin neden Rust öğrenmesi gerektiğini vurguladığu güzel bir karşılaştırma yazısına denk geldim; [Why C# Developers Should Also Learn Rust, and What It Can Teach Them](https://www.darrenhorrocks.co.uk/why-csharp-developers-should-learn-rust-and-what-it-can-teach-them/)
- Bir başka güzel yazıda Rust'ın özelliklerinin hangi dillerden esinlenilerek alındığını da içeren şu yazı. [Stop saying Rust is Complicated](https://rust-on-nails.com/blog/rust-complicated/). Bu yazıda Rust'ın sadece bir sistem programla dili olarak değil birçok alanda kullanılabileceği de belirtiliyor. Mesela WASM ile birlikte. Bende [ilkel bir WASM oyunumu paylaşayım](https://github.com/buraksenyurt/rust-farm/tree/main/handson/running_rectangle). Yarıda bırakmış olsam da ilgilenenler daha ileri bir seviyeye taşıyabilirler. 
- Gömülü sistemler ve emniyet-kritik olanlar demişken güzel bir podcast'e de denk geldim. Volvo' da bir Rust eko sisteminin oluşturulmasının hikayesini merak ediyorsanız buyrun. [Volvo with Julius Gustavsson - Rust in Production Podcast](https://corrode.dev/podcast/s03e08-volvo/)
- Gömülü sistemlerde Rust ile geliştirme yapmayı öğrenmek istiyorsanız [A 5-Step Guide For Learning Embedded Rust](https://www.theembeddedrustacean.com/p/embedded-rust-learning-guide) iyi bir giriş noktası olabilir _(Bu arada pek çok noktada HAL diye bir kavram geçiyor. Bu tabii Kubrick' in Space Odyssey filmindeki [HAL 9000](https://en.wikipedia.org/wiki/HAL_9000) değil :D Hardware Abstraction Layer manasında geçen bir kavram)_
- Emniyet-kritik sistemler _(Safety-Critical Systems)_ açık kaynak yazılımların belirli güvenlik standartlarına göre sertifikalandırılmasını ifade eder. Bu tür yazılımlar, hataların ciddi sonuçlar doğurabileceği alanlarda kullanılır, örneğin:
  - Otomotiv [ISO 26262-1:2018 Road vehicles — Functional safety](https://www.iso.org/standard/68383.html)
  - Havacılık [DO-178C - DO-178C - Wikipedia](https://en.wikipedia.org/wiki/DO-178C)
  - Tıp teknolojileri [IEC 62304 - IEC 62304:2006 - Software life cycle processes](https://www.iso.org/standard/38421.html)
  - Endüstriyel otomasyon [IEC 61508 - IEC 61508 - Wikipedia](https://en.wikipedia.org/wiki/IEC_61508)
- ESP kodlu mikro denetleyicilerde Rust ile geliştirme yapmak isterseniz genele açık şu kitabı tavsiye ederim. [Introduction - The Rust on ESP Book ESP](https://docs.esp-rs.org/book/introduction.html) 
- Mikrodenetleyicilerde genellikle farklı bir işletim sistemi çalışır. RTOS _(Real Time Operating System)_ olarak adlandırılır. Temelleri için [RTOS Fundamentals - FreeRTOS™](https://www.freertos.org/Documentation/01-FreeRTOS-quick-start/01-Beginners-guide/01-RTOS-fundamentals) isimli dokümana bakılabilir. Ayrıca iki örnek işletim sistemi şunlar; [Zephyr Project](https://www.zephyrproject.org/) ve [FreeRTOS](https://www.freertos.org/Why-FreeRTOS/What-is-FreeRTOS)
- Bu bölümde yer verdiğim kitap ise Sovyetler döneminin en önemli bilim kurgu yazarlarından olan Arkadi ve Boris Strugatski kardeşlerin 1997 yılında kaleme aldığı [Kıyamete Bir Milyar Yıl](https://amzn.eu/d/6xdEmQC)

Programda ayrıca [Örnek Senaryolar](./documents/UseCases.md) dokümanındaki **UC02 - İzole Edilmiş SMTP Server** ve **UC03 - Servisler Arası İletişim içim gRPC Kullanımı** konularına da değinildi.

## Chapter 05 - Dark City

## Chapter 06 - Memento

## Chapter 07 - Johnny Mnemonic

## Chapter 08 - Return of the King
