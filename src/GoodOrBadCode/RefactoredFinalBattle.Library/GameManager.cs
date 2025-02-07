namespace RefactoredFinalBattle.Library;

public class GameManager(ILevelPolicy levelPolicy)
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
