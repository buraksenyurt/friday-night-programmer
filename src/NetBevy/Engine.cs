namespace NetBevy;

[AttributeUsage(AttributeTargets.Class)]
public class ComponentAttribute
    : Attribute
{

}

public interface IComponent
{
}

public interface IEntity
{
    Guid ID { get; set; }
    List<IComponent> Components { get; set; }
    void AddComponent(IComponent component);
}

public class Entity
    : IEntity
{
    public Guid ID { get; set; }
    public List<IComponent> Components { get; set; } = [];

    public void AddComponent(IComponent component)
    {
        Components.Add(component);
    }
}

public interface ISystem
{
    void Apply(List<IComponent>? components, params IEntity[]? entities);
}

public enum SystemState
{
    Startup,
    Update
}
public class SystemPair
{
    public SystemState State { get; set; }
    public IEnumerable<ISystem>? Systems { get; set; }
}

public class Scheduler
{
    public List<SystemPair> Systems { get; } = [];
    public void AddSystems(SystemState state, IEnumerable<ISystem> systems)
    {
        Systems.Add(new SystemPair
        {
            State = state,
            Systems = systems
        });
    }
}

public class World
{
    public Entity CreateEntity() => new()
    {
        ID = Guid.NewGuid()
    };

    public void Run(Scheduler scheduler)
    {
        var setupSystems = scheduler.Systems.Where(system => system.State == SystemState.Startup).ToList();
        foreach (var system in setupSystems)
        {
            foreach (var sys in system.Systems ?? [])
            {
                sys.Apply(null,null);
            }
        }   

        var updateSystems = scheduler.Systems.Where(system => system.State == SystemState.Update).ToList();
        foreach (var system in updateSystems)
        {
            foreach (var sys in system.Systems ?? [])
            {
                sys.Apply(null, null);
            }
        }
    }
}
