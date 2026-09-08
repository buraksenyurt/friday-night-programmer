using BevyDotNet.Library;
using BevyDotNet.Library.Core;

namespace BevyDotNet.Tester.ConsoleApp.Game;

public class SetupPositionSystem : ISystem<Position, Immobile>
{
    public void Apply(IEnumerable<(Entity entity, Position component1, Immobile component2)> components, EventBus eventBus)
    {
        var random = new Random();
        foreach (var (entity, position, _) in components)
        {
            position.X = random.Next(0, 100);
            position.Y = random.Next(0, 100);
            Console.WriteLine($"[Setup] Entity {entity.ID} initialized at ({position.X}, {position.Y})");
        }
    }
}

public class MovementSystem : ISystem<Position>
{
    public void Apply(IEnumerable<(Entity entity, Position component1)> components, EventBus eventBus)
    {
        foreach (var (entity, position) in components)
        {
            position.X += 1.0f;
            position.Y += 1.0f;
            Console.WriteLine($"[Update] Entity {entity.ID} moved to ({position.X}, {position.Y})");
        }
    }
}

/*
Bu sistem birde Commands kullanıyor. Entity despawn işlemi için. 
O yüzden IUsesCommands arayüzünü implement ediyor ve Commands property'ini alıyor.
 */
public class MovementWithVelocitySystem : ISystem<Position, Velocity>, IUsesCommands
{
    public Commands Commands { get; set; } = null!;
    public void Apply(IEnumerable<(Entity entity, Position component1, Velocity component2)> components, EventBus eventBus)
    {
        Console.WriteLine("\n[Update] MovementWithVelocitySystem is updating entities with Position and Velocity components;");
        foreach (var (entity, position, velocity) in components)
        {
            position.X += velocity.X;
            position.Y += velocity.Y;
            Console.WriteLine($"[Update] Entity {entity.ID} moved to ({position.X}, {position.Y}) with velocity ({velocity.X}, {velocity.Y})");

            if (position.X > 150.0f || position.X < -50.0f)
            {
                Commands.Despawn(entity);
                eventBus.Publish(new EntityDespawnedEvent(entity.ID)); // Despawn ile ilgili event yayınlanır
                Console.WriteLine($"[Despawn] Entity {entity.ID} has moved out of bounds and will be despawned.");
            }
        }
    }
}

public class LogWorldStateSystem : ISystem<Position, Immobile>
{
    public void Apply(IEnumerable<(Entity entity, Position component1, Immobile component2)> components, EventBus eventBus)
    {
        Console.WriteLine("\n[Log] Current world state:");
        foreach (var (entity, position, immobile) in components)
        {
            Console.WriteLine($"Entity {entity.ID}: After setup position ({position.X}, {position.Y}), Immobile ({immobile})");
        }
    }
}

/*
EntityDespawnedEvent nesnelerini dinleyen bir sistem. Bu nesneler bildiğiniz üzere olayları temsil ediyor.
*/
public class DespawnedEntityWatcherSystem : ISystem<Position>
{
    public void Apply(IEnumerable<(Entity entity, Position component1)> components, EventBus eventBus)
    {
        foreach (var @event in eventBus.Get<EntityDespawnedEvent>())
        {
            Console.WriteLine($"[Event] Entity {@event.EntityId} has been despawned.");
        }
    }
}