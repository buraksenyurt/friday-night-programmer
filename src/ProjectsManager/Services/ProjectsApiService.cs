using System.Text.Json;
using System.Text;
using ProjectsManager.Models;

namespace ProjectsManager.Services;

public class ProjectsApiService(HttpClient httpClient)
{
    private readonly HttpClient _httpClient = httpClient;

    public async Task<bool> CreateCriteriaAsync(Criteria criteria)
    {
        var json = JsonSerializer.Serialize(criteria);
        var content = new StringContent(json, Encoding.UTF8, "application/json");

        var response = await _httpClient.PostAsync("http://localhost:6503/api/criteria/set", content);

        return response.IsSuccessStatusCode;
    }
}
