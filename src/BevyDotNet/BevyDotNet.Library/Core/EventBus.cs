using System.Collections;

namespace BevyDotNet.Library;

/*
Bevy gibi oyun motorlarında sistemler arasında haberleşmeyi sağlamak için Event mekanizması kullanılıyor.
Örneğin sistemlerden birisi bir Entity nesnesini despawn ettiğini bunu doğrudan başka bir sistem bildirmek yerine,
bir olay bilgisi olarak yayınlar(publish). O olayla ilgilenen bir sistem varsa bunu okur ve gerekli işlemleri yapar.
Olay yayınlayan ve dinleyen sistemler arasında sıkı bir bağ yoktur tamamen birbirlerinden habersizlerdir.

EventBus sınıfı çok basit anlamda önceki ve şimdiki sistem olaylarını saklayan Dictionary türlerine sahip.
Burada amaç olayları belli bir süre hayatta tutup sonrasında uçurmak. Yoksa sistem başıoş olay nesnelerinden geçilmeyebilir.
Tabii Bevy gibi oyun motorları bunu cursor'lar ile çözümlüyor ki onu burada implemente etmek benim için pek kolay değil.
 */
public class EventBus
{
    private Dictionary<Type, IList> _previoesEvents = [];
    private Dictionary<Type, IList> _currentEvents = [];
    /*
     Bu generic metot ile T türünden bir olay yayınlanıyor.
     Yayınlanıyor derken aslında olay nesnesi EventBus sınıfının _currentEvents Dictionary'sine ekleniyor.
     */
    public void Publish<T>(T @event)
    {
        if (!_currentEvents.TryGetValue(typeof(T), out var events))
        {
            events = new List<T>();
            _currentEvents[typeof(T)] = events;
        }
        events.Add(@event);
    }

    /*
     Sistemin bir an önce ve şu anda yayınlamış olduğu tüm olayları geriye döndüren bir metot.
     Sistemlerin hangi sırada çalıştığı fark etmeksizin işini yapar.
     */
    public IEnumerable<T> Get<T>()
    {
        if (_previoesEvents.TryGetValue(typeof(T), out var prevEvents))
        {
            foreach (T item in prevEvents)
            {
                yield return item;
            }
        }

        if (_currentEvents.TryGetValue(typeof(T), out var currentEvents))
        {
            foreach (T item in currentEvents)
            {
                yield return item;
            }
        }
    }

    /*
     Bir tick tamamlandığında Scheduler tarafından çağıracağımız metot.
     Tick'in sahip olduğu anlık olaylar önceki olayların yerine geçer ve günce olay listesi boşaltılır.
     Burada amaç başta da belirttiğimiz üzere sistemdeki olaylar iki periyot kadar hayatta tutmak.
     */
    internal void Flush()
    {
        _previoesEvents = _currentEvents;
        _currentEvents = [];
    }
}
