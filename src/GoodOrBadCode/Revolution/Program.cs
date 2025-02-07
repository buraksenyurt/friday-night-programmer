/*
    Sonraki seviye : FinalBattle isimli proje
*/
namespace Revolution;

class Program
{
    static void Main()
    {
        var levelStrategy = new ProCompetitionPolicy();

        var gameManager = new GameManager();

        gameManager.AddPlayer("Larry", 81, levelStrategy);
        gameManager.AddPlayer("Cindy", 91, levelStrategy);
        gameManager.AddPlayer("Rookie", 56, levelStrategy);

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

record LevelRange(int Min, int Max);

interface ILevelPolicy
{
    LevelRange GetLevelRange();
}

class RookieCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange()
    {
        return new LevelRange(0, 50);
    }
}

class NormalCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange()
    {
        return new LevelRange(50, 70);
    }
}

class ProCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange()
    {
        return new LevelRange(70, 100);
    }
}

class Player
{
    public string Name { get; }
    public int Level { get; }

    private Player(string name, int level)
    {
        Name = name;
        Level = level;
    }

    // Create Instance with Factory method and strategy
    public static Result<Player> Create(string name, int level, ILevelPolicy levelPolicy)
    {
        var range = levelPolicy.GetLevelRange();
        if (level < range.Min || level > range.Max)
            return Result<Player>.Failure($"Player level must be between {range.Min} and {range.Max} for this competition.");

        return Result<Player>.Success(new Player(name, level));
    }

    public override string ToString() => $"Name: {Name}, Level: {Level}";
}

class GameManager
{
    private readonly List<Player> players = [];

    public Result<Player> AddPlayer(string name, int level, ILevelPolicy levelPolicy)
    {
        var result = Player.Create(name, level, levelPolicy);

        if (!result.IsSuccess || result.Value is null)
        {
            return Result<Player>.Failure(result.Message);
        }

        players.Add(result.Value);
        return Result<Player>.Success(result.Value);
    }

    public IEnumerable<Player> GetPlayers() => players;
}