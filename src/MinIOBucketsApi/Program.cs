using Microsoft.OpenApi.Models;
using Minio;
using Minio.DataModel.Args;

/*
    MinIO hafifsiklet diyebileceğimiz Amazon S3 uyumlu bir nesne depolama hizmetidir.
    Private Cloude Storage olarak local geliştirmelerde kullanımı idealdir.
    S3 API uyumluluğu olduğundan sanki AWS-S3 ile çalışıyor gibi işlem yapabiliriz.
    Dağıtık sistemlerde de değerlendirilebilir. 
    Açık kaynaklı bir projedir.
    Aşağıdaki örnek uygulamada çok basit olarak bir bucket oluşturulması,
    download, upload, listeleme ve silme işlemleri ele alınmaktadır.
*/

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c =>
{
    c.SwaggerDoc("v1", new OpenApiInfo { 
        Title = "MinIOBucketsApi"
        , Version = "v1" 
    });
});

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI(c => c.SwaggerEndpoint("/swagger/v1/swagger.json", "MinIOBucketsApi v1"));
}

var minio = new MinioClient()
    .WithEndpoint("localhost:9008")
    .WithCredentials("admin", "password")
    .WithSSL(false)
    .Build();

app.MapGet("/", () => "MinIO API");

app.MapPost("/buckets/create/{bucketName}", async (string bucketName) =>
{
    await minio.MakeBucketAsync(new MakeBucketArgs().WithBucket(bucketName));
    return Results.Ok($"'{bucketName}' başarılı şekilde oluşturuldu.");
});

app.MapPost("/buckets/upload/{bucketName}/{objectName}", async (string bucketName, string objectName, IFormFile file) =>
{
    using var stream = file.OpenReadStream();
    await minio.PutObjectAsync(new PutObjectArgs()
        .WithBucket(bucketName)
        .WithObject(objectName)
        .WithStreamData(stream)
        .WithObjectSize(file.Length)
    );
    return Results.Ok($"'{objectName}' isimli dosya, '{bucketName}' deposuna başarılı şekilde yüklendi.");
})
.DisableAntiforgery();
/*
    DisableAntiforgery metodu ile Antiforgery korumasını kasten devre dışı bırakılmıştır. Üretim ortamlarında kullanmayın! Bu senaryoda sadece geliştirme amaçlı devre dışı bırakılmıştır.
    Varsayılan olarak Minimal API'de bu özellik kapalıdır ancak IFormFile kullanımı sırasında otomatik olarak etkinleştirilir.
    AntiForgery koruması, uygulamanın güvenliğini artırmak için kullanılan bir tekniktir. Bu teknik, uygulamanın kimlik doğrulama bilgilerini korumak için kullanılır.
    (Cross-Site Request Forgery) CSRF saldırılarına karşı koruma sağlar.
*/

app.MapGet("/buckets/download/{bucketName}/{objectName}", async (string bucketName, string objectName) =>
{
    var memoryStream = new MemoryStream();
    await minio.GetObjectAsync(new GetObjectArgs()
        .WithBucket(bucketName)
        .WithObject(objectName)
        .WithCallbackStream(async stream => await stream.CopyToAsync(memoryStream))
    );

    memoryStream.Position = 0;
    return Results.File(memoryStream, "application/octet-stream", objectName);
});

app.MapGet("/buckets/list/{bucketName}", async (string bucketName) =>
{
    var objects = new List<string>();
    await foreach (var obj in minio.ListObjectsEnumAsync(new ListObjectsArgs().WithBucket(bucketName)))
    {
        objects.Add(obj.Key);
    }
    return Results.Ok(objects);
});

app.MapDelete("/buckets/delete/{bucketName}/{objectName}", async (string bucketName, string objectName) =>
{
    await minio.RemoveObjectAsync(new RemoveObjectArgs()
        .WithBucket(bucketName)
        .WithObject(objectName)
    );
    return Results.Ok($"'{objectName}' nesnesi '{bucketName}' deposundan kaldırıldı.");
});

app.Run();