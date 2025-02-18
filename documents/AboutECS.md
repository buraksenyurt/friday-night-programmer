# Oyun Programlamada ECS Kullanımı

**ECS** _(Entity Component System)_ Bevy gibi bazı oyun motorları tarafından sıklıkla kullanılan bir sistem yaklaşımıdır. Burada ECS ile ilgili genel notlar yer almaktadır.

- [İçerik](#oyun-programlamada-ecs-kullanımı)
  - [Tanım](#tanım)
  - [ECS Neden Gereklidir?](#ecs-neden-gereklidir)
  - [Örnek Senaryo](#örnek-senaryo)
  - [Composition Over Inheritance](#composition-over-inheritance)
  - [ECS ile OOP Arasındaki Farklar](#ecs-ile-oop-arasındaki-farklar)
  - [Tarihçe](#tarihçe)
  - [ECS in Kullanıldığı Diğer Alanlar](#ecs-in-kullanıldığı-diğer-alanlar)
  - [Bevy ECS Hakkında](#bevy-ecs-hakkında)
  - [Kaynaklar](#kaynaklar)

## Tanım

ECS, karmaşık oyun mantıklarının daha kolay yönetimi için geliştirilmiş mimari bir yaklaşımdır. Eseneklik, modülerlik ve yeniden kullanılabilirlik gibi özellikleri öne çıkarır, **Composition over Inheritance** ilkesini benimser.

- **Entity:** Benzersiz ID ile tanımlı basit bir konteyner olarak düşünülebilir. Gerekli bileşenleri içerir _(Tower, Player, Enemy, Bullet, Gate)_
- **Component:** Sadece veri içeren ve Entity nesnelerine eklenen nesnelerdir. Bir entity bir bileşen nesnesinden sadece bir tane içerebilir.
- **System:** Belli bileşenlere sahip Entity koleksiyonları üzerinde hareket edebilen, bileşen bazlı Entity kümelerini sorgulayabilen fonksiyonlardır.

## ECS Neden Gereklidir?

Bir oyun geliştirirken aktörler, nesneler, bileşenler ve kaynaklar gibi önemli enstrümanlar kullanılır. Bazı durumlarda oyun dünyası içindeki tüm nesnelerin bir hareketi söz konusu iken buna durağan nesneler dahil değildir. Dolayısıyla belli component'lere sahip olan nesneler için işletilecek süreçlerde, örneğin sadece hareket etme kabiliyeti olan varlıkların her frame time anında bir kurala göre yer değiştirmesi ya da çarpışma ve hasar alma verileri içeren varlıklardan yok olanların sahadan ve oyun nesne koleksiyonlarından çıkartılması gibi devasa süreçlerde veri ile davranışın ayrıştırılması kod yönetimi, kod okunurluğu ve çalışma zamanı performansını artırabilir. Kalıtım _(Inheritance)_ bazlı klasik kod pratiklerini içeren oyun sistemlerinde bunu sağlamak çok kolay olmayabilir. ECS burada bir çözüm olarak karşımıza çıkar. Yani nesne sayısının artmasına bağlı olarak oyun motorunun yavaşlaması ve kod ile verinin buna bağlı olarak çok karmaşıklaşması ECS ihtiyacını öne çıkaran konulardır.

ECS'in kazandırdığı bazı avantajlar şöyle sıralanabilir.

- Kod ve veri ayrıldığından veri yeniden yorumlanabilir.
- Kod tek bir Entity yerine birden fazal Entity üzerinde dolaşabilir.
- Sistemler otomatik olarak paralel çalıştırılabilir.
- Sadece belli bileşenleri içeren Entity kümelerinde dolaşmak kolaydır.

_**Unity DOTS** ve **Unreal Mass**'a nazaran Rust için geliştirilmiş olan Bevy'nin kullanımı oldukça kolaydır._

## Örnek Senaryo

**ECS** çatısında oyundaki her nesne benzersiz bir tanımlayıcı ile işaretlenir ve bu bir **Entity** olarak ifade edilir. Entity'lere eklenebilecek verileri içeren datatype nesneleri ise birer **Component** olarak tasarlanır. Sistemler belli bileşenlere sahip Entity setlerinin dolaşılması için kullanılır. ECS, kodun yeniden kullanılabilirliğini _(Reusability)_ artırır ve veriyi davranışlardan _(Behavior)_ ayırır. Popüler oyunlardan birisi olan Tower Defence' ı göz önüne alalım. Entity ve Component ilişkilerini aşağıdaki tabloda görüldüğü gibi özetleyebiliriz.

```text
+----------------+----------+----------+----------+----------+
|   Components   | Player   |  Tower   |  Enemy   | Bullet   |
+----------------+----------+----------+----------+----------+
| Position       | (x,y)    | (x, y)   | (x, y)   | (x, y)   |
| Health         | (hp)     |          | (hp)     |          |
| Damage         | (dmg)    | (dmg)    | (dmg)    |          |
| Range          |          | (range)  |          |          |
| Velocity       |          |          | (vx, vy) | (vx, vy) |
| Inventory      | (inv)    |          |          |          |
+----------------+----------+----------+----------+----------+
```

Position, Health, Damage, Range, Velocity ve Inventory birer Component olarak tasarlanır ve kendi verilerini tutarlar. Player, Tower, Enemy ve Bullet ise oyundaki Entity nesneleri olarak düşünülebilir. Bu Entity nesneleri farklı component'lere sahiptir. Örneğin tüm varlıkların 2D sahada bir konumu vardır ancak Health bileşeni sadece Player ve Enemy için geçerli iken Tower ve Bullet için kullanılmamaktadır. Kule, oyuncu ve düşman hasar alabilir. Hareket edebilen nesneler sadece Enemy ve Bullet' tır. Buna göre sadece hareket edebilen nesneler için bir sistem fonksiyonu yazılabilir.

## Composition Over Inheritance

Entity Component System, kalıtım _(inheritance)_ yerine Composition over Inheritance yaklaşımını kullanır. Bir Entity tür hiyerarşisi yerine onunla ilişkili bileşenleri _(Component)_ tarafından tanımlanır. Sistemler, istenen bileşenlere sahip Entity koleksiyonları üzerinde harket ederek çeşitli işlemler icra edebilir. Bu konuyu daha iyi anlamak adına aşağıdaki iki farklı Rust kodunu ele alabiliriz.

Her ne kadar Rust dilinde inheritance kullanarak hareket etmek kulağa tuhaf geliyor olsa da pekala bu mümkün. Örnek senaryoda savaşçıları, okçuları ve şifacıları ele alıyoruz. Her birisinin oyundaki bir karakter olarak düşünürsek ortak özelliklerini saklayana Character isimli bir yapı kullanmak oldukça mantıklıdır. Buradan hareketle her birinin hasar alma durumunu olabileceğinden bu işi üstlenen ortak bir fonksiyon da tanımlanabilir. Pek tabii Archer ve Warrior'un savaşma biçimleri farklıdır. Biri fire diğeri attack isimli fonksiyonlarıyla ayrışır.

```rust
struct Character {
    id: String,
    health_value: f32,
}

impl Character {
    fn new(id: &str, health_value: f32) -> Self {
        Self {
            id: id.to_string(),
            health_value,
        }
    }
    fn take_damage(&mut self, amount: f32) {
        self.health_value -= amount;
    }
}

struct Archer {
    character: Character,
    range: f32,
}

impl Archer {
    fn new(id: &str, health_value: f32, range: f32) -> Self {
        Self {
            character: Character::new(id, health_value),
            range,
        }
    }
    fn fire(&self) {
        println!(
            "Archer {} fires at well with range {} unit",
            self.character.id, self.range
        );
    }
}

struct Warrior {
    character: Character,
    strength: f32,
}

impl Warrior {
    fn new(id: &str, health_value: f32, strength: f32) -> Self {
        Self {
            character: Character::new(id, health_value),
            strength,
        }
    }
    fn attack(&self) {
        println!(
            "Warrior {} attacks with strength {} power",
            self.character.id, self.strength
        );
    }
}

struct Healer {
    character: Character,
    mana_power: f32,
}

impl Healer {
    fn new(name: &str, health_value: f32, mana_power: f32) -> Self {
        Self {
            character: Character::new(name, health_value),
            mana_power,
        }
    }

    fn heal(&self) {
        println!(
            "{} heals with power {} mana",
            self.character.id, self.mana_power
        );
    }
}

fn main() {
    let white_hand = Archer::new("wh-666", 100.0, 900.50);
    let mut boramir = Warrior::new("boramir-13", 100.0, 8.75);
    let gandalf = Healer::new("gandalf-13", 100.0, 1.25);

    white_hand.fire();
    boramir.attack();
    boramir.character.take_damage(10.5);
    gandalf.heal();
    println!("Boramir health {}", boramir.character.health_value);
}
```

Aslında kullanım şekline baktığımızda pekala mantıklı bir yaklaşım uyguladığımız söylenebilir. Şimdi sisteme farklı kabiliyetleri olan yeni bir karakter eklemek istediğimiz düşünelim. Hatta farklı davranışları olduğundan farklı metotlar da içerecek. Hatta şifacının kabiliyetlerine farklı bir tane daha eklemek isteyelim. Bir süre sonra çok fazla tekrarlı fonksiyona rastlayabilir ve kalıtım mertebesindeki organizasyonu yönetmekte zorlanabiliriz. Şimdi benzer kodu birde ECS sistemine yakın bir biçimde yazalım.

```rust
// Components

struct Position(f32, f32);
struct Velocity(f32, f32);
struct Health(i32);
struct Strength(i32);
struct Mana(i32);
struct Damage(i32);
struct Range(f32);

// Entities
struct Warrior {
    id: String,
    position: Position,
    health: Health,
    strength: Strength,
}

struct Healer {
    id: String,
    position: Position,
    health: Health,
    mana: Mana,
}

struct Archer {
    id: String,
    position: Position,
    health: Health,
    damage: Damage,
    range: Range,
}

struct Villager {
    id: String,
    position: Position,
    health: Health,
}

struct Tower {
    position: Position,
    damage: Damage,
    range: Range,
}

// Systems
fn attack(id: &str, strength: &Strength) {
    println!("{} attacks with strength {}", id, strength.0);
}
fn take_damage(health: &mut Health, amount: i32) {
    health.0 -= amount;
}

fn shoot_arrow(name: &str, damage: &Damage, range: &Range) {
    println!(
        "{} shoots an arrow with damage {} at range {}",
        name, damage.0, range.0
    );
}

fn heal(id: &str, mana: &Mana, target: &mut Health) {
    target.0 += mana.0;
    println!("{} heals with power {}", id, mana.0);
}

fn build(position: &Position, damage: &Damage, range: &Range) {
    println!(
        "Located on {}:{} with {} damage and range {}",
        position.0, position.1, damage.0, range.0
    );
}

fn main() {
    let mut warrior = Warrior {
        id: "Red Skull".to_string(),
        position: Position(50.0, 10.0),
        health: Health(100),
        strength: Strength(25),
    };

    let mut healer = Healer {
        id: "Athena".to_string(),
        position: Position(1.0, 1.0),
        health: Health(80),
        mana: Mana(30),
    };

    let archer = Archer {
        id: "Legolas".to_string(),
        position: Position(2.0, 2.0),
        health: Health(70),
        damage: Damage(40),
        range: Range(100.0),
    };

    let tower = Tower {
        position: Position(5.0, 5.0),
        damage: Damage(60),
        range: Range(50.0),
    };

    attack(&warrior.id, &warrior.strength);
    heal(&healer.id, &healer.mana, &mut warrior.health);
    shoot_arrow(&archer.id, &archer.damage, &archer.range);
    take_damage(&mut warrior.health, 45);
    build(&tower.position, &tower.damage, &tower.range);

    println!("{} has {} health left.", warrior.id, warrior.health.0);
}
```

Bu sefer ilk olarak component'leri tanımladık. İhtiyacımız olan karakterler bu component'leri birer özellik gibi alan komposit yapılar olarak tasarlandı. System fonksiyonlarına dikkat edecek olursak belli Component'leri referans olarak alıp kullanmakta olduklarını fark ederiz. Yani bir sistem fonksiyonunu sadece ilgili component'lere sahip olan aktörler üzerinde kullanırız. 

Tabii buradaki örnek çok basit bir temsil şekli. ECS tabanlı oyun motorlarında component setleri üzerinde hareket edebilmemizi sağlayan dinamik fonksiyonlar bulunuyor. Hatta bir oyun motoru açısından düşündüğümüzde bir Entity nesnesinin örneklenip, ona programcı tarafından yazılmış component'lerin eklenebilmesi demek esasında oyun motorunun tariflediği sözleşmelere uygun bir bileşenin programcı tarafından yazılmasını gerektiriyor. Bunu oyun motoru tarafından tariflenen bir Interface türünün uygulanması gibi düşünebilir ya da Component isimli bir macro attribute ile bir veri yapısına uygulandığı anda gerekli kodun üretilmesi olarak  yorumlayabiliriz.

## ECS ile OOP Arasındaki Farklar

- OOP tarafından kalıtım _(Inheritance)_ birinci sınıf vatandaş _(First-Citizen)_ iken ECS çatısında bu Composition'dır.
- OOP veriyi encapsulate etmeyi önerir, ECS ise Plain Old Data nesnelerini kullanmaya teşvik eder.
- ECS veriyi davranışlardan _(behavior)_ ayırırken, OOP verileri davranışla birleştiren bir yol önerir.

Bu arada ille bunlardan birisini kullanacağız diye bir kural yoktur. Fyrox Engine gibi yeni nesil girişimler hibrit model de kullanırlar. Yani gerektiği yerde kalıtıma gerektiği yerde de composition over inheritance'a geçerler.

## Tarihçe

Aslında ECS mevzusu yeni bir konu değildir. Bu konudaki araştırmalarım şöyle;

- Kayıtlara göre ECS'in ilk öncüsü 1998 yılında yayınlanan **Thief: The Dark Project** isimli oyundur. Bu oyunda kullanılan ECS motoru sonrasında devam oyununda ve **System Shock 2** oyununda kullanılmıştır.
- 2007 yılında ECS sistemlerinin **MMOG-Massively Multiplayer Online Game** türünde kullanımı ile ilgili **Adam Martin** tarafından [detaylı bir yazı](https://t-machine.org/index.php/2007/09/03/entity-systems-are-the-future-of-mmog-development-part-1/) yayınlandı.
- 2015 yılında **Apple**, ECS'in bir uyarlamasını içeren ve iOS, macOS ve tvOS'larda oyun geliştirmek için kullanılan **GameplayKit** isimli bir framework yayınladı.
- 2018 yılında **Sander Mertens** [flecs](https://github.com/SanderMertens/flecs) isimli bir ECS Framework'ü oluşturdu. Bu framework C ve C++ için yapılmış bir uyarlamaydı.
- 2018 yılında Unity platformu da ECS'i kullanan bir demo yayınladı.

## ECS in Kullanıldığı Diğer Alanlar

Şu ana kadar ECS'i oyun motorları ile ele alsak da aslında birçok alanda da kullanılabilir. İşte diğer kullanım alanları;

- **Simülasyon Yazılımları :** ECS, karmaşık sistemlerin modellenmesi gereken simülasyon yazılımlarında kullanılabilir. Örneğin, trafik simülasyonlarını ele alalım. Arabalar ve yayalar birer Entity olarak düşünülebilir. Araçların konumları, hızları ve yönleri birer bileşen _(Component)_ olarak tasarlanabilir. Sistemler, çarpışma algılama ve rota planlama gibi işlevleri yürütebilir.
- **Robotik/IoT :** Robitik veya IoT sistemlerde bir cihazın parçalarını ve etkileşimlerini yönetmek için ECS'den yararlanılabilir. Örneğin bir robotun farklı uzuvları birer Entity olarak düşünülebilir. Kolları, sensörleri, ayakları vs. Yine bu nesnelerin konumları, anlık durumları birer bileşen olaran düşünülebilir. Sistemler bu parçaların koordinasyon ve kontrolünü yönetir ve gezinme, rota belirleme, metrik ölçümleyip durum tespiti yapma, çevre tarama ve basit görevleri etkinleştirir.
- **Data-Driven Mimariler :** Büyük verilerin _(Big Data)_ işlenmesi ve analizinde kullanılabilir. Veri akışları _(Data Streams)_ birer Entity olabilir, metadata ve transformation kuralları ise birer bileşen olarak düşünülebilir. Sistemler verileri bu kurallara göre işler ve analiz eder.
- **Sanal/Artırılmış Gerçeklik (VR/AR) :** Sanal ortamdaki nesneler birer Entity olarak temsil edebilir. Bu nesnelerin fiziksel özellikleri ve davranışları ise birer bileşen olarak düşünülebilir. Sistemler rendering, etkileşim ve gerçek hayat fizik ilkelerini işleyebilir.
- **UI Frameworks :** Bu tip bir framework içerisinde Button, Slider, CheckBox, TextBox gibi unsular birer Entity olarak düşünüldüğünde boyutları, renkleri, durumları vb unsurlar da bileşen olarak tesis edilebilir. Sistemler çeşitli bileşenlere sahip entity nesnelerinin render edilmesi veya kullanıcı ile etkileşimini yönetebilir.

## Bevy ECS Hakkında

Rust'ın en popüler oyun motorlarından birisi haline gelen **Bevy**, ECS çatısının uygulanabildiği en ergonomik çatılardan birisidir. Bileşenler _(Components)_ struct olarak tanımlanırken, sistemler birer fonksiyon olarak yazılır. Bevy ECS, oyun dünyası _(World)_ , planlayıcı _(Scheduler)_ , komut listesi _(Command List)_, kaynaklar _(Resources)_ , sistem setleri _(System Sets)_ ve bundle gibi enstrümanları da sağlayarak programcının işini epeyce kolaylaştırır. Bevy'deki genel kavramlar şu şekilde özetlenebilir;

- **World:** ECS içinde kullanılanacak tüm veri ve kaynakları içeren nesnedir. Entity'ler ve bileşenlerini, kaynakları ve sistemler arası mesajlaşmalar için de kullanılabilecek Event'leri içerir.
- **Resources:** World içerisindeki global değişkenler olarak düşünülebilir. _(Elapsed Times: örneğin belli aralıklarda sahaya bir göktaşının inmesi, Assets: her türlü ses ve grafik, Renderers)_
- **Schedule:** Sistemlerin belli sırada çalıştırılmasını sağlamak için kullanılan bir enstrümandır.
- **Commands:** World nesnesi içerisinde yapısal değişiklikler için kullanılır. Örneğin Entity'lerin spawn/despawn edilmeleri, Entity nesnelerine Component'lerin eklenmesi, Resource nesnelerinin yönetimi gibi
- **System Sets:** Bazı özelliklerin birden fazla sisteme kolayca uygulanabilmesi için kullanılan enstrümandır.

Esas itibariyle ECS tabanlı kendi oyun motorumuzu geliştirmek istediğimiz bir durumda sadece oyun döngüsü ve çevresine değil, oyun programcısının oyunla ilgili bileşenlerini yönetecek runtime'ın hangi operasyonları sağlayacağına da odaklanmak gerekiyor. Yukarıdaki enstrümanlar genel bir konsept hakkında fikir verebilir.

Konuyu daha iyi pekiştirmek adına dilerseniz Bevy'nin ECS'i nasıl kullandığına kısaca bakalım. İşte örnek kod parçası.

```rust
use bevy::prelude::*;

#[derive(Debug, Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Resource)]
struct Timer(f32);

fn main() {
    let mut world = World::new();

    let mut aragon = world.spawn_empty();
    aragon.insert((
        Position { x: 10.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.0 },
        Player,
    ));

    let mut legolas = world.spawn_empty();
    legolas.insert((
        Position { x: 16.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.0 },
        Player,
    ));

    let mut orc_warrior = world.spawn_empty();
    orc_warrior.insert((Position { x: 50.0, y: 0.0 }, Velocity { x: -1.0, y: 0.0 }));

    let mut tower = world.spawn_empty();
    tower.insert(Position { x: 25.0, y: 25.0 });

    world.insert_resource(Timer(0.2));

    let mut schedule = Schedule::default();
    schedule.add_systems((
        (setup, show_players, show_enemies).chain(),
        move_enemies
            .after(setup)
            .before(show_players)
            .before(show_enemies),
    )); // Yazıldığı sırada çalıştırır

    // schedule.add_systems(setup);
    // schedule.run(&mut world);
    //
    // println!();
    //
    // schedule.add_systems((show_players, show_enemies));
    schedule.run(&mut world);
}

fn setup(query: Query<(Entity, &Position)>) {
    println!("Setup system");
    for (entity, position) in query.iter() {
        println!("{:?}\t{:?}. ", entity, position);
    }
}

// fn move_characters(mut query: Query<(&mut Position, &Velocity)>) {
//     for (mut position, velocity) in query.iter_mut() {
//         position.x += velocity.x;
//         position.y += velocity.y;
//     }
// }

fn show_players(query: Query<&Position, With<Player>>) {
    println!("Show players");
    for position in query.iter() {
        println!("Player on {:?}. ", position);
    }
}

fn show_enemies(query: Query<(&Position, &Velocity), Without<Player>>) {
    println!("Show enemies");
    for (position, _) in query.iter() {
        println!("Enemy go to position {:?}. ", position);
    }
}

fn move_enemies(mut query: Query<(&mut Position, &Velocity), Without<Player>>, timer: Res<Timer>) {
    println!("Moving Enemies");
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x * timer.0;
        position.y += velocity.y * timer.0;
    }
}
```

Bu kod parçasında Bevy'nin ana enstrümanları daha net görülebilir. Component nitelikleri ile imzalanmış olan Position ve Velocity birer bileşendir. Player' da benzer şekilde bir Component olarak ifade edilir. Timer isimli bir resource kullanılmaktadır. main fonksiyonunda dikkat edileceği üzere her şey World nesnesi üzerinden işler. Yeni bir Entity ihtiyacımız mı var, World nesnesinden isteyin _(spawn_entity fonksiyonu)_ Entity'ye yeni component'ler mi eklemek istiyoruz, nesne üzerinden çağırın insert fonksiyonunu eklensinler. Oyun dünyasına yeni bir resource mu eklemek istiyoruz, _(timer gibi)_ insert_resource fonksiyonunu çağırmak yeterli. World içindeki tüm sistem fonksiyonarı artık bu bileşenleri ve kaynakları kullanablir. Sistem fonksiyonlarının tanımı ise kayda değerdir. World nesnesindeki Entity ve Component'leri sorgulayabilmek için Query isimli yetenkli bir nesne kullanılır.

- show_players metodunun Query ifadesi, Position bileşeni içeren Player Entity'lerini ele alır.
- show_enemies fonksiyonu Position ve Velocity bileşeni bulunan ama Player olmayan Entity'lerin sorgulanması sağlar _(Örneğin düşmanlar)_
- move_enemies metodu yine Position ve Velocity bileşeni içeren ama Player olmayan Entity'leri işlerken, hareket vektörünü değiştirmek için World'e eklenmiş ve kendisine parametre olarak gelen bir Resource'u kullanır.
- Yorum satırında duran move_characters metodu Position ve Velocity bileşeni içeren her Entity'yi ele alır.

World nesnesi aynı zamanda dahili bir planlayıcı _(scheduler)_ kullanılır. Planlayıcı nesne _(Schedule örneği)_ oluşturulurken kendisine sistem fonksiyonları bildirilir. Bu fonksiyonların sürekli mi çalışacağı, sadece girişte mi işletileceği veya sıralamaları ayarlanabilir. En nihayetinde planlayıcı tüm bu sistem fonksiyonlarını bir World nesne örneği için başlatır. 

Bu arada fonksiyon adları değişiklik gösterebilir. Bevy'nin belki de en önemli sorunu versiyonlar arası bazen terk edilen veya değiştirilen kavramları barındırmasıdır. Migrate dokümanlarını okumakta yarar vardır. Gerçi bunun yerine benim tavsiyem Bevy'nin gerçekleştirdiği bu işi örneğin .Net tarafında yazmaya çalışmanızdır. Mesela şu yetenekli Query nesnesini nasıl yazabiliriz ya da class, struct veya record gibi bir türün bir Component olabileceğini nasıl belirtiriz, peki ya Scheduler... Siz bunları bi düşünün :)

## Kaynaklar

- Kendi ECS çatımızı yazmak istersek Ian'ın [şu adresteki](https://ianjk.com/ecs-in-rust/) öğretisine bakabiliriz. simple_ecs ve simple_ecs_2'de bu öğretinin pratik uygulaması yer almaktadır.
- [Entity Component System - Wikipedia](https://en.wikipedia.org/wiki/Entity_component_system)
- [Rust Entity Component Systems: ECS Libraries for Rust Game Dev 🧩 | Rodney Lab](https://rodneylab.com/rust-entity-component-systems/)
- [Bevy Engine](https://bevyengine.org/)
- [Build Your First Game in Bevy and Rust - Step by Step Tutorial](https://www.youtube.com/watch?v=E9SzRc9HkOg)
- [ECS with Bevy Game Engine](https://www.youtube.com/watch?v=iH5NkbaXi0o)
- [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/introduction.html)
