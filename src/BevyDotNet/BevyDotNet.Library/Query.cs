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
