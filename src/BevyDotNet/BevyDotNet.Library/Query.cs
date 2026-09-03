namespace BevyDotNet.Library;

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

public class Query<T1, T2> where T1 : IComponent where T2 : IComponent
{
    private readonly World _world;
    public Query(World world) => _world = world;
    public IEnumerable<(Entity entity, T1 component1, T2 component2)> GetEntities()
    {
        return _world.GetEntities()
            .SelectMany(e => e.Components
                .OfType<T1>()
                .SelectMany(c1 => e.Components
                    .OfType<T2>()
                    .Select(c2 => (e, c1, c2))));
    }
}
