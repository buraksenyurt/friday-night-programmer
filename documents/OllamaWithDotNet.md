# Ollama Yardımıyla Deepseek Dil Modelini .Net Platformunda Kullanmak

Son yıllarda hayatımıza girmiş bulunan bir çok dil modeli ve pek tabii bunları işleterek çeşitli konularda bizi asiste eden GenAI ürünleri geliştirildi. Yeni modeller de geliştirilmeye, parametre sayıları milyarlar mertebesinde muazzam değerlere de ulaşmaya devam ediyor. Herhalde en popülerlerinden birisi ChatGPT olsa gerek. Ancak maliyet açısından bakıldığında bireysel kullanım için dahi olsa gerçekten işe yarar sonuçlara götürecek olan versiyonlar biraz pahalı gibi. Yazıyı kaleme aldığım tarih itibariyle benimde kullandığım bireysel paket fiyatı aylık 20 dolar seviyesinde. Oysa ki OpenAI'ın kuruluş aşamalarında her şeyin açık kaynak ve ücretsiz olacağına dair bir bildiri vardı diye hatırlıyorum. Geçen günlerde çıkan Çin merkezli [Deepseek](https://github.com/deepseek-ai) bu durumu biraz değiştirdi gibi. Çok daha düşük bir maliyetle _(ki haber kanallarında geçen bilgilere göre sadece 5.8 milyon dolar civarında bir yatırımla)_ tamamen açık kaynak sunulup epey de iyi bir sonuç elde ederek büyük oyuncuların tüm hisselerini kısa süreliğine de olsa sarsmış durumda.

_Yapay zeka dil modellerinin ve buna dayalı çalışan kod asistanlarının biz programcıların işini elimizden alacağına pek inanmıyorum. Bunun yerine verimliliğimizi artıracak şekilde bizi daha da iyi asiste edeceklerini düşünüyorum._

