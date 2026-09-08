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

        Logger.Debug($"Running systems for state: {state}");

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
            // Commands için kullandığımız aynı taktiği EventBus için de uyguladık.
            if (system is IUsesEventBus usesEventBus)
            {
                usesEventBus.EventBus = _eventBus;
            }
            if (system is IUsesLogger usesLogger)
            {
                usesLogger.Logger = Logger;
            }

            var queryInstance = Activator.CreateInstance(invoker.QueryType, world)!;
            var entities = invoker.GetEntitiesMethod.Invoke(queryInstance, null)!;

            Logger.Trace($"{system.GetType().Name} system is running");
            invoker.ApplyMethod.Invoke(system, [entities]); // Artık Commands ve EventBus enjekte edilerek kullanıldığı için burası daha temiz oldu.
        }

        commands.Flush(world);
        _eventBus.Flush();
        Logger.Debug($"Finished running systems for state: {state}");
    }
}
