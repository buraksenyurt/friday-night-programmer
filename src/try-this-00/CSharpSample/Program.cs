namespace CSharpSample;

// İdeal yaklaşıma göre bir nesnenin durumunu eğer dışarıdan bir bileşen bağımlılığı üzerinden değiştirmek gerekmiyorsa 
// (yani stok seviyesinin değiştirilme ilkeleri Part sınıfına dışarıdan öğretilecekse ki bu durumda Part sınıfı yerine StockLevelService 
//gibi başka bir sınıf kullanılır) nesnenin durumunu değiştiren metotları o nesnenin kendi içinde tanımlamaktır.
public class Program
{
    static void Main(string[] args)
    {
        Part part = new()
        {
            Id = 1,
            Name = "Widget",
        };

        Console.WriteLine($"Before update: Part ID: {part.Id}, Name: {part.Name}, Stock Level: {part.StockLevel}");
        part.UpdateStockLevel(150);
        Console.WriteLine($"After update: Part ID: {part.Id}, Name: {part.Name}, Stock Level: {part.StockLevel}");
    }
}

public class Part
{
    public uint Id { get; set; }
    public string? Name { get; set; }
    public int StockLevel { get; private set; }
    public void UpdateStockLevel(int newStockLevel)
    {
        StockLevel = newStockLevel;
    }
}

// public class Program
// {
//     static void Main(string[] args)
//     {
//         Part part = new()
//         {
//             Id = 1,
//             Name = "Widget",
//             StockLevel = 100
//         };

//         Console.WriteLine($"Before update: Part ID: {part.Id}, Name: {part.Name}, Stock Level: {part.StockLevel}");
//         UpdateStockLevel(part, 150);
//         Console.WriteLine($"After update: Part ID: {part.Id}, Name: {part.Name}, Stock Level: {part.StockLevel}");
//     }

//     static void UpdateStockLevel(Part part, int newStockLevel)
//     {
//         part.StockLevel = newStockLevel;
//     }
// }

// public class Part
// {
//     public uint Id { get; set; }
//     public string? Name { get; set; }
//     public int StockLevel { get; set; }
// }