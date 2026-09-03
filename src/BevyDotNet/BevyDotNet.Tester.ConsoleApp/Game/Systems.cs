using BevyDotNet.Library;

namespace BevyDotNet.Tester.ConsoleApp.Game;

public class SetupPositionSystem : ISystem<Position>
{
    public void Apply(IEnumerable<(Entity entity, Position component)> components)
    {
        foreach (var (entity, position) in components)
        {
            position.X = 0;
            position.Y = 0;
            Console.WriteLine($"[Setup] Entity {entity.ID} initialized at (0,0)");
        }
    }
}

public class MovementSystem : ISystem<Position>
{
    public void Apply(IEnumerable<(Entity entity, Position component)> components)
    {
        foreach (var (entity, position) in components)
        {
            position.X += 1.0f;
            position.Y += 1.0f;
            Console.WriteLine($"[Update] Entity {entity.ID} moved to ({position.X}, {position.Y})");
        }
    }
}

public class MovementWithVelocitySystem : ISystem<Position, Velocity>
{
    public void Apply(IEnumerable<(Entity entity, Position component1, Velocity component2)> components)
    {
        Console.WriteLine("\n[Update] MovementWithVelocitySystem is updating entities with Position and Velocity components;");
        foreach (var (entity, position, velocity) in components)
        {
            position.X += velocity.X;
            position.Y += velocity.Y;
            Console.WriteLine($"[Update] Entity {entity.ID} moved to ({position.X}, {position.Y}) with velocity ({velocity.X}, {velocity.Y})");
        }
    }
}
