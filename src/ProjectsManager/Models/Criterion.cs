using System.Text.Json.Serialization;

namespace ProjectsManager.Models;

public class Criterion
{
    [JsonPropertyName("name")]
    public string Name { get; set; }
    [JsonPropertyName("point")]
    public int Point { get; set; }
}
