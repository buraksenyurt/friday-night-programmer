/*
    OpenAI Api Key'i proje kök dizinindeki .env dosyasında tutuyoruz:
    OPENAI_API_KEY=sk-proj-...

    Alternatif olarak sistem ortam değişkeni de kullanılabilir:
    setx OPENAI_API_KEY "your_api_key"
    (Not: setx yalnızca yeni terminal oturumlarında geçerli olur)
*/
using dotenv.net;
using LangChain.DocumentLoaders;
using LangChain.Prompts;
using LangChain.Schema;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.VectorData;
using Microsoft.SemanticKernel.Connectors.InMemory;
using OpenAI.Chat;
using OpenAI.Embeddings;

namespace HelloLangchain;

public class Program
{
    public static async Task Main()
    {
        using var loggerFactory = LoggerFactory.Create(builder =>
        {
            builder
                .AddSimpleConsole(options =>
                {
                    options.ColorBehavior = Microsoft.Extensions.Logging.Console.LoggerColorBehavior.Enabled;
                    options.SingleLine = true;
                    options.TimestampFormat = "HH:mm:ss ";
                })
                .SetMinimumLevel(LogLevel.Information);
        });
        var logger = loggerFactory.CreateLogger<Program>();

        logger.LogDebug(".env dosyası yükleniyor...");
        DotEnv.Load(options: new DotEnvOptions(envFilePaths: ["../.env"]));
        logger.LogInformation(".env dosyası başarıyla yüklendi");

        logger.LogInformation("OpenAI Chat client oluşturuluyor...");
        ChatClient client = new(model: "gpt-5-mini", apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY"));
        logger.LogInformation("Chat client başarıyla oluşturuldu");

        logger.LogInformation("HTML dokümanı yükleniyor...");
        var loader = new HtmlLoader();
        var document = await loader.LoadAsync(DataSource.FromUrl("https://python.langchain.com/docs/get_started/introduction"));
        logger.LogInformation("Doküman başarıyla yüklendi: {PageCount} sayfa", document.Count);

        logger.LogDebug("Prompt template oluşturuluyor...");
        var template = "You are an expert software engineer who answers {question} based on {context}.";
        var prompt = new PromptTemplate(new PromptTemplateInput(template, inputVariables: ["question", "context"]));

        logger.LogInformation("Embedding client oluşturuluyor...");
        EmbeddingClient embeddingClient = new(
            model: "text-embedding-3-small"
            , apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY"));
        logger.LogInformation("Embedding client başarıyla oluşturuldu");

        logger.LogInformation("Vector store oluşturuluyor...");
        var vectorStore = new InMemoryVectorStore();
        var collection = vectorStore.GetCollection<string, DocumentChunk>("rag-documents");
        await collection.EnsureCollectionExistsAsync();
        logger.LogInformation("Vector store koleksiyonu hazır: rag-documents");

        logger.LogInformation("Doküman chunk'lara bölünüyor...");
        var fullText = string.Join("\n", document.Select(d => d.PageContent));
        var chunks = fullText.Chunk(500) // her chunk ~500 karakter
                             .Select(chars => new string([.. chars]))
                             .ToList();

        logger.LogInformation("Doküman {ChunkCount} parçaya bölündü", chunks.Count);
        Console.WriteLine($"Doküman {chunks.Count} parçaya bölündü.");

        logger.LogInformation("Embedding'ler oluşturuluyor ve vector store'a kaydediliyor...");
        var embeddingOptions = new EmbeddingGenerationOptions { Dimensions = 1536 };
        for (int i = 0; i < chunks.Count; i++)
        {
            logger.LogDebug("Chunk {Index}/{Total} için embedding oluşturuluyor", i + 1, chunks.Count);
            var embeddingResult = await embeddingClient.GenerateEmbeddingAsync(chunks[i], embeddingOptions);
            await collection.UpsertAsync(new DocumentChunk
            {
                Id = $"chunk-{i}",
                Text = chunks[i],
                Embedding = embeddingResult.Value.ToFloats()
            });
        }
        logger.LogInformation("Tüm embedding'ler başarıyla oluşturuldu ve kaydedildi");
        Console.WriteLine("Embedding'ler oluşturuldu ve vector store'a kaydedildi.");
        Console.WriteLine("\nBir soru sorun(örneğin: What is LangChain?):");

        var question = Console.ReadLine() ?? "What is LangChain?";
        Console.WriteLine($"\nSoru: {question}");
        logger.LogInformation("Kullanıcı sorusu alındı: {Question}", question);
        
        logger.LogDebug("Soru için embedding oluşturuluyor...");
        var queryEmbedding = await embeddingClient.GenerateEmbeddingAsync(question, embeddingOptions);
        logger.LogDebug("Soru embedding'i oluşturuldu");
        logger.LogDebug("Soru embedding'i oluşturuldu");
        
        logger.LogInformation("Similarity search yapılıyor (top 3)...");
        var searchResults = collection.SearchAsync(
            queryEmbedding.Value.ToFloats(),
            top: 3
        );

        var contextParts = new List<string>();
        await foreach (var result in searchResults)
        {
            logger.LogDebug("Benzer chunk bulundu - Skor: {Score:F4}, ID: {Id}", result.Score, result.Record.Id);
            Console.WriteLine($"  Skor: {result.Score:F4} | {result.Record.Text[..Math.Min(80, result.Record.Text.Length)]}...");
            contextParts.Add(result.Record.Text);
        }
        logger.LogInformation("{Count} benzer chunk bulundu ve context oluşturuldu", contextParts.Count);
        var context = string.Join("\n\n", contextParts);

        logger.LogDebug("Prompt formatlanıyor...");
        var systemMessage = await prompt.FormatAsync(new InputValues(new Dictionary<string, object>
        {
            ["question"] = question,
            ["context"] = context
        }));
        logger.LogDebug("Prompt hazırlandı: {SystemMessage}", systemMessage);

        logger.LogInformation("LLM'den yanıt bekleniyor...");
        ChatCompletion ragCompletion = await client.CompleteChatAsync(
        [
            new SystemChatMessage(systemMessage),
            new UserChatMessage(question)
        ]);
        logger.LogInformation("LLM yanıtı alındı");
        logger.LogDebug("Yanıt içeriği: {Content}", ragCompletion.Content[0].Text);
        
        Console.WriteLine($"\nSorunuz: {question}");
        Console.WriteLine($"Cevap: {ragCompletion.Content[0].Text}");
        
        logger.LogInformation("RAG Application tamamlandı");

    }
}

public class DocumentChunk
{
    [VectorStoreKey]
    public string Id { get; set; } = string.Empty;

    [VectorStoreData]
    public string Text { get; set; } = string.Empty;

    [VectorStoreVector(1536, DistanceFunction = DistanceFunction.CosineSimilarity)]
    public ReadOnlyMemory<float> Embedding { get; set; }
}
