using System.Reflection;

namespace BevyDotNet.Library;

public partial class Scheduler
{
    /*
     * Run metodundaki döngüde her seferinde girdiğimiz bir reflection maliyeti var.
     * Sistem tipini buluyoruz, kullandığı Component tiplerinden yola çıkarak Query sınıfını bulup
     * bir örneğini oluşturuyor, GetEntities ve Apply metodlarını çağırıyoruz. Bunun maliyeti şu anda oldukça yüksek.
     * Her döngü her sistem için reflection yapıyor. Bunu önlemek için bir cache mekanizması kullanılabilir.
     * 
     * SystemInvoker adında bir record oluşturuyoruz. 
     * Sistemin tipini, kullandığı Query tipini, GetEntities ve Apply metodlarını tutuyor.
     * Her tür için eşleşen bir SystemInvoker record nesnesi söz konusu. Bu sadece okunabilir ve değiştirilemez bir cache olarak saklanıyor.
     * 
     * Run metodundaki for döngüsü her seferinde önce GetOrCreateInvoker metodunu çağırıyor. 
     * Bu metod eğer daha önce bu sistem tipi için bir invoker oluşturulmuşsa üretiyor eğer varsa da onu kullanıyor.
     * 
     */
    private sealed record SystemInvoker(Type QueryType, MethodInfo GetEntitiesMethod, MethodInfo ApplyMethod);
    private static readonly Dictionary<Type, SystemInvoker> _systemInvokerCache = [];
    private static SystemInvoker GetInvoker(Type instanceType)
    {
        if (_systemInvokerCache.TryGetValue(instanceType, out var invoker))
        {
            return invoker;
        }
        var systemType = instanceType
            .GetInterfaces()
            .First(i => i.IsGenericType && GeneratedSystemInterfaces.Contains(i.GetGenericTypeDefinition()));

        var componentTypes = systemType.GetGenericArguments();
        var queryType = GeneratedQueryTypes[componentTypes.Length - 1].MakeGenericType(componentTypes);
        var getEntitiesMethod = queryType.GetMethod("GetEntities")!;
        var applyMethod = systemType.GetMethod("Apply")!;
        invoker = new SystemInvoker(queryType, getEntitiesMethod, applyMethod);

        _systemInvokerCache[instanceType] = invoker;
        return invoker;
    }
}
