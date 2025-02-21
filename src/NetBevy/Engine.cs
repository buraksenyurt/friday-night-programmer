namespace NetBevy;

//[AttributeUsage(AttributeTargets.Class)]
//public class ComponentAttribute
//    : Attribute
//{

//}

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

public interface ISystem<T> where T : IComponent
{
    void Apply(IEnumerable<(Entity entity, T component)> components);
}

public enum SystemState
{
    Startup,
    Update
}

public class Scheduler
{
    private readonly World _world;

    private Dictionary<SystemState, List<object>> _systems = new()
    {
        { SystemState.Startup, new List<object>() },
        { SystemState.Update, new List<object>() }
    };

    public Scheduler(World world) => _world = world;

    public void AddSystem<T>(SystemState state, ISystem<T> system) where T : IComponent
    {
        _systems[state].Add(system);
    }

    public void Run(SystemState state)
    {
        if (!_systems.TryGetValue(state, out List<object>? value)) return;

        foreach (var system in value)
        {
            var systemType = system
                .GetType()
                .GetInterfaces()
                .FirstOrDefault(i => i.IsGenericType && i.GetGenericTypeDefinition() == typeof(ISystem<>));

            if (systemType != null)
            {
                var componentType = systemType.GetGenericArguments()[0];
                var queryType = typeof(Query<>).MakeGenericType(componentType);
                var queryInstance = Activator.CreateInstance(queryType, _world);
                var getEntitiesMethod = queryType.GetMethod("GetEntities");
                var entities = getEntitiesMethod.Invoke(queryInstance, null);

                var applyMethod = systemType.GetMethod("Apply");
                _ = applyMethod.Invoke(system, [entities]);
            }
        }
    }
}


public class World
{
    private List<Entity> _entities = [];

    public Entity CreateEntity()
    {
        var entity = new Entity { ID = Guid.NewGuid() };
        _entities.Add(entity);
        return entity;
    }

    public IEnumerable<Entity> GetEntities() => _entities;
}

public class Query<T> where T : IComponent
{
    private readonly World _world;

    public Query(World world) => _world = world;

    public IEnumerable<(Entity entity, T component)> GetEntities()
    {
        return _world.GetEntities()
            .SelectMany(e => e.Components
                .OfType<T>()
                .Select(c => (e, c)));
    }
}