Yakın zamanda AI hizmetlerini .Net uygulamalarına adapte edebilmek için iki soyutlama paketi tanıtıldı. [Microsoft.Extensions.AI](https://learn.microsoft.com/en-us/dotnet/ai/ai-extensions) ve Microsoft.Extensions.AI.Abstractions. Bu kütüphanelerden yararlanarak belli başlı dil modeli servislerini kullanabiliyoruz. OpenAO, Azure OpenAI, Azure AI Infrence ve [Ollama](https://ollama.com/) gibi. Bu servisler birçok dil modelini çalıştırmak için birer sunucu gibi hareket ediyorlar. .Net kütüphaneleri ise bu servisleri kullanmak için gerekli fonksiyonellikleri sağlayarak kullanımı kolaylaştırıyor.

Bu özet yazıda söz konusu süreci nasıl işleteceğimizi bir örnek üzerinden ele almaya çalışacağız. Senaryomuz C# kod dosyalarının analiz edilmesi ve kod kalitesinin ölçülmesi üzerine bir çalışma olacak. Tabii çok basit bir şekilde ele alacağız ki niyetimiz Sonarqube metrikleri ile yarışmak değil. Dilerseniz adım adım ilerleyelim.

## Dil Modeli için Gerekli Ortamın Hazırlanması

İlk olarak seçtiğimiz dil modelini işletecek ortamı hazırlamak lazım. Bu noktada kodu analiz ettireceğimiz dil modelini çalıştıracak bir servise de ihtiyacımız var. Ben [Ollama'yı](https://ollama.com/) tercih ediyorum. Kodu yazmakta olduğum makine Windows 11 işletim sistemine sahip. Ollama'yı macOs ve Linux platformları için de kullanabiliyoruz.

```bash
# Kurulumun başarılı olup olmadığını versiyon kontrolü ile yapabilriz
ollama -v
```

İndirme ve kurulum işlemi tamamlandıktan sonra birde dil modeline ihtiyacımız olacak elbette. Ollama'nın [buradaki sayfasından](https://ollama.com/search) yararlanarak çalışmak istediğimiz dil modelini çalışacağı sisteme almamız lazım.Burada dikkat edilmesi gereken birkaç nokta var. Belli konulara özel geliştirilmiş dil modelleri mevcut. Örneğin kimisi görsel öğelere has kabiliyetlere sahip, yani fotoğraf analizi yaptırmak gibi konularda başarılı. Kimisi genel dil modelleri. Bazı dil modelleri çalışmak için yüksek konfigürasyon makinelere ihtiyaç duyabilir. Özellikle kaç parametre ile çalışacağımızı seçerken buna dikkat etmek lazım. Örneğimizde ben deepseek-r1 modelini ve 7 milyar parametre ile çalışan sürümünü kullanmayı tercih ettim.

![OllamaWithNet_00](../images/OllamaWithNet_00.png)

```bash
# Dil modelini sisteme almak için aşağıdaki komutu kullanmak yeterli
# Buna göre deepseek-r1 in 7 milyar parametre ile çalışan versiyonu kullanılacak
ollama run deepseek-r1:7b
```

Paket boyutlarına dikkat etmekte de yarar var. Parametre sayısının artması daha iyi ekran kartları haricinde daha fazla disk alanına da ihtiyaç duymamızı gerektirebilir :D Örnekte kullandığım Deepseek-r1:7b _(7 milyar parametre alan)_ versiyon 4.7 Gb'lık bir download paketine sahip.

![OllamaWithNet_01](../images/OllamaWithNet_01.png)

Diğer modellerin kapladığı alan ise şöyle. Yazıyı yazdığım zaman itibariyle Chat GPT'nin 4o modelinin tahminen 2 trilyona yakın parametre ile çalıştığı ifade ediliyordu. Deepseek'in uzmanlaşmış dil modellerinin bir araya geldiği 671 milyar parametrelik versiyonunun 404 Gb yer tuttuğu düşünülürse gerçekten en iyi kalitede işçilik için yine bol miktarda sıfırı olan finansmana ihtiyaç var gibi :D

![Models sizes](../images/OllamaWithNet_07.png)

Artık local makinede çalışan bir dil modelimiz mevcut. Hatta bunu list parametresi ile görebilmemiz lazım.

![OllamaWithNet_02](../images/OllamaWithNet_02.png)

Ollama ile bir dil modeli hizmeti başlatıldığında makine restart olsa bile servisin çalışmaya devam ettiğini fark ettim. Makinede yüklü olan servisleri görmek için ps komutu kullanılabilir.

```bash
# Hali hazırda çalışan servisleri görmek için
ollama ps

# Çalışan bir dil modelini durdurmak için (Örneğin deepseek'i durdurmak için)
ollama stop deepseek-r1:7b

# tekrar başlatmak için
ollama run deepseek-r1:7b

# Çalışan bir dil modeli hakkında bazı bilgileri elde etmek için
ollama show deepseek-r1:7b

# İşe yarar diğer terminal komutlarını görmek için
ollama help

# Bir komut hakkında kullanım bilgisi edinmek için (Örneğin remove model komutu ile ilgili)
ollama rm --help
```

Sıradaki adımımız Ollama servisini bir Console uygulamasında kullanabilmek.

## Hello World

Basit bir adımla başlayalım ve dil modeli ile karşılıklı sohbet edebileceğimiz bir kod parçası geliştirelim. Console uygulamasında AI soyutlamalarını kullanabilmek için aşağıdaki paketleri eklememiz gerekiyor. _(Yazıyı yazdığımız vakitte AI ve Ollama paketleri henüz prerelease sürümdeydi. Bu nedenle eklerken --prerelease ile eklemem gerekti)_

```bash
# Önce projeyi oluşturalım
dotnet new console -o HelloOllama

# Ardından gerekli paketleri ekleyelim
cd HelloOllama
dotnet add package Microsoft.Extensions.AI --prerelease
dotnet add package Microsoft.Extensions.AI.Ollama --prerelease
dotnet add package Microsoft.Extensions.Hosting # Dependency Injection Container için gerekli
```

İlk kodlarımızı da aşağıdaki gibi geliştirebiliriz.

```csharp
using Microsoft.Extensions.AI;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

var builder = Host.CreateApplicationBuilder();
/*
    Ollama servisi ile konuşacak bir client nesnesi söz konusu.
    Bu nesne localhost:11434 portundan çalışan servise gidip onun deepseek-r1 ile çalışmasını istiyor.

    Aşağıdaki sonsuz döngü kısmında istemci nesne birden çok kez çağırılmakta. Bu nedenle
    ilgili nesneyi Dependency Injection Container'a alıp oradan tedarik ederek kullandırmak çok daha mantıklı.

    Sonsuz döngü aslında bir chatbot ile konuşma efektini vermek için.

    Sorulan sorular (prompts) bir ChatMessage nesnesi haline getirilerek kullanılmakta. 
    Aslında taraflar için tek bir nesne modeli var. Bunlardan hangisinin kullanıcıya ait olduğu hangisinin chatbot'tan beklenen 
    bir mesaj olduğu genellikle ChatRole struct nesnesi üzerinden belirleniyor. (User ve Assistant kullanımlarına dikkat edelim)
*/
builder.Services.AddChatClient(new OllamaChatClient(new Uri("http://localhost:11434"), "deepseek-r1:7b"));
var app = builder.Build();
var chatClient = app.Services.GetRequiredService<IChatClient>();
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
```

Bu uygulamayı çalıştırdıktan sonra Deepseek ile konuşabiliriz. Örneğin dünyanın en yüsek 5 dağının listesini istediğimizi düşünelim. İşte sonuç. Bu arada Deepseek'in espri anlayışı da var gibi :D

![Ollama runtime sample 1](../images/OllamaWithNet_03.png)

Bu arada kodu çalıştırdıktan ve soruyu sorduktan sonra buraya dönüp makaleyi düzenlemeye devam ettim ve üstünden neredeyse beş dakika geçti. Deepseek kararsızlıklar yaşayarak cevaplar vermeye devam etti ve duruma göre ilk beşteki dağlardan bazıları arasında sıralama değişiklikleri yaptı. Bu elbette kullandığımız dil modelinin 7 milyar parametreli versiyonundan ya da sorduğum sorunun kalitesinden de kaynaklanıyor olabilir.

![Ollama runtime sample 2](../images/OllamaWithNet_04.png)

Pek tabii daha üst modelleri çalıştırmak için daha güçlü bir sisteme ihtiyacım var. Bu noktada güçlü sunucularda daha iyi ve yüksek parametreli dil modellerinden çok daha iyi sonuçlar alınabileceği öngörülebilir fakat şu haliyle dahi Deepseek bana kalırsa epey etkili.

Şimdi yazımızın başlarında belirttiğim senaryo ile devam edelim. C# dosyalarını bu dil modeline verip kalitesini yorumlatmak istiyoruz. Pek tabii burada çok iyi prompt girilmesi gerekiyor. Dolayısıyla farklı bir yaklaşıma gideceğiz. Kodlarımızı aşağıdaki gibi geliştirerek ilerleyelim.

```csharp
using Microsoft.Extensions.AI;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.Diagnostics;

var builder = Host.CreateApplicationBuilder();
builder.Services.AddChatClient(new OllamaChatClient(new Uri("http://localhost:11434"), "deepseek-r1:7b"));
var app = builder.Build();
var chatClient = app.Services.GetRequiredService<IChatClient>();

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
```

Örnek kodun en önemli kısmı prompt içeriği. Burada görüldüğü üzere chatbot konuşmalarından çok daha farklı bir bildiri söz konusu. İstediğimiz kod analizini yapması için dil modeline detaylı bilgiler veriyoruz. Bu arada söz konusu promptu chatgpt'ye yaptırdığımı ifade edeyim ama kendisi Deepseek için bunu istediğimi henüz önemsememiş gibi :D Dolayısıyla Prompt Engineering mevzusu oldukça önemli olabilir. Sonuçta bu tip bir promptu yazdırmak içinde iyi seviyede programlama bilgisine en azından programlama dilinin yapısı ile ilgili kavramlara hakim olmak lazım. Korkma sayın programcı bize hala iş var :D Neyse biz konumuza geri dönelim dilerseniz. Örneğin aşağıdaki kod parçası için çalıştırdım.

```csharp
using System;

public class GameInfo
{
    public string Name { get; set; }
    public string Description { get; set; }
    public int UserPoint { get; private set; }

    public GameInfo(string name, string description)
    {
        Name = name;
        Description = description;
    }
    public void IncreaseAveragePoint(int value)
    {
        UserPoint += value;
    }
    public override string ToString()
    {
        return $"{Name}:{Description}";
    }
}
```

Programı çalıştırmadan önce bu kod dosyasını yorumlamanızı öneririm. Birkaç özellik barındıran basit bir sınıf tasarımı söz konusu. Object sınıfından gelen ToString metodu override edilmiş halde. Ayrıca dışarıya kapatılmış UserPoint özelliğini değiştirebildiğimiz birde metodumuz var. String interpolation kullanılmış vs Bakalım DeepSeek-r1:7b dil modeli bunu nasıl yorumluyor. İşte çalışma zamanına ait bir ekran görüntüsü.

![Code metrix runtime 1](../images/OllamaWithNet_05.png)

Bu basit kod dosyası için ilgili dil modelinin epey isabetli sonuçlara ulaştığını söylemek yanlış olmaz her halde. Tüm analiz örneği geliştirdiğim bilgisayarda yaklaşık olarak 2.5 dakika kadar sürdü diyebilirim. Bu arada makinenin özelliklerini de paylaşayım.

| Key                   | Value                             |
| ----------------------| --------------------------------- |
| System Manufacturer | MONSTER   |
| System Model | HUMA H4 V5.2  |
| OS Name | Microsoft Windows 11 Pro |
| Processor |12th Gen Intel(R) Core(TM) i7-1255U, 1700 Mhz, 10 Core(s), 12 Logical Processor(s) |
| RAM |32.0 GB    |
| VGA | Intel(R) Iris(R) Xe Graphics  |

Elbette prompt içeriğini biraz daha detaylandırıp çıktı isteyebiliriz. Sevgili Çeto'nun (ChatGpt'ye böyle diyorum) katkılarıyla aşağıdaki promptu deneyebiliriz.

```text
You are an expert in analyzing and evaluating C# source code. You will receive a C# code file as input, and your task is to analyze it and produce a structured JSON response that includes:

1. **Functionality Summary**: A brief description of what the code does.
2. **Key Components**: A list of major classes, methods, and their responsibilities.
3. **Potential Issues**: A list of possible issues such as security vulnerabilities, performance bottlenecks, or bad coding practices.
4. **Code Quality Score**: A rating (1-10) based on readability, maintainability, and adherence to best practices.
5. **Recommendations**: Concrete suggestions to improve the code quality.

## Important Notes:
- Provide **only** a strict RFC8259 compliant JSON response.
- Do **not** modify or infer missing parts of the code.
- If the code is incomplete, specify it in the `"notes"` section.

## JSON Format Example:
```json
{
    "functionality_summary": "Brief description of what the code does.",
    "key_components": [
        {
            "name": "ClassName",
            "type": "class",
            "description": "Purpose of this class"
        },
        {
            "name": "MethodName",
            "type": "method",
            "description": "What this method does"
        }
    ],
    "potential_issues": [
        "List of possible security risks, performance issues, or bad practices"
    ],
    "code_quality_score": 8,
    "recommendations": [
        "Improve variable naming",
        "Refactor long methods into smaller functions"
    ],
    "notes": "Additional comments if necessary"
}
```

Kendi sistemimde bu prompt için aşağıdaki çıktıyı elde ettiğimi ifade edebilirim.

![Code metrix runtime 2](../images/OllamaWithNet_06.png)

Bu sefer bu basit C# dosyasının analizi kendi sistemimde neredeyse beş dakikaya yakın sürede tamamlandı ancak biraz daha detaylı bilgi aldığımızı ifade edebilirim. Hatta yorum kısımlarında Deepseek sanki gerçekten Code Review yapan bir programcıymış gibi davranıyor desek yalan olmaz. 

Yazının bundan sonraki kısmında farklı modellerden çeşitli prompt'lar üretip söz konusu dosyanın yorumlanmasını isteyebiliriz. Lakin endüstriyel anlamda baktığımda milyon satır kod tabanına ulaşabilen sistemlerin kod dosyalarını kalitesinin ölçümü için çok daha fazla parametre ile çalışan _(ki tahminlere göre Chat Gpt 4o versiyonu neredeyse 2 trilyon parametre ile çalışıyor)_ ve pek tabii çok daha yüksek sistem konfigürasyonuna ihtiyaç duyan ve pek tabiii daha çok enerji ihtiyacı duyacak ortamlara ihtiyacımız olacağı kesin. Tüm bu gelişmelere karşın Microsoft'un yapay zeka modellerini kod tabanında kolayca kullanabilmemiz için soyutlamalar getirmesi, Ollama'nın OpenAI'ın tüm karşıt görüşleri ve tutumlarına rağmen DeepSeek'i model kataloğunda tutması çok farklı bir geleceğin göstergesi gibi.

## Modeli Zorlayalım

Gelin modeli biraz daha zorlayalım. Bu sefer içerisinde bazı SOLID ilkelerinin ihlal edildiği aşağıdaki kod dosyasını ele alalım.

```csharp
using System;
using System.Collections.Generic;
using System.IO;
using System.Net.Mail;

public class Developer
{
    public string Username { get; set; }
    public string Password { get; set; }
    public string Email { get; set; }
}

public class CompanyManager
{
    private List<Developer> _developers = new();

    public void RegisterDeveloper(string username, string password, string email)
    {
        var developer = new Developer { Username = username, Password = password, Email = email };
        _developers.Add(developer);
        Console.WriteLine($"{username} registered successfully.");
        SendEmail(email, "Welcome", "Thank you for registering!");
    }

    public void SendEmail(string to, string subject, string body)
    {
        try
        {
            var client = new SmtpClient("smtp.azoncorp.com");
            var mailMessage = new MailMessage("no-reply@example.com", to, subject, body);
            client.Send(mailMessage);
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex);
        }
    }
}

