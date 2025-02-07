namespace FinalBattle;

class Program
{
    static void Main()
    {
        var levelPolicy = new ProCompetitionPolicy();
        var gameManager = new GameManager(levelPolicy);

        gameManager.AddPlayer("Larry", 81);
        gameManager.AddPlayer("Cindy", 91);
        gameManager.AddPlayer("Rookie", 56);

        foreach (var player in gameManager.GetPlayers())
        {
            Console.WriteLine(player);
        }
    }
}

public class Result<T>
{
    public bool IsSuccess { get; }
    public T? Value { get; }
    public string Message { get; }

    private Result(bool success, T? value, string message)
    {
        IsSuccess = success;
        Value = success ? value : default;
        Message = message;
    }

    public static Result<T> Success(T value) => new(true, value, "Created successfully");
    public static Result<T> Failure(string message) => new(false, default, message);
}

record LevelRange(int Min, int Max);

interface ILevelPolicy
{
    LevelRange GetLevelRange();
}

class RookieCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange() => new(0, 50);
}

class NormalCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange() => new(50, 70);
}

class ProCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange() => new(70, 100);
}

class Player(string name, int level)
{
    public string Name { get; } = name;
    public int Level { get; } = level;

    public override string ToString() => $"Name: {Name}, Level: {Level}";
}

class GameManager(ILevelPolicy levelPolicy)
{
    private readonly List<Player> players = [];
    private readonly ILevelPolicy _levelPolicy = levelPolicy;

    public Result<Player> AddPlayer(string name, int level)
    {
        var range = _levelPolicy.GetLevelRange();

        if (level < range.Min || level > range.Max)
            return Result<Player>.Failure($"Player level must be between {range.Min} and {range.Max} for this competition.");

        var player = new Player(name, level);
        players.Add(player);
        return Result<Player>.Success(player);
    }

    public IEnumerable<Player> GetPlayers() => players;
}