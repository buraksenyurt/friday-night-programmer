using CsvToJson;
using FluentFTP;
using Grpc.Core;

namespace GrcpUploadService.Services;

public class UploadService(ILogger<UploadService> logger) : CsvToJson.UploadService.UploadServiceBase
{
    private readonly string ftpHost = "ftp://localhost";
    private readonly string ftpUser = "userone";
    private readonly string ftpPass = "123";

    private readonly ILogger<UploadService> _logger = logger;

    public async override Task<FileUploadStatus> Upload(IAsyncStreamReader<FileChunk> requestStream, ServerCallContext context)
    {
        try
        {
            string fileName = Guid.NewGuid().ToString() + ".csv";
            string remotePath = $"/uploads/{fileName}";

            using (var ftpClient = new AsyncFtpClient(ftpHost, ftpUser, ftpPass))
            {
                await ftpClient.Connect();
                _logger.LogInformation("Ftp client connected");

                using (var ftpStream = await ftpClient.OpenWrite(remotePath))
                {
                    await foreach (var chunk in requestStream.ReadAllAsync())
                    {
                        await ftpStream.WriteAsync(chunk.Content.ToByteArray());
                    }
                }

                await ftpClient.Disconnect();
            }
            _logger.LogInformation("Uploaded {}", fileName);

            return new FileUploadStatus { Success = true, Message = $"File uploaded: {fileName}" };
        }
        catch (Exception ex)
        {
            _logger.LogError("Error on ftp upload process {}", ex.Message);
            return new FileUploadStatus { Success = false, Message = ex.Message };
        }
    }
}
