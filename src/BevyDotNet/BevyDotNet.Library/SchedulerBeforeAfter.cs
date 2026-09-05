namespace BevyDotNet.Library;

/*

Before/After fonksiyonelliğini yazmak benim için oldukça zordu. Bir noktaya kadar getirebilsem de topolojik sıralama ve 
uygulama noktasında epey takıldım. Aşağıdaki sıralama algoritmasını tamamen Claude Opus 5 yazdı. 
Anladığım haliyle yorumlarla kodu anlaşılır hale getirmeye çalıştım.

Bevy oyun motorunda sistemlerin çalıştırılma sırasını da belirleyebiliyoruz.
Zira bazı hallerde bir sistemin diğerlerinden önce veya sonra çalıştırılması gerekebiliyor.
Bu desteği sağlamak için oldukça köklü değişiklikler gerekti. 
Github history'sinde commit'leri ne kadar düzgün tutmaya çalışsam da sonradan anlamak zorlaşabilir bu yüzden bu konu başlığını
Reflection maliyet optimizasyon örneğinde olduğu gibi başka bir Partial class altında topladım.

Aşağıdaki kod parçasında Scheduler sınıfına sistemlerin çalıştırılma sırasını belirlemek için gerekli mekanizmalar eklenmiş durumda.

En kritik metot TopoligicalSort metodu ki burada DFS post-order taktiği söz konusu.

Görevi sistemlerin birbirleri ile olan bağımlılıklarını çözerek doğru bir çalıştırma sırası belirlemek. Böylece bir sistemin başka
bir sistemden önce veya sonra çalışma gerektiği durumlarda doğru bir sıralama elde edebiliriz. Eğer döngüsel bir bağımlılık söz konusu ise InvalidOperationException fırlatılmakta..

Tabii burada yine bir işlem maliyetimiz var. Zira her seferinde sistemlerin bağımlılıklarını çözmek için bir topolojik sıralama yapıyoruz.
Belki bunun önüne geçmek için bağımlılıkların listesini oyun başında çözümleyip Source Code Generator ile bir veri yapısı içerisine
de gömebiliriz. Bu sayede runtime maliyetini ortadan kaldırılabilir diye düşünüyorum.
 */
public partial class Scheduler
{
    internal void InvalidateOrder(SystemState state) => _sortedCache[state] = null;

    // Topolojik sıralama algortiması kullanan metot.
    // Burada DFS post-order taktiği uygulanmakta. Bir diğer teknikse Kahn's algoritması olabilir.
    private static List<SystemEntry> TopologicalSort(List<SystemEntry> entries)
    {
        var byType = entries.ToDictionary(e => e.System.GetType(), e => e);
        var dependsOn = entries.ToDictionary(e => e, _ => new HashSet<SystemEntry>());

        // Entries içerisindeki her bir sistemin RunBefore ve RunAfter listelerini kontrol ederek
        // bağımlılıkları keşfedip dependsOn isimli sözlüğe ekliyoruz.
        foreach (var entry in entries)
        {
            foreach (var beforeType in entry.RunBefore)
                if (byType.TryGetValue(beforeType, out var target))
                    dependsOn[target].Add(entry);

            foreach (var afterType in entry.RunAfter)
                if (byType.TryGetValue(afterType, out var target))
                    dependsOn[entry].Add(target);
        }

        var result = new List<SystemEntry>();
        var visited = new HashSet<SystemEntry>();
        var visiting = new HashSet<SystemEntry>();

        // Visit metodu dikkat edileceği üzere sadece bu metot içerisinde tanımlanmış bir local function.
        // Recursive çalışıyor ve DFS post-order mantığı ile bağımlılıkları çözerek doğru bir sıralama elde ediyor.
        void Visit(SystemEntry entry)
        {
            if (visited.Contains(entry)) return;
            if (!visiting.Add(entry))
                throw new InvalidOperationException(
                    $"Invalid system order detected: {entry.System.GetType().Name}");

            foreach (var dependency in dependsOn[entry])
                Visit(dependency);

            visiting.Remove(entry);
            visited.Add(entry);
            result.Add(entry);
        }

        foreach (var entry in entries)
            Visit(entry);

        return result;
    }

    private List<SystemEntry> GetOrderedSystems(SystemState state)
    {
        if (_sortedCache[state] is { } cached)
            return cached;

        var sorted = TopologicalSort(_systems[state]);
        _sortedCache[state] = sorted;
        return sorted;
    }

}
