using BevyDotNet.Library;

namespace BevyDotNet.Tester.ConsoleApp.Game;

public class SetupPositionSystem : ISystem<Position, Immobile>
{
    public void Apply(IEnumerable<(Entity entity, Position component1, Immobile component2)> components)
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
    public void Apply(IEnumerable<(Entity entity, Position component1)> components)
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
public class MovementWithVelocitySystem : ISystem<Position, Velocity>, IUsesCommands, IUsesEventBus, IUsesLogger
{
    public Commands Commands { get; set; } = null!;
    public EventBus EventBus { get; set; } = null!;
    public Logger Logger { get; set; } = null!;

    public void Apply(IEnumerable<(Entity entity, Position component1, Velocity component2)> components)
    {
        Logger.Debug("[Update] MovementWithVelocitySystem is updating entities with Position and Velocity components;");
        foreach (var (entity, position, velocity) in components)
        {
            position.X += velocity.X;
            position.Y += velocity.Y;
            Logger.Debug($"[Update] Entity {entity.ID} moved to ({position.X}, {position.Y}) with velocity ({velocity.X}, {velocity.Y})");

            if (position.X > 150.0f || position.X < -50.0f)
            {
                Commands.Despawn(entity);
                EventBus.Publish(new EntityDespawnedEvent(entity.ID)); // Despawn ile ilgili event yayınlanır
                Logger.Debug($"[Despawn] Entity {entity.ID} has moved out of bounds and will be despawned.");
            }
        }
    }
}

public class LogWorldStateSystem : ISystem<Position, Immobile>, IUsesLogger
{
    public Logger Logger { get; set; } = null!;
    public void Apply(IEnumerable<(Entity entity, Position component1, Immobile component2)> components)
    {
        Logger.Info("[Log] Current world state:");
        foreach (var (entity, position, immobile) in components)
        {
            Logger.Info($"Entity {entity.ID}: After setup position ({position.X}, {position.Y}), Immobile ({immobile})");
        }
    }
}

/*
EntityDespawnedEvent nesnelerini dinleyen bir sistem. Bu nesneler bildiğiniz üzere olayları temsil ediyor.
*/
public class DespawnedEntityWatcherSystem : ISystem<Position>, IUsesEventBus
{
    public EventBus EventBus { get; set; } = null!;
    public void Apply(IEnumerable<(Entity entity, Position component1)> components)
    {
        foreach (var @event in EventBus.Get<EntityDespawnedEvent>())
        {
            Console.WriteLine($"[Event] Entity {@event.EntityId} has been despawned.");
        }
    }
}