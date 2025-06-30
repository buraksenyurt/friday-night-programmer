# Nuxt Hello World

Projeyi oluşturmak için Windows 11 ortamında yarn ile aşağıdaki şekilde ilerledim.

```bash
yarn create nuxt book-form

# Projeyi çalıştırmak için(dev mode)
cd book-form
yarn dev --open

# Harici paket yüklemek için (örneğin bootstrap)
yarn add bootstrap
```

## Development Server

Eğer ayarlar değiştirilmezse uygulama `http://localhost:3000`: adresinden ayağa kalkar.

## Production

Production hazırlıkları için,

```bash
yarn build
```

Ve üretime çıkan sürümü test etmek için

```bash
yarn preview
```

## Notlar

Senaryoda Hugo ödülleri almış bilim kurgu eserlerinin girildiği, listelendiği veya silindiği iki basit Vue formu söz konusu. Uygulamada yapılanları şöyle özetleyebiliriz.

- Bilgileri in-memory olarak bir array'de tutulmakta.
- Sayfa bazlı yönlendirmeler söz konusu. Her operasyon ayrı bir sayfaya yönlendirildi.
- Kod tekrarını azaltmak için components, composable, types gibi klasörlerdeki enstrümanlar oluşturuldu.
- Veri okuma, ekleme ve silme işlemleri api klasöründeki servis fonksiyonları ile karşılandı.
- Servis ve sayfa habeleşmesi services içerisindeki bookService nesnesi üzerinden sağlandı.
- CSS framework olarak bootstrap adapte edildi.

Örnek ekran görüntüleri.

İlk açılış;

![Runtime_00](Images/Runtime_00.png)

Listeleme sayfası (/book/list)

![Runtime_01](Images/Runtime_01.png)

Listeye kitap ekleme sayfası

![Runtime_02](Images/Runtime_02.png)

Listeden kitap çıkarma

![Runtime_03](Images/Runtime_03.png)

![Runtime_04](Images/Runtime_04.png)
