namespace BevyDotNet.Library;

public enum SystemState
{
    Startup,
    Update
}

public class Scheduler(World world)
{
    private static readonly Type[] SystemInterfaces =
    [
        typeof(ISystem<>),
        typeof(ISystem<,>)
    ];

    private static readonly Type[] QueryInterfaces =
    [
        typeof(Query<>),
        typeof(Query<,>)
    ];
    private readonly Dictionary<SystemState, List<object>> _systems = new()
    {
        { SystemState.Startup, new List<object>() },
        { SystemState.Update, new List<object>() }
    };

    public void AddSystem<T>(SystemState state, ISystem<T> system) where T : IComponent
    {
        _systems[state].Add(system);
    }

    public void AddSystem<T1, T2>(SystemState state, ISystem<T1, T2> system) where T1 : IComponent where T2 : IComponent
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
                .FirstOrDefault(i => i.IsGenericType && SystemInterfaces.Contains(i.GetGenericTypeDefinition()));

            if (systemType == null) continue;

            var componentTypes = systemType.GetGenericArguments();

            var queryOpenType = QueryInterfaces[componentTypes.Length - 1];
            var queryType = queryOpenType.MakeGenericType(componentTypes);

            var queryInstance = Activator.CreateInstance(queryType, world);
            var getEntitiesMethod = queryType.GetMethod("GetEntities");
            var entities = getEntitiesMethod!.Invoke(queryInstance, null);

            var applyMethod = systemType.GetMethod("Apply");
            _ = applyMethod!.Invoke(system, [entities]);
        }
    }
}
