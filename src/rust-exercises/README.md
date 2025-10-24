# Rust Kodlama İdmanları

Bu dokümanda rust bilgilerimizi tazelemek için çeşitli kaynaklardan derlediğim örneklere yer verilmektedir.

## Başlangıç Seviyesi

### Örnek 1: Unwrap/Expect Tuzaklarından Kaçınmak

Rust'ın güçlü yönlerinden birisi Option< T > ve Result<T, E> tipleri ile hata yönetimidir. Bazen özellikle development safhasındayken unwrap ve expect kullanarak ilerleyebiliriz zira match veya if let kullanmak kodu uzatabilir. Ancak bu yöntem production kodunda ciddi problemlere yol açabilir. Bir sistemin açılırken kritik bir yapılandırma dosyasını okumaya çalıştığını düşünelim. Dosyanın bulunamamsı veya okuma sırasında bir hata alınması halinde programın paniklemesi yerine kullanıcıya anlamlı bir hata mesajı döndürmek veya izlenebilir, tedbir alınabilir bir makine logu bırakmak daha sağlıklı olacaktır.

```rust

```
