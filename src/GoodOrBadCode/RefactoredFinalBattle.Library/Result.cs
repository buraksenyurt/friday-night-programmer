namespace RefactoredFinalBattle.Library;

public class Result<T>
{
    public bool IsSuccess { get; }
    public T? Value { get; }
    public string Message { get; }

    private Result(bool success, T? value, string message)
    {
        IsSuccess = success;
        Value = success ? value : default;
        Message = message;
    }

    public static Result<T> Success(T value) => new(true, value, "Created successfully");
    public static Result<T> Failure(string message) => new(false, default, message);
}