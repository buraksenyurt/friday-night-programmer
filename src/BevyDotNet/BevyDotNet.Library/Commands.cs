namespace BevyDotNet.Library;

/*
Bu sınıf ne yapıyor? 

Klasik ECS sistemlerinde Entity oluşturma ve yok etme gibi işlemlerin bir komut nesnesi aracılığı
ile yapıldığını ve genellikle dünya (World) nesnesinden ayrı tutulduğunu görüyoruz. Bu sınıfta World sınıfında
tanımladığımız Entity oluşturma ve silme işlemlerini Action türevli temsilcileri(delegate) kuyrukladığı
bir liste aracılığı ile yönetiyor.

Burada amaç tüm sistemlerin ortaklaşa kullandığı aynı World görüntüsünde Entity ekleme ve çıkarma 
gibi işlemleri stage sonunda topluca uygulamak. Bu sayede sistemler birbirlerinin işlemlerinden etkilenmeden çalışabilirler.
Zira eski düzenimizde böyle bir risk vardı.

Kısacası bu sınıf ile sistemeler Word'e doğrudan erişim vermek yerine, "bunu şimdi değil sonra yap" gibi bir aracı nesne sağlıyoruz.
 */
public class Commands
{
    // Kuyruklanan(Aslında World kullanan bir temsilciye atanmış operasyonlar) aksiyon listemiz
    private readonly List<Action<World>> _queue = [];

    // Artık Entity oluşturma bir aksiyon ve Spawn araclığı ile önce kuyruğa ekleniyor.
    public void Spawn(Action<Entity> action)
    {
        _queue.Add(world =>
        {
            var entity = world.CreateEntity();
            action(entity);
        });
    }

    // Artık Entity yok etme bir aksiyon ve Despawn aracılığı ile önce kuyruğa ekleniyor.
    public void Despawn(Entity entity)
    {
        _queue.Add(world => world.DestroyEntity(entity));
    }

    // İşte burası kuyruğa eklenen tüm aksiyonların sırayla çalıştırıldığı yer
    // Bu sayede sistemler birbirlerinin işlemlerinden etkilenmeden çalışabilirler.
    internal void Flush(World world)
    {
        foreach (var action in _queue)
        {
            action(world);
        }
        _queue.Clear();
    }
}
