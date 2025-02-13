using Microsoft.Extensions.AI;
using System.Diagnostics;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Text.RegularExpressions;

namespace HelloOllama;

static class TechDebtHunter
{
    public static async Task CheckCodeQuality(IChatClient chatClient)
    {
        var codeFiles = Directory.GetFiles("C:\\samples", "*.cs").ToArray();
        Console.WriteLine($"Looking for {codeFiles.Length} code files");
        foreach (var codeFile in codeFiles)
        {
            var time = Stopwatch.StartNew();
            Console.WriteLine($"Analysing {codeFile}. Time {DateTime.Now.ToLongTimeString()}");
            var prompt = Prompts.GetLevel200(codeFile);
            var chatCompletion = await chatClient.CompleteAsync(prompt);
            Console.WriteLine(chatCompletion.Message.Text);

            if (chatCompletion.Message.Text != null)
            {
                var details = await GetJson(chatCompletion.Message.Text);
                if (details != null)
                {
                    Console.WriteLine(details.FunctionalitySummary);
                }
            }

            Console.WriteLine(Environment.NewLine);
            Console.WriteLine($"Total time of analysis {time.Elapsed.TotalSeconds}");
        }
    }

    public async static Task<CodeQualityAnalysis?> GetJson(string markdownResponse)
    {
        try
        {
            var match = Regex.Match(markdownResponse, @"```json\s*(\{[\s\S]*?\})\s*```");

            if (match.Success)
            {
                string json = match.Groups[1].Value;
                using var stream = new MemoryStream(Encoding.UTF8.GetBytes(json));

                return await JsonSerializer.DeserializeAsync<CodeQualityAnalysis>(stream);
            }
        }
        catch (JsonException ex)
        {
            Console.WriteLine($"JSON Parsing Error: {ex.Message}");
        }

        return null;
    }

}

public class CodeQualityAnalysis
{
    [JsonPropertyName("functionality_summary")]
    public string? FunctionalitySummary { get; set; }

    [JsonPropertyName("key_components")]
    public List<KeyComponent>? KeyComponents { get; set; }

    [JsonPropertyName("potential_issues")]
    public List<string>? PotentialIssues { get; set; }

    [JsonPropertyName("code_quality_score")]
    public int CodeQualityScore { get; set; }

    [JsonPropertyName("recommendations")]
    public List<string>? Recommendations { get; set; }

    [JsonPropertyName("notes")]
    public string? Notes { get; set; }
}

public class KeyComponent
{
    [JsonPropertyName("name")]
    public string? Name { get; set; }

    [JsonPropertyName("type")]
    public string? Type { get; set; }

    [JsonPropertyName("description")]
    public string? Description { get; set; }
}
