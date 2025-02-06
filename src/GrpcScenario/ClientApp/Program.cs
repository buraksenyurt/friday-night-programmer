using CsvToJson;
using Google.Protobuf;
using Grpc.Core;
using Grpc.Net.Client;

Console.WriteLine("Press any key to start testing");
Console.ReadLine();

using var uploadFileChannel = GrpcChannel.ForAddress("http://localhost:5266");
var uploadClient = new UploadService.UploadServiceClient(uploadFileChannel);

var sampleFilePath = Path.Combine(Environment.CurrentDirectory, "90s_video_games.csv");

if (!File.Exists(sampleFilePath))
{
    Console.WriteLine("Sorry. I can't find the test file.");
    return;
}

var fileUploadStatus = await UploadCsvFile(uploadClient, sampleFilePath);
if (fileUploadStatus.Success)
{
    Console.WriteLine(fileUploadStatus.Message);
}
else
{
    Console.WriteLine("File upload unsucceded.");
    return;
}

using var transformFileChannel = GrpcChannel.ForAddress("http://localhost:5131");
var transformClient = new TransformService.TransformServiceClient(transformFileChannel);

var response = await transformClient.TransformAsync(new FileRequest { FileName = fileUploadStatus.CreatedFileName });
Console.WriteLine($"{response.Message}");

if (!response.Success)
{
    Console.WriteLine(response.Message);
    return;
}

using var accessFileChannel = GrpcChannel.ForAddress("http://localhost:5231");
var accessClient = new AccessService.AccessServiceClient(accessFileChannel);
var call = accessClient.Get(new FileRequest { FileName = Path.GetFileNameWithoutExtension(fileUploadStatus.CreatedFileName) + ".json" });

Console.WriteLine("JSON Content:\n");
await foreach (var chunk in call.ResponseStream.ReadAllAsync())
{
    string jsonPart = System.Text.Encoding.UTF8.GetString(chunk.Content.ToByteArray());
    Console.Write(jsonPart);
}

Console.WriteLine("Press any key to exit");
Console.ReadLine();

static async Task<FileUploadStatus> UploadCsvFile(UploadService.UploadServiceClient client, string filePath)
{
    using var call = client.Upload();
    string fileName = Path.GetFileName(filePath);

    await using var fileStream = File.OpenRead(filePath);
    byte[] buffer = new byte[1024 * 64];
    int bytesRead;

    while ((bytesRead = await fileStream.ReadAsync(buffer)) > 0)
    {
        await call.RequestStream.WriteAsync(new FileChunk
        {
            Content = ByteString.CopyFrom(buffer, 0, bytesRead),
            FileName = fileName
        });
    }

    await call.RequestStream.CompleteAsync();
    return await call.ResponseAsync;
}
