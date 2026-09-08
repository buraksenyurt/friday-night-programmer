namespace BevyDotNet.Library;

/*
    Büyüyen Scheduler sınıfımızın bir başka parçası. Burada EventBus sınıfını özellik olarak ekledik sadece.
    Tek bir property için ayrı bir partial dosyayı fiziken oluşturmak mantıksız gelebilir ama kavramsal
    olarak Events ile alakalı olduğunu görmek açısından faydalı olduğunu düşünüyorum.
 */
public partial class Scheduler
{
    internal readonly EventBus _eventBus = new();
}
