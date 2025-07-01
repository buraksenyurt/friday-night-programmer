/*
    Nuxt ile geliştirilen issue-app tarafından kullanılan dummy servistir.
    issue-app bu servise bir kullanıcı numarası gönderir. Bu servis geriye,
    onaycılarının döndürür. Dummy bir servis olduğundan sabit bir içerik
    dönmektedir.
*/
using Microsoft.AspNetCore.Http.Json;

var builder = WebApplication.CreateBuilder(args);
builder.Services.Configure<JsonOptions>(options =>
{
    options.SerializerOptions.PropertyNamingPolicy = null;
});
var app = builder.Build();

app.MapGet("/api/approvers/{userNo:int}", (int userNo) =>
    {
        var approvers = new[]{
            new Approver(10012,"Billi Cinn"),
            new Approver(10234,"Marry the Problem Solver"),
            new Approver(1024,"Ancelina Coli"),
            new Approver(234,"Zip Zip Maryo"),
            new Approver(134,"Denver Di Last Daynazor")
        };

        return Results.Ok(approvers);
    }

);

app.Run();

record Approver(int IdentityNo, string FullName);
