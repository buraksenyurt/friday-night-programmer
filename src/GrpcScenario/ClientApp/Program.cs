using CsvToJson;
using Google.Protobuf;
using Grpc.Net.Client;

Console.WriteLine("Press any key to start testing");
Console.ReadLine();

using var channel = GrpcChannel.ForAddress("http://localhost:5266");
var uploadClient = new UploadService.UploadServiceClient(channel);

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
}

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
