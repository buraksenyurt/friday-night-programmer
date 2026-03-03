using System.Net.Http.Headers;
using System.Runtime.Serialization;
using GitHub.Copilot.SDK;

await using var client = new CopilotClient();

var modelName = "claude-sonnet-4.5";
var intro = $"Starting Copilot session with model: {modelName}";
Console.WriteLine(intro);
Console.WriteLine(new string('-', intro.Length));
Console.WriteLine();

await using var session = await client.CreateSessionAsync(
    new SessionConfig
    {
        Model = modelName,
        OnPermissionRequest = PermissionHandler.ApproveAll,
        Streaming = true
    }
);

session.On(e =>
{
    if (e is AssistantMessageDeltaEvent deltaEvent)
    {
        Console.Write(deltaEvent.Data.DeltaContent);
    }
    if (e is SessionIdleEvent) Console.WriteLine();
});

await session.SendAndWaitAsync(
    new MessageOptions
    {
        Prompt = "Give me the first 10 Fibonacci numbers and then find the related Norris Joke."
    }
);

