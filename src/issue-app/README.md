# Yazılım Destek Formu Senaryosu

Senaryoda asıl amaç sayfalar arası veri taşıma yöntemlerinin incelenmesi. Kök senaryoda yazılım destek formu oluşturulması, oluşturulan formun bir onay ekranına gönderilmesi ele alınıyor. Veriyi saklamak için ilk etapta in-memory bir array kullanılacak. Ancak sonraki adımlarda gerçek bir repository ortamı da ele alınabilir. Senaryonun Windows 11 işletim sisteminde, Visual Studio Code kullanılarak yazılıyor ve paket yöneticisi olarak yarn kullanılıyor.

## Projenin Oluşturulması

```bash
yarn create nuxt issue-app

cd issue-app
yarn add bootstrap
# SSR-Server Side Rendering tabanlı state yönetimi için pinia paketinin eklenmesi
yarn add pinia

yarn dev --open
```
