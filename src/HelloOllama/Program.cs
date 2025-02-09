using Microsoft.Extensions.AI;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.Diagnostics;

var builder = Host.CreateApplicationBuilder();
/*
    Ollama servisi ile konuşacak bir client nesnesi söz konusu.
    Bu nesne localhost:11434 portundan çalışan servise gidip onun deepseek-r1 ile çalışmasını istiyor. 
*/
builder.Services.AddChatClient(new OllamaChatClient(new Uri("http://localhost:11434"), "deepseek-r1:7b"));
var app = builder.Build();
var chatClient = app.Services.GetRequiredService<IChatClient>();

#region Sample 02 - Code Quality Check

var codeFiles = Directory.GetFiles("C:\\samples", "*.cs").ToArray();
Console.WriteLine($"Looking for {codeFiles.Length} code files");
foreach (var codeFile in codeFiles)
{
    var time = Stopwatch.StartNew();
    Console.WriteLine($"Analysing {codeFile}. Time {DateTime.Now.ToLongTimeString()}");
    string prompt = $$"""
    You are an expert in analyzing C# source code. Your task is to quickly summarize the given code file.

    ## Expected Response:
    - **Purpose**: A one-sentence description of what this code does.
    - **Main Components**: A list of important classes and methods with a short explanation.
    - **Potential Issues**: Mention one or two possible problems, if any.
    
    ## Response Format:
    ```json
    {
        "purpose": "Short description of the code's functionality.",
        "main_components": [
            {
                "name": "ClassName",
                "type": "class",
                "description": "Short explanation."
            },
            {
                "name": "MethodName",
                "type": "method",
                "description": "Short explanation."
            }
        ],
        "potential_issues": [
            "Brief mention of possible issues (if any)."
        ]
    }
    ```

    ## C# Code:
    {{File.ReadAllText(codeFile)}}
    """;

    var chatCompletion = await chatClient.CompleteAsync(prompt);
    Console.WriteLine(chatCompletion.Message.Text);
    Console.WriteLine(Environment.NewLine);
    Console.WriteLine($"Total time of analysis {time.Elapsed.TotalSeconds}");
}


#endregion

#region Sample 01 - Classic Chatbot

/*
    Aşağıdaki sonsuz döngü kısmında istemci nesne birden çok kez çağırılmakta. Bu nedenle ilgili nesneyi 
    Dependency Injection Container'a alıp oradan tedarik ederek kullandırmak çok daha mantıklı.

    Sonsuz döngü aslında bir chatbot ile konuşma efektini vermek için.

    Sorulan sorular (prompts) bir ChatMessage nesnesi haline getirilerek kullanılmakta. 
    Aslında taraflar için tek bir nesne modeli var. Bunlardan hangisinin kullanıcıya ait olduğu hangisinin chatbot'tan beklenen 
    bir mesaj olduğu genellikle ChatRole struct nesnesi üzerinden belirleniyor. (User ve Assistant kullanımlarına dikkat edelim) 
*/
//Console.WriteLine("Asistant mode has been started...");

//var chatHistory = new List<ChatMessage>();

//while (true)
//{
//    Console.WriteLine("> Waiting for command...");
//    var userPrompt = Console.ReadLine();
//    if (string.IsNullOrWhiteSpace(userPrompt)) continue;
//    if (userPrompt.Equals("exit", StringComparison.CurrentCultureIgnoreCase)) break;
//    if (userPrompt.Equals("clear", StringComparison.CurrentCultureIgnoreCase))
//    {
//        chatHistory.Clear();
//        Console.WriteLine("Clearing history...");
//        Console.Clear();
//        continue;
//    }
//    chatHistory.Add(new ChatMessage(ChatRole.User, userPrompt));

//    var chatResponse = string.Empty;
//    await foreach (var item in chatClient.CompleteStreamingAsync(chatHistory))
//    {
//        Console.Write(item.Text);
//        chatResponse += item.Text;
//    }
//    chatHistory.Add(new ChatMessage(ChatRole.Assistant, chatResponse));
//    Console.WriteLine();
//}

#endregion