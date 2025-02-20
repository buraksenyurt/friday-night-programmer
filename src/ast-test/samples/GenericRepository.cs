namespace Fnp.Works;

public class Repository<T>
{
    public List<T> Items { get; set; } = new List<T>();

    public void Add(T item)
    {
        Items.Add(item);
    }

    public T GetFirst()
    {
        return Items.FirstOrDefault();
    }
}
