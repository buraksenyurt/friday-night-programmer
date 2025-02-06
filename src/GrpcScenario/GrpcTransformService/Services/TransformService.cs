using CsvToJson;
using FluentFTP;
using Grpc.Core;
using System.Text.Json;

namespace GrpcTransformService.Services;

public class TransformService(ILogger<TransformService> logger) : CsvToJson.TransformService.TransformServiceBase
{
    private readonly string ftpHost = "ftp://localhost";
    private readonly string ftpUser = "userone";
    private readonly string ftpPass = "123";

    private readonly ILogger<TransformService> _logger = logger;

    public override async Task<FileTransformStatus> Transform(FileRequest request, ServerCallContext context)
    {
        try
        {
            string csvPath = $"/home/ftpuser/uploads/{request.FileName}";
            string jsonPath = $"/home/ftpuser/processed/{Path.GetFileNameWithoutExtension(request.FileName)}.json";

            using (var ftpClient = new AsyncFtpClient(ftpHost, ftpUser, ftpPass))
            {
                await ftpClient.Connect();

                using (var csvStream = new MemoryStream())
                {
                    if (!await ftpClient.DownloadStream(csvStream, csvPath))
                    {
                        _logger.LogError("CSV file not found on FTP");
                        throw new RpcException(new Status(StatusCode.NotFound, "CSV file not found on FTP"));
                    }

                    csvStream.Position = 0;
                    using var reader = new StreamReader(csvStream);
                    using var writer = new StringWriter();

                    var csv = new List<Dictionary<string, string>>();
                    string[] headers = (await reader.ReadLineAsync()).Split('|');

                    while (!reader.EndOfStream)
                    {
                        string[] values = (await reader.ReadLineAsync()).Split('|');
                        var row = headers.Zip(values, (header, value) => new { header, value })
                                         .ToDictionary(x => x.header, x => x.value);

                        csv.Add(row);
                    }

                    string jsonData = JsonSerializer.Serialize(csv);
                    byte[] jsonBytes = System.Text.Encoding.UTF8.GetBytes(jsonData);
                    _logger.LogInformation("{} bytes data serialized to json", jsonBytes.Length);

                    using var jsonStream = new MemoryStream(jsonBytes);
                    await ftpClient.UploadStream(jsonStream, jsonPath);
                }

                await ftpClient.Disconnect();
            }

            return new FileTransformStatus { Success = true, Message = "CSV successfully transformed to JSON!" };
        }
        catch (Exception ex)
        {
            _logger.LogError("Error on file transform operation. {}", ex.Message);
            return new FileTransformStatus { Success = false, Message = ex.Message };
        }
    }
}
