using Microsoft.AspNetCore.Mvc;

namespace EcoFriendlyApi.Controllers;

[ApiController]
[Route("[controller]")]
public class EcoWinnerCenter : ControllerBase
{
    private readonly ILogger<EcoWinnerCenter> _logger;

    public EcoWinnerCenter(ILogger<EcoWinnerCenter> logger)
    {
        _logger = logger;
    }

    [HttpGet(Name = "GetWinners")]
    public IEnumerable<OurFriend> Get()
    {
        var random = new Random();

        var testNames = new List<string>
        {
            "Jane Smith",
            "Charlie Davis",
            "John Doe",
            "Bob Brown",
            "Ivy Walker",
            "Eve Wilson",
            "Frank Harris",
            "Grace Lee",
            "Hank Young",
            "Alice Johnson",
        };

        var winners = Enumerable.Range(1, 10).Select(id =>
        {
            var lastWaste = Math.Round(random.NextDouble() * (10 - 3) + 3, 2);
            var allTimeWaste = Math.Round(lastWaste + random.NextDouble() * random.Next(10, 50), 2);
            return new OurFriend
            {
                Id = id,
                Fullname = testNames[random.Next(testNames.Count)],
                Summary = new Summary
                {
                    LastWasteTotal = lastWaste,
                    AllTimeWasteTotal = allTimeWaste,
                    LastDeliveryDate = DateTime.Now.AddDays(-random.Next(1, 11))
                }
            };
        });

        _logger.LogInformation("{} winners are ready", winners.Count());

        return winners.OrderByDescending(w => w.Summary.AllTimeWasteTotal);
    }
}
