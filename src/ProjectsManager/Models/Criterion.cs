using System.Text.Json.Serialization;

namespace ProjectsManager.Models;

public class Criterion
{
    [JsonPropertyName("id")]
    public int Id { get; set; }
    [JsonPropertyName("name")]
    public string Name { get; set; }
    [JsonPropertyName("point")]
    public int Point { get; set; }
}
