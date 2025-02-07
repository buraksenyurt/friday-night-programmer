/*
    Senaryomuz şu;

    Bir oyunumuz var ve yarışmacılar önceki oyunlardan kazandıkları puanlarla geldikleri seviyeye göre diğer yarışmalara katılabiliyorlar.
    Örneğin ayın ilk haftası çaylak seviyedeki oyuncular için yeni bir bölüm açılıyor ya da herhangibir kampanya döneminde Pro seviyedekiler için farklı bir tane.
    Dolayısıyla ilgili senaryoda sadece belli puanın üstündeki veya belli puan aralığındaki oyuncular katılabilir.
    Proof of Concept çalışmasında bu tip bir senaryo için nasıl bir kod yazabileceğimize bakıyoruz.
    
    Sonraki seviye -> FirstRefactoring isimli proje
 */
class Program
{
    static readonly Dictionary<string, int> players = [];

    static void Main()
    {
        AddPlayer("Larry", 81);
        AddPlayer("Cindy", 91);
        AddPlayer("Rookie", 58);

        foreach (var entry in players)
        {
            Console.WriteLine("Player: " + entry.Key + ", Level: " + entry.Value);
        }
    }

    static void AddPlayer(string name, int level)
    {
        if (level < 70)
        {
            Console.WriteLine("Level must be grater than 70!");
            return;
        }

        players.Add(name, level);
    }
}