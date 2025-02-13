using Microsoft.Extensions.AI;

namespace HelloOllama.Drivers;

class Basics
{
    /*
         Aşağıdaki sonsuz döngü kısmında istemci nesne birden çok kez çağırılmakta. Bu nedenle ilgili nesneyi 
         Dependency Injection Container'a alıp oradan tedarik ederek kullandırmak çok daha mantıklı.

         Sonsuz döngü aslında bir chatbot ile konuşma efektini vermek için.

         Sorulan sorular (prompts) bir ChatMessage nesnesi haline getirilerek kullanılmakta. 
         Aslında taraflar için tek bir nesne modeli var. Bunlardan hangisinin kullanıcıya ait olduğu hangisinin chatbot'tan beklenen 
         bir mesaj olduğu genellikle ChatRole struct nesnesi üzerinden belirleniyor. (User ve Assistant kullanımlarına dikkat edelim) 
    */
    public static async Task Run(IChatClient chatClient)
    {
        Console.WriteLine("Asistant mode has been started...");

        var chatHistory = new List<ChatMessage>();

        while (true)
        {
            Console.WriteLine("> Waiting for command...");
            var userPrompt = Console.ReadLine();
            if (string.IsNullOrWhiteSpace(userPrompt)) continue;
            if (userPrompt.Equals("exit", StringComparison.CurrentCultureIgnoreCase)) break;
            if (userPrompt.Equals("clear", StringComparison.CurrentCultureIgnoreCase))
            {
                chatHistory.Clear();
                Console.WriteLine("Clearing history...");
                Console.Clear();
                continue;
            }
            chatHistory.Add(new ChatMessage(ChatRole.User, userPrompt));

            var chatResponse = string.Empty;
            await foreach (var item in chatClient.CompleteStreamingAsync(chatHistory))
            {
                Console.Write(item.Text);
                chatResponse += item.Text;
            }
            chatHistory.Add(new ChatMessage(ChatRole.Assistant, chatResponse));
            Console.WriteLine();
        }
    }
}
