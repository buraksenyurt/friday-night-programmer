using CsvToJson;
using FluentFTP;
using Google.Protobuf;
using Grpc.Core;

namespace GrpcAccessService.Services;

public class AccessService(ILogger<AccessService> logger) : CsvToJson.AccessService.AccessServiceBase
{
    private readonly string ftpHost = "ftp://localhost";
    private readonly string ftpUser = "userone";
    private readonly string ftpPass = "123";
    private readonly ILogger<AccessService> _logger = logger;

    public async override Task Get(FileRequest request, IServerStreamWriter<FileChunk> responseStream, ServerCallContext context)
    {
        _logger.LogInformation("Requested file {}", request.FileName);

        string remotePath = $"/home/ftpuser/processed/{request.FileName}";

        using var ftpClient = new AsyncFtpClient(ftpHost, ftpUser, ftpPass);
        await ftpClient.Connect();

        using (var memoryStream = new MemoryStream())
        {
            if (!await ftpClient.DownloadStream(memoryStream, remotePath))
            {
                _logger.LogError("JSON file not found on FTP server");
                throw new RpcException(new Status(StatusCode.NotFound, "JSON file not found on FTP"));
            }

            memoryStream.Position = 0;
            byte[] buffer = new byte[1024 * 64];
            int bytesRead;

            while ((bytesRead = await memoryStream.ReadAsync(buffer)) > 0)
            {
                await responseStream.WriteAsync(new FileChunk
                {
                    Content = ByteString.CopyFrom(buffer, 0, bytesRead),
                    FileName = request.FileName
                });
            }

            _logger.LogInformation("{} bytes JSON context fetched", memoryStream.Length);
        }

        await ftpClient.Disconnect();
    }
}
