# Friday Night Programmer

Bu sene boyunca yapacağımız Friday Night Programmer çalışmalarını toplayacağımız genel repodur.

## Önsöz

Yeni yıl planlarım arasında yer alan ve çok uzun zamandır gerçekleştirmek istediğim bir canlı yayın serisi var. Her cuma gecesi kendimi geliştirmek, tecrübelerimi aktarmak, bilgilerimi pekiştirmek ve tartışmak için çalışma masamın başında olmayı planlıyorum. Programın misyonu ve içeriği aşağıdaki gibidir.

![image](https://github.com/user-attachments/assets/ae1e7a98-b7ba-45e2-85ce-5910012b601a)

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

_Yayın sırasında Rust programlama dilinin bu kadar çok reklamını yapmadım elbette 😄_

İlk programımız bu şekilde sonlandı diyebilirim. Yayın sırasında her ne kadar sürçü lisan ettiysem affola diyeyim. Bir sonraki canlı yayında görüşmek ümidiyle.

## Chapter 01 - A New Hope _(10 Ocak 2025 Cuma, 21:30 - 22:30)_

## Chapter 02 - War Games

## Chapter 03 - Ready, Player One

## Chapter 04 - Enemy At The Gates

## Chapter 05 - Back to the Future

## Chapter 06 - Memento

## Chapter 07 - Johnny Mnemonic

## Chapter 08 - Return of the King