public class Logger
{
    public void Log(string message)
    {
        File.AppendAllText("log.txt", $"{DateTime.Now}: {message}\n");
    }
}

class Program
{
    static void Main()
    {
        var companyManager = new CompanyManager();
        companyManager.RegisterDeveloper("johndoe", "P@ssw0rd", "john.doe@azoncorp.com");

        var logger = new Logger();
        logger.Log("Developer registered.");
    }
}
```

Bu sefer cevapları almak biraz daha fazla sürdü elbette. Bu basit iyi bir programcının bir dakikadan az sürede analiz edebileceği kod içeriği için neredeyse 7 dakika civarında. Ancak DeepSeek'in bunu nasıl yorumladığını da paylaşmak isterim.

Yorumlama kısmı;

```text
Okay, so I'm looking at this C# code that someone provided. Let me try to understand what it does and figure out the analysis parts.

First, there are three main classes: Developer, CompanyManager, and Logger. The Program class has a static Main method which creates instances of these classes and calls some methods.

Starting with the Developer class-it's pretty straightforward. It has properties for Username, Password, and Email, but no constructors or setters beyond those properties. So when you create a new Developer, you have to set all three properties each time.

Next is CompanyManager. It has a private list of Developers called _developers. The RegisterDeveloper method takes in username, password, email, creates a new Developer object, adds it to the list, and sends an email with some standard messages. I notice that in the SendEmail method, they're using the SmtpClient class but not closing the client after sending emails. That might be a resource leak because once the program ends, the client could remain open.

