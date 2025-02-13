using System.Text.Json.Serialization;

namespace HelloOllama.Models;

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
