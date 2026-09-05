namespace BevyDotNet.Library;

public interface IEntity
{
    Guid ID { get; set; }
    List<IComponent> Components { get; set; }
    void AddComponent(IComponent component);
}
