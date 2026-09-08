namespace BevyDotNet.Tester.ConsoleApp.Game;

// Bir Entity despawn edildiğinde tetiklenecek olay sınıfı.
public record struct EntityDespawnedEvent(Guid EntityId);