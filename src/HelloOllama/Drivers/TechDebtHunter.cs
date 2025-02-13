using HelloOllama.Mappers;
using Microsoft.Extensions.AI;
using System.Diagnostics;

namespace HelloOllama.Drivers;

static class TechDebtHunter
{
    public static async Task ReviewCode<T>(IChatClient chatClient,IResponseMapper<T> mapper, PromptType promptType)
    {
        var codeFiles = Directory.GetFiles("C:\\samples", "*.cs").ToArray();
        Console.WriteLine($"Looking for {codeFiles.Length} code files");
        foreach (var codeFile in codeFiles)
        {
            var time = Stopwatch.StartNew();
            Console.WriteLine($"Analysing {codeFile}. Time {DateTime.Now.ToLongTimeString()}");
            var prompt = Prompts.GetPrompt(promptType, codeFile);
            var chatCompletion = await chatClient.CompleteAsync(prompt);
            Console.WriteLine(chatCompletion.Message.Text);

            if (chatCompletion.Message.Text != null)
            {
                var details = await mapper.Map(chatCompletion.Message.Text);
                if (details != null)
                {
                    Console.WriteLine("JSON convert completed");
                }
            }

            Console.WriteLine(Environment.NewLine);
            Console.WriteLine($"Total time of analysis {time.Elapsed.TotalSeconds}");
        }
    }
}