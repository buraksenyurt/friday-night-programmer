namespace RefactoredFinalBattle.Library;

public class Player(string name, int level)
{
    public string Name { get; } = name;
    public int Level { get; } = level;

    public override string ToString() => $"Name: {Name}, Level: {Level}";
}
