namespace RefactoredFinalBattle.Library;

public record LevelRange(int Min, int Max);

public interface ILevelPolicy
{
    LevelRange GetLevelRange();
}

public class RookieCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange() => new(0, 50);
}

public class NormalCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange() => new(50, 70);
}

public class ProCompetitionPolicy : ILevelPolicy
{
    public LevelRange GetLevelRange() => new(70, 100);
}