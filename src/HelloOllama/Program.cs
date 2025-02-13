using HelloOllama.Drivers;
using HelloOllama.Mappers;
using Microsoft.Extensions.AI;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

var builder = Host.CreateApplicationBuilder();

/*
    Ollama servisi ile konuşacak bir client nesnesi söz konusu.
    Bu nesne localhost:11434 portundan çalışan servise gidip onun deepseek-r1 ile çalışmasını istiyor. 
*/
builder.Services.AddChatClient(new OllamaChatClient(new Uri("http://localhost:11434"), "deepseek-r1:7b"));
var app = builder.Build();
var chatClient = app.Services.GetRequiredService<IChatClient>();

// await Basics.Run(chatClient);

// await TechDebtHunter.ReviewCode(chatClient, new CodeQualityAnalysisMapper(), PromptType.Advanced);
await TechDebtHunter.ReviewCode(chatClient, new CodeAnalysisResultMapper(), PromptType.OnlyMetrics);