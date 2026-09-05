namespace BevyDotNet.Library;

public enum SystemState
{
    Startup,
    Update
}

public partial class Scheduler(World world)
{
    private readonly Dictionary<SystemState, List<object>> _systems = new()
    {
        { SystemState.Startup, new List<object>() },
        { SystemState.Update, new List<object>() }
    };

    public void Run(SystemState state)
    {
        if (!_systems.TryGetValue(state, out List<object>? value)) return;

        var commands = new Commands();

        foreach (var system in value)
        {
            var systemType = system
                .GetType()
                .GetInterfaces()
                .FirstOrDefault(i => i.IsGenericType && GeneratedSystemInterfaces.Contains(i.GetGenericTypeDefinition()));

            if (systemType == null) continue;

            var componentTypes = systemType.GetGenericArguments();
            var queryType = GeneratedQueryTypes[componentTypes.Length - 1].MakeGenericType(componentTypes);

            var queryInstance = Activator.CreateInstance(queryType, world);
            var entities = queryType.GetMethod("GetEntities")!.Invoke(queryInstance, null);

            systemType.GetMethod("Apply")!.Invoke(system, [entities, commands]);
        }

        commands.Flush(world);
    }
}
