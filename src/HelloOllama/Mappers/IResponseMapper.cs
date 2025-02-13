namespace HelloOllama.Mappers;

interface IResponseMapper<T>
{
    Task<T?> Map(string content);
}
