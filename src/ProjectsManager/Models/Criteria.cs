using System.Text.Json.Serialization;

namespace ProjectsManager.Models;

public class Criteria
{
    [JsonPropertyName("name")]
    public string Name { get; set; }
    [JsonPropertyName("set")]
    public List<Criterion> Set { get; set; } = [];
}
