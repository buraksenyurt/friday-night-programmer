using Microsoft.Extensions.AI;
using System.Diagnostics;
using System.Text;

namespace SqlCoderPoc;
class Program
{
    static async Task Main()
    {
        var client = new OllamaChatClient("http://localhost:11434", "sqlcoder:7b");

        var questions = new List<string> {
            "What are the top 5 most expensive products?",
            "How many products are there in each category?",
            "Which suppliers provide products in the Beverages category" ,
            "What is the average unit price of products by category?",
            "List all products that are out of stock and need to be reordered.",
            "Which category has the highest average unit price?",
            "Find all suppliers from Germany and how many products they supply.",
            "What percentage of products are discontinued for each category?",
            "Which products have a unit price higher than the average unit price across all products?"
        };

        var totalTimeKeeper = Stopwatch.StartNew();
        Console.WriteLine($"Started at {DateTime.Now}. Please be patient!\n");
        var processTimeKeeper = Stopwatch.StartNew();

        foreach (var question in questions)
        {
            var output = new StringBuilder();
            var prompt = NorthwindPrompt.GetPrompt(question);

            processTimeKeeper.Restart();
            await foreach (var update in client.GetStreamingResponseAsync(prompt))
            {
                output.Append(update.Text);
                // Console.Write(update.Text);
            }
            processTimeKeeper.Stop();

            Console.WriteLine($"\nCompleted in {processTimeKeeper.Elapsed.Seconds} seconds.");
            Console.WriteLine(output.ToString());
        }

        Console.WriteLine($"\nTotal elapsed in {totalTimeKeeper.Elapsed.Minutes} minutes.");
        Console.WriteLine("Press any key to exit.");
        Console.ReadLine();
    }
}