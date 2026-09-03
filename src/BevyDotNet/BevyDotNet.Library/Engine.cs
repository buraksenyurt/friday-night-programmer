namespace BevyDotNet.Library;

public enum SystemState
{
    Startup,
    Update
}

public class Scheduler(World world)
{
    private readonly Dictionary<SystemState, List<object>> _systems = new()
    {
        { SystemState.Startup, new List<object>() },
        { SystemState.Update, new List<object>() }
    };

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
                var queryInstance = Activator.CreateInstance(queryType, world);
                var getEntitiesMethod = queryType.GetMethod("GetEntities");
                var entities = getEntitiesMethod.Invoke(queryInstance, null);

                var applyMethod = systemType.GetMethod("Apply");
                _ = applyMethod.Invoke(system, [entities]);
            }
        }
    }
}
