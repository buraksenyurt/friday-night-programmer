namespace BevyDotNet.Library;

public class World
{
    private List<Entity> _entities = [];

    public Entity CreateEntity()
    {
        var entity = new Entity { ID = Guid.NewGuid() };
        _entities.Add(entity);
        return entity;
    }

    public void DestroyEntity(Entity entity) => _entities.Remove(entity);

    public IEnumerable<Entity> GetEntities() => _entities;
}
