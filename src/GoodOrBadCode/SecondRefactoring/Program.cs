/*
    Sonraki seviye : Revolution isimli proje
*/
namespace SecondRefactoring;

class Program
{
    static void Main()
    {
        var gameManager = new GameManager();

        var result = gameManager.AddPlayer("Larry", 81);
        Console.WriteLine(result.Message);
        result = gameManager.AddPlayer("Cindy", 91);
        Console.WriteLine(result.Message);
        result = gameManager.AddPlayer("Rookie", 56);
        Console.WriteLine(result.Message);

        foreach (var player in gameManager.GetPlayers())
        {
            Console.WriteLine(player);
        }
    }
}

// Result Pattern
public class Result<T>
{
    public bool IsSuccess { get; }
    public T? Value { get; }
    public string Message { get; }

    private Result(bool success, T? value, string message)
    {
        IsSuccess = success;
        Value = success ? value : default; // Null check
        Message = message;
    }

    public static Result<T> Success(T value) => new(true, value, "Created succesfully");
    public static Result<T> Failure(string message) => new(false, default, message);
}

class Player
{
    public string Name { get; }
    public int Level { get; }

    private const int MinLevel = 70;

    private Player(string name, int level)
    {
        Name = name;
        Level = level;
    }

    // Create Instance with Factory
    public static Result<Player> Create(string name, int level)
    {
        if (level < MinLevel)
            return Result<Player>.Failure($"Player level must be grater than {MinLevel}");

        return Result<Player>.Success(new Player(name, level));
    }

    public override string ToString() => $"Name: {Name}, Level: {Level}";
}

class GameManager
{
    private readonly List<Player> players = [];

    public Result<Player> AddPlayer(string name, int level)
    {
        var result = Player.Create(name, level);

        if (!result.IsSuccess || result.Value is null)
        {
            return Result<Player>.Failure(result.Message);
        }

        players.Add(result.Value);
        return Result<Player>.Success(result.Value);
    }

    public IEnumerable<Player> GetPlayers() => players;
}