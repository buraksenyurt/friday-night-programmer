namespace BevyDotNet.Library;

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
