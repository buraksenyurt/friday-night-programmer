namespace ProjectsManager.Models;

public class ApiResponse<T>
{
    public bool IsSuccess { get; set; }
    public string Message { get; set; }
    public string Error { get; set; }
    public List<T> Data { get; set; }
}
