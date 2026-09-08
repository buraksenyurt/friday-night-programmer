namespace BevyDotNet.Library;

/*
IUsesCommands ile aynı mantık. EventBus'ı kullanmak isteyen sistemler kendilerine EventBus özelliğini
enjekte etmeliler. Bunu bildirdiğimiz sözleşme arayüzü(contract).
 */
public interface IUsesEventBus
{
    EventBus EventBus
    {
        get; set;
    }
}
