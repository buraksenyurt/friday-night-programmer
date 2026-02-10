using DmsAIAssistant.Plugins;
using Microsoft.SemanticKernel;
using Microsoft.SemanticKernel.ChatCompletion;
using Microsoft.SemanticKernel.Connectors.OpenAI;

/*
 Bu örnek semantik kernel paketinden yararlanarak çok ilkel bir servis asistanı oluşturur. 
 Kullanıcıdan araç plakasını alır ve araç durumunu sorgular veya randevu oluşturur.
 Local ortamda çalışan LM Studio'ya bağlanacak şekilde yapılandırılmıştır.
*/

//string apiKey = Environment.GetEnvironmentVariable("OPENAI_API_KEY")
//    ?? throw new InvalidOperationException("OPENAI_API_KEY environment variable is not set.");
var localEndpoint = new Uri("http://localhost:1234/v1");

string modelId = "llama-3.2-1b-instruct";
string apiKey = "lm-studio";

var builder = Kernel.CreateBuilder();
// builder.AddOpenAIChatCompletion(modelId, apiKey);
builder.AddOpenAIChatCompletion(modelId: modelId, apiKey: apiKey, endpoint: localEndpoint);
builder.Plugins.AddFromType<VehicleServicePlugin>("ServicePlugin");

var kernel = builder.Build();

var chatCompletionService = kernel.GetRequiredService<IChatCompletionService>();
var history = new ChatHistory();

history.AddSystemMessage(
    "You are a helpful assistant for a vehicle service center." +
    "You can provide information about vehicle status and create service appointments based on license plate numbers." +
    "Use the ServicePlugin to get vehicle status and create appointments when requested by the user."
    );

OpenAIPromptExecutionSettings openAIPromptExecutionSettings = new()
{
    ToolCallBehavior = ToolCallBehavior.AutoInvokeKernelFunctions
};
Console.WriteLine("DMS Service Copilot (Press 'q' for exit) ---");

while (true)
{
    Console.Write("User: ");
    string userInput = Console.ReadLine() ?? string.Empty;
    if (userInput.Equals("q", StringComparison.OrdinalIgnoreCase))
    {
        break;
    }
    history.AddUserMessage(userInput);

    try
    {
        var result = await chatCompletionService.GetChatMessageContentAsync(
            history,
            executionSettings: openAIPromptExecutionSettings,
            kernel: kernel);

        Console.WriteLine($"--> {result.Content}");
        history.AddAssistantMessage(result.Content ?? "...");
    }
    catch (Exception ex)
    {
        Console.ForegroundColor = ConsoleColor.Red;
        Console.WriteLine($"Hata oluştu: {ex.Message}");
        Console.ResetColor();
    }
}