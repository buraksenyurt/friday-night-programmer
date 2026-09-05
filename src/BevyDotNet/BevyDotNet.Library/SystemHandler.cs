namespace BevyDotNet.Library;

public sealed class SystemHandler
{
    private readonly Scheduler _scheduler;
    private readonly SystemState _state;
    private readonly SystemEntry _entry;

    internal SystemHandler(Scheduler scheduler, SystemState state, SystemEntry entry)
    {
        _scheduler = scheduler;
        _state = state;
        _entry = entry;
    }

    public SystemHandler After<TSystem>()
    {
        _entry.RunAfter.Add(typeof(TSystem));
        _scheduler.InvalidateOrder(_state);
        return this;
    }

    public SystemHandler Before<TSystem>()
    {
        _entry.RunBefore.Add(typeof(TSystem));
        _scheduler.InvalidateOrder(_state);
        return this;
    }
}
