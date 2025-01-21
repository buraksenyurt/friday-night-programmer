namespace NewLinqFunctions;

/*
    
 */
internal class Program
{
    static void Main()
    {
        var games = GameRepository.Load();

        // CountBy: Count games by category
        var countByCategory = games.CountBy(game => game.Category);
        Console.WriteLine("Count by Category:");
        foreach (var (category, count) in countByCategory)
        {
            Console.WriteLine($"{category}: {count}");
        }

        // AggregateBy: Calculate total rate by category
        var aggregateByCategory = games.AggregateBy(
           game => game.Category,
           total_rate => 0.0,
           (x, y) => x + y.Rate
       );
        Console.WriteLine("\nTotal Rate by Category:");
        foreach (var (category, totalRate) in aggregateByCategory)
        {
            Console.WriteLine($"{category}: {totalRate}");
        }

        // Index: Game names list with index
        foreach (var (index, item) in games.Index())
        {
            Console.WriteLine($"Entry {index}: {item.Name}");
        }

        // MaxBy: Find the game with the highest rate
        var highestRatedGame = games.MaxBy(game => game.Rate);
        Console.WriteLine($"\nHighest Rated Game: {highestRatedGame?.Name} ({highestRatedGame?.Rate})");

        // MinBy: Find the game with the lowest rate
        var lowestRatedGame = games.MinBy(game => game.Rate);
        Console.WriteLine($"\nLowest Rated Game: {lowestRatedGame?.Name} ({lowestRatedGame?.Rate})");

        // Chunk: Divide games into chunks of 5
        var chunks = games.Chunk(5);
        int chunkIndex = 1;
        foreach (var chunk in chunks)
        {
            Console.WriteLine($"Chunk {chunkIndex++}:");
            foreach (var game in chunk)
            {
                Console.WriteLine($"  - {game.Name} ({game.Year})");
            }
        }
    }
}
