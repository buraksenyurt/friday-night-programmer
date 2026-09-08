namespace BevyDotNet.Library;

/*
    Büyüyen Scheduler sınıfımızın bir başka parçası. Burada EventBus sınıfını kullanıyoruz.
    EventBus nesnemiz Scheduler hayatta olduğu sürece yaşayacak bir nesne örneği. Commands sınıfının kullanımından farklı, dikkat edelim.
    EndTick metodu ile EventBus nesnesinin Flush metodunu çağırıyoruz. Bu sayede sistemler arası olaylar bir sonraki tick'e taşınmış oluyor.
 */
public partial class Scheduler
{
    internal readonly EventBus _eventBus = new();
    /*
     Felsefe şöyle; Bir oyun tick'i genelde her update döngüsü tamamlandığında çağırılır.
     Tabii bu metodu Scheduler Run içerisinde kullanmayı unutmamalıyım.
     */
    public void EndTick() => _eventBus.Flush();
}
