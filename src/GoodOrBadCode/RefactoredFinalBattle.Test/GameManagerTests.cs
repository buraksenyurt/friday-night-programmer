using RefactoredFinalBattle.Library;

namespace RefactoredFinalBattle.Test;

public class GameManagerTests
{
    [Fact]
    public void AddPlayer_ShouldAddPlayer_WhenLevelIsWithinRange()
    {
        // Arrange
        var levelPolicy = new ProCompetitionPolicy();
        var gameManager = new GameManager(levelPolicy);

        // Act
        var result = gameManager.AddPlayer("ProPlayer", 85);

        // Assert
        Assert.True(result.IsSuccess);
        Assert.NotNull(result.Value);
        Assert.Equal("ProPlayer", result.Value.Name);
        Assert.Equal(85, result.Value.Level);
        Assert.Single(gameManager.GetPlayers());
    }

    [Fact]
    public void AddPlayer_ShouldFail_WhenLevelIsTooLow()
    {
        // Arrange
        var levelPolicy = new ProCompetitionPolicy();
        var gameManager = new GameManager(levelPolicy);

        // Act
        var result = gameManager.AddPlayer("TooLow", 60);

        // Assert
        Assert.False(result.IsSuccess);
        Assert.Contains("Player level must be between", result.Message);
        Assert.Empty(gameManager.GetPlayers());
    }

    [Fact]
    public void AddPlayer_ShouldFail_WhenLevelIsTooHigh()
    {
        // Arrange
        var levelPolicy = new RookieCompetitionPolicy();
        var gameManager = new GameManager(levelPolicy);

        // Act
        var result = gameManager.AddPlayer("TooHigh", 55);

        // Assert
        Assert.False(result.IsSuccess);
        Assert.Contains("Player level must be between", result.Message);
        Assert.Empty(gameManager.GetPlayers());
    }

    [Theory]
    [InlineData(10, true)]  // Rookie (0-50) → 10 valid
    [InlineData(45, true)]  // Rookie (0-50) → 45 valid
    [InlineData(55, false)] // Rookie (0-50) → 55 invalid
    [InlineData(65, true)]  // Normal (50-70) → 65 valid
    [InlineData(100, true)] // Pro (70-100) → 100 valid
    [InlineData(101, false)] // Pro (70-100) → 101 invalid
    public void AddPlayer_ShouldValidateLevelsCorrectly(int level, bool _)
    {
        // Arrange
        var policies = new ILevelPolicy[]
        {
            new RookieCompetitionPolicy(), // 0-50
            new NormalCompetitionPolicy(), // 50-70
            new ProCompetitionPolicy() // 70-100
        };

        foreach (var policy in policies)
        {
            var gameManager = new GameManager(policy);
            var range = policy.GetLevelRange();

            if (level >= range.Min && level <= range.Max)
            {
                // Act
                var result = gameManager.AddPlayer("TestPlayer", level);

                // Assert
                Assert.True(result.IsSuccess);
                Assert.NotNull(result.Value);
                Assert.Single(gameManager.GetPlayers());
            }
            else
            {
                // Act
                var result = gameManager.AddPlayer("TestPlayer", level);

                // Assert
                Assert.False(result.IsSuccess);
                Assert.Contains("Player level must be between", result.Message);
                Assert.Empty(gameManager.GetPlayers());
            }
        }
    }
}