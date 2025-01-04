# Gitflow Kullanımı Hakkında

Birçok geliştiricinin çalıştığı ürünlere ait kod depolarında dağıtım stratejileri _(Deployment Strategies)_ önemli konuların başında gelir. Burada kullanılan branch stratejilerden birisi de Gitflow akışlarıdır. [Vincent Driessen](http://nvie.com/posts/a-successful-git-branching-model/) tarafından ilk duyurusu yapılan akış CI/CD süreçlerinde sıklıkla tercih edilir. Atlassian'ın [bu konuda](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) detaylı bir anlatımı var. Gitflow modeli geliştirme sürecini beş farklı branch olarak değerlendirir.

- Master(Main) Branch : Üretime alınmış sürümlerin temsil edildiği daldır
- Develop Branch : Aktif olarak geliştirmenin yapıldığı daldır.
- Feature Branches : Yeni özelliklerin geliştirilmesi için açılan dallardır. Feature branch'lerde geliştirilen kodlar çeşitli test onaylarını takiben develop branch'ine birleştirilir ve sonrasında feature'lar kapatılır. Develop branch'e bu şekilde merge edilen feature'lar potansiye olarak üretime yani bir sonraki sürüme çıkacak olan kodları içerir.
- Release : Üretime çıkılacak son sürümden önce düzenlemelerin yapıldığı son daldır. Release branch sürüm çıktıntan sonra kapatılır ve bu ensanda release üzerindeki kod master ve develop branch'lerine birleştirilir _(Merge)_
- Hotfix : Üretim ortamında oluşmuş kritik hataların düzeltilmesi için açılan daldır. Genellikle Master branch'ten oluşturulur ve mümkün olan en kısa sürede kapatılması beklenir. Kapatılma işleminde düzeltilmiş kodun son hali yine master ve develop branch'lerine birleştirilir _(merge)_

Gitflow komut satırından kullanılabilen bir akıştır ama aynı zamanda Git Extensions isimli tool yardımıyla görsel olarak da kullanılabilir. Sistemde gitflow yoksa yüklemek gerekir.

```bash
# git flow'un yüklü olup olmadığını kontrol etmek için
git flow version
```

## Örnek

Git flow'un nasıl kullanıldığını örneklemek için basit bir senaryo göz önüne alalım. Bir oyun geliştirme framework'ü üzerinde çalıştığımız varsayalım. Yeni sürümde grafik kütüphanesinin yeni bir sürümü olacak ve bunu bir feature üzerinde geliştireceğiz. İlgili feature'u üretim ortamına çıkacağız ve sonra bir sorun olduğunu fark edip hotfix açarak devam edeceğiz. Gerçek hayatta çok sık karşılaştığımız bir senaryo olduğunu söyleyebiliriz. Adım adım ilerleyelim.

**Adım 1 :** İlk olarak yeni bir repo oluşturup git flow ortamını hazırlayalım.

```bash
mkdir AzonGameEngine
cd AzonGameEngine
touch README.md

git init
echo "# Azon ECS Game Engine Framework" > README.md
git add README.md
git commit -m "Initial commit"

git flow init
```

**Adım 2:** Yeni bir Featur açılması.

```bash
# vector-math-refacotring isimli yeni bir feature açıyoruz
git flow feature start vector-math-refactoring

# Sembolik bir takım iyileştirmeler yaptığımızı düşünelim
echo "Vector aritmetiği için gerekli fonksiyonar..." >> README.md
git add README.md
git commit -m "Add vector math feature"

# Bu değişikliklerden sonra feature'ı kapatmak istersek aşağıdaki gibi ilerleyebiliriz.
git flow feature finish vector-math-refactoring
```

**Adım 3:** Release Hazırlanması

```bash
# Önce release branch açılır
git flow release start 1.1.0

# Ardından sürümle ilgili bazı notlar eklenir.
echo "Version 1.1.0: Vector math feature has been added" > CHANGELOG.md
git add CHANGELOG.md
git commit -m "Update changelog for 1.1.0"

# Son olarak sürüm kapatılır
git flow release finish 1.1.0
```

**Hotfix Kullanımı:** Diyelim ki son sürümde kritik bir hata tespit edildi.

```bash
# Bir hotfix açalım
git flow hotfix start vector-func-error-fix

# Sembolik olarak hatayı düzeltelim
echo "Fixed the critical error" > Fixed.txt
git add Fixed.txt
git commit -m "Fix critical bug on vector functionality"

# Hotfix'i kapatalım
git flow hotfix finish vector-func-error-fix
```

## Gitflow Komutları

Çok sık kullanılan komutları şöyle özetleyebiliriz.

| Amaç              | Komut                             |
| ----------------- | --------------------------------- |
| Feature başlatma  | `git flow feature start <featureName>`   |
| Feature tamamlama | `git flow feature finish <featureName>`  |
| Release başlatma  | `git flow release start <releaseName>`  |
| Release tamamlama | `git flow release finish <releaseName>` |
| Hotfix başlatma   | `git flow hotfix start <hotfixName>`    |
| Hotfix tamamlama  | `git flow hotfix finish <hotfixName>`   |
