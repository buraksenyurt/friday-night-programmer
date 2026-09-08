using BevyDotNet.Library.Core;

namespace BevyDotNet.Library;

public partial class Scheduler(World world)
{
    private readonly Dictionary<SystemState, List<SystemEntry>> _systems = new()
    {
        { SystemState.Startup, [] },
        { SystemState.Update, [] }
    };

    private readonly Dictionary<SystemState, List<SystemEntry>?> _sortedCache = new(){
        { SystemState.Startup, null },
        { SystemState.Update, null  }
    };

    public void Run(SystemState state)
    {
        if (!_systems.ContainsKey(state)) return;

        var commands = new Commands();

        foreach (var entry in GetOrderedSystems(state))
        {
            var system = entry.System;
            var invoker = GetInvoker(system.GetType());

            // Eğer sistem IUsesCommands arayüzünü de implement etmişse
            // mecburen Commands property de uyarlamıştır ve yüklenmiş bir Commands nesnesi varsa 
            // onu enjekte edip kullanıma sunabiliriz.
            if (system is IUsesCommands usesCommands)
            {
                usesCommands.Commands = commands;
            }

            var queryInstance = Activator.CreateInstance(invoker.QueryType, world)!;
            var entities = invoker.GetEntitiesMethod.Invoke(queryInstance, null)!;

            invoker.ApplyMethod.Invoke(system, [entities, _eventBus]);
        }

        commands.Flush(world);
    }
}
