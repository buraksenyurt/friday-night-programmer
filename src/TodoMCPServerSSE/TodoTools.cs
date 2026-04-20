using ModelContextProtocol.Server;
using System.ComponentModel;

[McpServerToolType]
public class TodoTools
{
    [McpServerTool, Description("Returns all todo items in the list regardless of their status. Use this to get a full overview of all tasks.")]
    public static async Task<string> GetAllTodos(TodoApiService todoApiService)
    {
        return await todoApiService.GetAllTodosAsync();
    }

    [McpServerTool, Description("Returns only the completed (done) todo items. Use this when the user asks which tasks have been finished or marked as done.")]
    public static async Task<string> GetCompletedTodos(TodoApiService todoApiService)
    {
        return await todoApiService.GetCompletedTodosAsync();
    }

    [McpServerTool, Description("Returns only the incomplete (undone) todo items. Use this when the user asks which tasks are still pending or not yet started.")]
    public static async Task<string> GetIncompleteTodos(TodoApiService todoApiService)
    {
        return await todoApiService.GetIncompleteTodosAsync();
    }

    [McpServerTool, Description("Returns a single todo item by its unique ID. Use this when the user asks about a specific task and you already know its ID.")]
    public static async Task<string> GetTodoById(
        TodoApiService todoApiService,
        [Description("The unique UUID of the todo item to retrieve.")] string id)
    {
        return await todoApiService.GetTodoByIdAsync(id);
    }

    [McpServerTool, Description("Creates a new todo item. Use this when the user wants to add a task to the todo list. Difficulty must be one of: easy, medium, hard. Deadline must be an ISO 8601 UTC datetime string (e.g. '2025-06-20T00:00:00Z').")]
    public static async Task<string> CreateTodo(
        TodoApiService todoApiService,
        [Description("The title or description of the task to create.")] string title,
        [Description("Difficulty level of the task. Accepted values: easy, medium, hard. Defaults to medium if omitted.")] string? difficulty = null,
        [Description("Optional deadline for the task in ISO 8601 UTC format, e.g. '2025-06-20T00:00:00Z'.")] string? deadline = null)
    {
        return await todoApiService.CreateTodoAsync(title, difficulty, deadline);
    }

    [McpServerTool, Description("Updates an existing todo item. Only the fields that are provided will be changed; omitted fields keep their current values. Status must be one of: done, undone, inprogress. Difficulty must be one of: easy, medium, hard. Deadline must be an ISO 8601 UTC datetime string.")]
    public static async Task<string> UpdateTodo(
        TodoApiService todoApiService,
        [Description("The unique UUID of the todo item to update.")] string id,
        [Description("New title for the task. Leave null to keep the current title.")] string? title = null,
        [Description("New status for the task. Accepted values: done, undone, inprogress. Leave null to keep the current status.")] string? status = null,
        [Description("New difficulty level. Accepted values: easy, medium, hard. Leave null to keep the current value.")] string? difficulty = null,
        [Description("New deadline in ISO 8601 UTC format, e.g. '2025-06-20T00:00:00Z'. Leave null to keep the current deadline.")] string? deadline = null)
    {
        return await todoApiService.UpdateTodoAsync(id, title, status, difficulty, deadline);
    }

    [McpServerTool, Description("Permanently deletes a todo item by its ID. Use this when the user explicitly asks to remove or delete a specific task. This action is irreversible.")]
    public static async Task DeleteTodo(
        TodoApiService todoApiService,
        [Description("The unique UUID of the todo item to delete.")] string id)
    {
        await todoApiService.DeleteTodoAsync(id);
    }

    [McpServerTool, Description("Finds all tasks whose deadline has passed and are not yet marked as done, then sets their status to 'undone'. Use this when the user asks to reset or flag overdue tasks. Returns the list of affected todo items.")]
    public static async Task<string> UpdateOverdueTodos(TodoApiService todoApiService)
    {
        return await todoApiService.UpdateOverdueTodosAsync();
    }
}