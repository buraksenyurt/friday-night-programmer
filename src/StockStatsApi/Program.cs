using Microsoft.AspNetCore.Http.HttpResults;
using System.Text.Json.Serialization;

var builder = WebApplication.CreateSlimBuilder(args);

builder.Services.ConfigureHttpJsonOptions(options =>
{
    options.SerializerOptions.TypeInfoResolverChain.Insert(0, AppJsonSerializerContext.Default);
});

builder.Services.AddOpenApi();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.MapOpenApi();
}
/*
    Şu sorulara cevap verebilecek türden zengin bir veri seti düşünelim.
    Pekala bir veritabanında geleceklerde. Demo için şimdilik yeterli.

    - "Ankara Merkez stoğunda hangi ürünlerden kaç adet var?"
    - "Şişli bayisinde Envidya GTX 2000 ekran kartlarından kaç adet kaldı?"
    - "Hangi bayilerde 32 GB RAM var?"
    - "Stoğuna RAM bulunmayan bayiler hangileri?"
    - "24 inç ve üstü monitör bulunduran bayiler hangileri?"
*/
Dealer[] data = StockStatsApi.SeedData.GetDealers();

var dataApi = app.MapGroup("/dealers");
dataApi.MapGet("/", () => data)
        .WithName("GetAllStats");

dataApi.MapGet("/{id}", Results<Ok<Dealer>, NotFound> (int id) =>
    data.FirstOrDefault(a => a.Id == id) is { } dealer
        ? TypedResults.Ok(dealer)
        : TypedResults.NotFound())
    .WithName("GetDealerById");

app.Run();

public record Dealer(int Id, string Title, string City, List<Stats> Stats);
public record Stats(int Id, string Product, int Quantity);

[JsonSerializable(typeof(Dealer[]))]
internal partial class AppJsonSerializerContext : JsonSerializerContext
{

}
