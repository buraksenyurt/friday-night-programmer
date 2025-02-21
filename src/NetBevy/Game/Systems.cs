namespace NetBevy.Game;

public class SetupSystem
 : ISystem
{
    public void Apply(List<IComponent>? components, params IEntity[]? entities)
    {
        Console.WriteLine("Setup...");
    }
}

public class MovementSystem
    : ISystem
{
    public void Apply(List<IComponent>? components, params IEntity[]? entities)
    {
        Console.WriteLine("Movement...");
    }
}

public class PlayerCollisionCheckSystem
    : ISystem
{
    public void Apply(List<IComponent>? components, params IEntity[]? entities)
    {
        Console.WriteLine("Collision check for player...");
    }
}