The SendEmail method creates a new MailMessage each time it's called, which is fine but maybe inefficient if done frequently. Also, catching exceptions just logs them to the console without any meaningful error messages-maybe that should provide more details for debugging or include logging of errors.

Looking at the Logger class, the Log method appends messages to log.txt using DateTime.Now. It doesn't have any logging level flags like Debug, Info, Warning, etc., which could be useful for differentiating message severities. Also, if this is part of a larger system, it might not handle rotation or truncation correctly-though that's more about how the file is managed elsewhere.

In the Program class, Main is static and top-level as expected in an executable class. It creates a CompanyManager instance and calls RegisterDeveloper with test values. Then it logs that a developer was registered. There are no issues shown here at first glance-it seems correct but maybe could be made more robust.

Now, moving on to the analysis questions:

1. Functionality Summary: The code appears to manage developers for an application. It allows registering developers with basic information and sends confirmation emails. Logging is done to a file for record-keeping.

2. Key Components:
   - Developer class: Manages individual developer data.
   - CompanyManager class: Handles registration of developers, including adding them to the list and sending emails.
   - Logger class: Logs messages to a text file.

3. Potential Issues:
   - Resource leak in SendEmail method where the SmtpClient is not closed after use.
   - Inefficient email sending by recreating MailMessage objects each time.
   - No logging of error exceptions, which could hinder debugging if something goes wrong during email sending.
   - Lack of logging for other parts like registration success or failures-though in this code, it's just a test, so maybe not an issue here. But if there were errors in RegisterDeveloper, the logs wouldn't capture them unless added.

4. Code Quality Score: Considering readability and maintainability. The code is okay but could use better variable names (like email instead of to). There are some inefficiencies with creating MailMessage each time. Could benefit from method-level logging for errors or warnings. The list in Program's Main isn't defensive programming, so it might throw an exception if the constructor fails.

5. Recommendations:
   - Rename variables like 'to' to something more descriptive.
   - Use string formatting instead of concatenation for better readability and maintainability.
   - Refactor the SendEmail method to send multiple emails in a single call if possible or improve efficiency.
   - Close resources properly by using try-with-resources where applicable, especially for the SmtpClient.
   - Consider adding logging for error cases when email sending fails so that issues can be tracked down more easily.

Notes: The code as provided seems complete. It's a simple example without any missing parts mentioned.s
```

Üretilen JSON içeriği ise şu şekilde.

![Code metrix runtime last](../images/OllamaWithNet_08.png)

Bu kadar şeyden sonra artık daha ne desem bilemedim :D Biz programcılara halen daha çok ihtiyaç var ve bu düşüncemin sonuna kadar arkasındayım.
