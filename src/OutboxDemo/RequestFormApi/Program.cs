using System.Text.Json;
using Microsoft.EntityFrameworkCore;
using RequestFormApi.Db;
using RequestFormApi.Model;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
// PoC amaçlı. Gerçek hayatta böyle yapmayın :D
builder.Services.AddDbContext<DealerDbContext>(options => options.UseNpgsql("Host=localhost;Database=dealer;Username=johndoe;Password=somew0rds"));

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

app.MapPost("/api/serviceform", async (ServiceRequestForm requestForm, DealerDbContext db) =>
{

    requestForm.CreateDate = DateTime.UtcNow;

    var outboxMessage = new OutboxMessage
    {
        Id = Guid.NewGuid(),
        EventType = "ServiceRequestForm_Created",
        Payload = JsonSerializer.Serialize(new
        {
            requestForm.Id,
            requestForm.CustomerFullName,
            requestForm.Description,
            requestForm.ServiceRepresentativeId,
            requestForm.CreateDate
        }
        ),
        IsSent = false,
        CreateDate = DateTime.UtcNow
    };

    db.ServiceRequestForms.Add(requestForm);
    db.OutboxMessages.Add(outboxMessage);

    await db.SaveChangesAsync(); // Değişiklikleri tek seferde tek bir transaction ile yapar

    return Results.Created($"/api/serviceform/{requestForm.Id}", requestForm);

}).WithName("DealerServiceRequestForm").WithOpenApi();

app.Run();
