namespace BevyDotNet.Library;

public interface ISystem<T> where T : IComponent
{
    void Apply(IEnumerable<(Entity entity, T component)> components);
}
