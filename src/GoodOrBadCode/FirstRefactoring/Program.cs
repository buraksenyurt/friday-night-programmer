/*
    Sonraki seviye : SecondRefactoring isimli proje
*/
namespace FirstRefactoring;

class Program
{
    static void Main()
    {
        var gameManager = new GameManager();

        gameManager.AddPlayer(new Player("Ali", 20));
        gameManager.AddPlayer(new Player("Ayşe", 21));
        gameManager.AddPlayer(new Player("Mehmet", 19));

        foreach (var player in gameManager.GetPlayer())
        {
            Console.WriteLine(player);
        }
    }
}

// SRP
class Player
{
    public string Name { get; }
    public int Level { get; }

    private const int MinLevel = 70;

    public Player(string name, int age)
    {
        if (age < MinLevel)
            throw new ArgumentException($"Player level must be grater than {MinLevel}");

        Name = name;
        Level = age;
    }

    public override string ToString() => $"Player: {Name}, Level: {Level}";
}

class GameManager
{
    private readonly List<Player> players = [];

    public void AddPlayer(Player player)
    {
        players.Add(player);
    }

    public IEnumerable<Player> GetPlayer() => players;
}