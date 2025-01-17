using Moq.Protected;
using Moq;
using System.Net;
using CallMeSdk.Core.Interfaces;
using CallMeSdk.Core.Models;
using CallMeSdk.Providers.Providers;
using Microsoft.Extensions.Logging;

namespace CallMeSdk.Tests;

public class RestDataProviderTests
{
    [Fact]
    public async Task FetchAsync_ShouldReturnParsedCustomers_When_ApiReturnsValidData()
    {
        // Arrange
        var mockHttpHandler = new Mock<HttpMessageHandler>();
        mockHttpHandler
            .Protected()
            .Setup<Task<HttpResponseMessage>>(
                "SendAsync",
                ItExpr.IsAny<HttpRequestMessage>(),
                ItExpr.IsAny<CancellationToken>()
            )
            .ReturnsAsync(new HttpResponseMessage
            {
                StatusCode = HttpStatusCode.OK,
                Content = new StringContent("[{\"Fullname\":\"Billy the Geyts\"},{\"Fullname\":\"Zeina Callibur\"}]")
            });

        var httpClient = new HttpClient(mockHttpHandler.Object);

        var mockParser = new Mock<ICustomerDataParser>();
        mockParser
            .Setup(p => p.ConvertFrom(It.IsAny<string>()))
            .Returns(
            [
                new CustomerBase { Fullname = "Billy the Geyts" },
                new CustomerBase { Fullname = "Zeina Callibur" }
            ]);

        var mockLogger = new Mock<ILogger>();

        var apiConfig = new RestConfiguration
        {
            BaseUrl = new Uri("https://api.azonbank.test.com"),
            Endpoint = "/customers"
        };

        var provider = new RestDataProvider(mockParser.Object, httpClient, apiConfig, mockLogger.Object);

        // Act
        var customers = await provider.FetchAsync();

        // Assert
        Assert.Equal(2, customers.Count());
        Assert.Equal("John Doe", customers.First().Fullname);
    }
}
