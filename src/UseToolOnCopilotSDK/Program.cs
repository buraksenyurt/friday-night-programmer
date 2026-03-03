using GitHub.Copilot.SDK;
using Microsoft.Extensions.AI;
using System.ComponentModel;

await using var client = new CopilotClient();

var getAllDealerStats = AIFunctionFactory.Create(
    () =>
    {
        // Optimizasyon için istemci tarafında servis çıktısı cache'lenebilir.
        var httpClient = new HttpClient();
        var response = httpClient.GetAsync("http://localhost:5101/dealers").Result;
        var content = response.Content.ReadAsStringAsync().Result;
        return content;
    },
    "get_all_dealer_stats",
    "Get the full dealer stats"
);

await using var session = await client.CreateSessionAsync(new SessionConfig
{
    Model = "claude-sonnet-4.5",
    Streaming = true,
    Tools = [getAllDealerStats],
    OnPermissionRequest = PermissionHandler.ApproveAll,
});

session.On(e =>
{
    switch (e)
    {
        case AssistantMessageDeltaEvent messageEvent:
            Console.Write(messageEvent.Data.DeltaContent);
            break;
        case SessionIdleEvent messageEvent:
            Console.WriteLine("");
            break;
    }
});

Console.WriteLine("📊  Dealer Stats Assistant (type 'exit' to quit)");
Console.WriteLine("   Try: 'Ankara Merkez stoğunda hangi ürünlerden kaç adet var?' or 'Şişli bayisinde Envidya GTX 2000 ekran kartlarından kaç adet kaldı?'\n");

while (true)
{
    Console.Write("--> ");
    var input = Console.ReadLine();

    if (string.IsNullOrEmpty(input) || input.Equals("exit", StringComparison.OrdinalIgnoreCase))
    {
        break;
    }

    Console.Write("Thinking...");
    await session.SendAndWaitAsync(new MessageOptions { Prompt = input });
    Console.WriteLine("\n");
}