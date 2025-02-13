using System.Text.Json.Serialization;

namespace HelloOllama.Models;

public class CodeAnalysisResult
{
    [JsonPropertyName("code_metrics")]
    public CodeMetrics? CodeMetrics { get; set; }

    [JsonPropertyName("notes")]
    public string? Notes { get; set; }
}

public class CodeMetrics
{
    [JsonPropertyName("cognitive_complexity_score")]
    public int CognitiveComplexityScore { get; set; }
    [JsonPropertyName("cyclomatic_complexity")]
    public int CyclomaticComplexity { get; set; }
    [JsonPropertyName("code_duplication_percentage")]
    public double CodeDuplicationPercentage { get; set; }
    [JsonPropertyName("maintainability_index")]
    public int MaintabilityIndex { get; set; }
    [JsonPropertyName("code_quality_score")]
    public int QualityScore { get; set; }
}
