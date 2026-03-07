using System.Net.Http.Json;

public class TodoApiService(IHttpClientFactory httpClientFactory, string baseUrl)
{
    private readonly IHttpClientFactory _httpClientFactory = httpClientFactory;
    private readonly string _baseUrl = baseUrl.TrimEnd('/');

    public async Task<string> GetAllTodosAsync()
    {
        var client = _httpClientFactory.CreateClient();
        var response = await client.GetAsync($"{_baseUrl}/api/todos");
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync();
    }

    public async Task<string> GetCompletedTodosAsync()
    {
        var client = _httpClientFactory.CreateClient();
        var response = await client.GetAsync($"{_baseUrl}/api/todos/completed");
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync();
    }

    public async Task<string> GetIncompleteTodosAsync()
    {
        var client = _httpClientFactory.CreateClient();
        var response = await client.GetAsync($"{_baseUrl}/api/todos/incomplete");
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync();
    }

    public async Task<string> GetTodoByIdAsync(string id)
    {
        var client = _httpClientFactory.CreateClient();
        var response = await client.GetAsync($"{_baseUrl}/api/todos/{id}");
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync();
    }

    public async Task<string> CreateTodoAsync(string title, string? difficulty = null, string? deadline = null)
    {
        var payload = new Dictionary<string, object?> { ["title"] = title };
        if (difficulty is not null) payload["difficulty"] = difficulty;
        if (deadline is not null) payload["deadline"] = deadline;

        var client = _httpClientFactory.CreateClient();
        var response = await client.PostAsJsonAsync($"{_baseUrl}/api/todos", payload);
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync();
    }

    public async Task<string> UpdateTodoAsync(string id, string? title = null, string? status = null, string? difficulty = null, string? deadline = null)
    {
        var payload = new Dictionary<string, object?>();
        if (title is not null) payload["title"] = title;
        if (status is not null) payload["status"] = status;
        if (difficulty is not null) payload["difficulty"] = difficulty;
        if (deadline is not null) payload["deadline"] = deadline;

        var client = _httpClientFactory.CreateClient();
        var response = await client.PutAsJsonAsync($"{_baseUrl}/api/todos/{id}", payload);
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync();
    }

    public async Task DeleteTodoAsync(string id)
    {
        var client = _httpClientFactory.CreateClient();
        var response = await client.DeleteAsync($"{_baseUrl}/api/todos/{id}");
        response.EnsureSuccessStatusCode();
    }

    public async Task<string> UpdateOverdueTodosAsync()
    {
        var client = _httpClientFactory.CreateClient();
        var response = await client.PatchAsync($"{_baseUrl}/api/todos/overdue", content: null);
        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsStringAsync();
    }
}