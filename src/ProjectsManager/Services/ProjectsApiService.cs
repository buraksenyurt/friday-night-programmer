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

    public async Task<List<Criteria>> GetAllCriteriaSetsAsync()
    {
        var response = await _httpClient.GetAsync("http://localhost:6503/api/criteria");
        if (response.IsSuccessStatusCode)
        {
            var json = await response.Content.ReadAsStringAsync();
            var apiResponse = JsonSerializer.Deserialize<ApiResponse<Criteria>>(json, options: new JsonSerializerOptions
            {
                PropertyNameCaseInsensitive = true
            });

            return apiResponse?.Data ?? [];
        }
        return [];
    }

    public async Task<bool> DeleteCriterionAsync(int setId, string criterionName)
    {
        var deleteRequest = new
        {
            set_id = setId,
            name = criterionName
        };

        var json = JsonSerializer.Serialize(deleteRequest);
        var content = new StringContent(json, Encoding.UTF8, "application/json");

        var request = new HttpRequestMessage
        {
            Method = HttpMethod.Delete,
            RequestUri = new Uri("http://localhost:6503/api/criterion"),
            Content = content
        };

        var response = await _httpClient.SendAsync(request);
        return response.IsSuccessStatusCode;
    }

}
