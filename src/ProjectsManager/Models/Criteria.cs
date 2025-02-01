using System.Text.Json.Serialization;

namespace ProjectsManager.Models;

public class Criteria
{
    [JsonPropertyName("id")]
    public int Id { get; set; }
    [JsonPropertyName("name")]
    public string Name { get; set; }
    [JsonPropertyName("set")]
    public List<Criterion> Set { get; set; } = [];
}
