using HelloOllama.Models;
using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace HelloOllama.Mappers;

public class CodeAnalysisResultMapper
    : IResponseMapper<CodeAnalysisResult>
{
    public async Task<CodeAnalysisResult?> Map(string content)
    {
        try
        {
            var match = Regex.Match(content, @"```json\s*(\{[\s\S]*?\})\s*```");

            if (match.Success)
            {
                string json = match.Groups[1].Value;
                using var stream = new MemoryStream(Encoding.UTF8.GetBytes(json));

                return await JsonSerializer.DeserializeAsync<CodeAnalysisResult>(stream);
            }
        }
        catch (JsonException ex)
        {
            Console.WriteLine($"JSON Parsing Error: {ex.Message}");
        }

        return null;
    }
}
