namespace BevyDotNet.Library;

public interface ISystem<T> where T : IComponent
{
    void Apply(IEnumerable<(Entity entity, T component)> components);
}

public interface ISystem<T1, T2> where T1 : IComponent where T2 : IComponent
{
    void Apply(IEnumerable<(Entity entity, T1 component1, T2 component2)> components);
}