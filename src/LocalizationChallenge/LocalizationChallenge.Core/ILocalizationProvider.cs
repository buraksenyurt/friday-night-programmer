namespace LocalizationChallenge.Core;

public interface ILocalizationProvider
{
    string ProviderName { get; }
    ValueTask<string> GetLocalizedStringAsync(string key, string culture, CancellationToken cancellationToken = default);
}
