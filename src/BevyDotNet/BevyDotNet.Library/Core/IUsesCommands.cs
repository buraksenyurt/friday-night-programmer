namespace BevyDotNet.Library.Core;

/*
Şu ana kadar kullandığımız Apply metodu mutlaka bir Commands parametresi alıyor. Aslında bunu zorunlu kılmak istemiyorum.
İlk önce aşağıdaki gibi bir sözleşme tanımlayalım. Bir Commands nesnesini özellik olarak taşıyan bir sözleşme.
Eğer bir sistem bu sözleşmeyi implement ederse, Run metodu içerisinde Commands nesnesi enjekte edilmiş olacak. 
Zaten Commands sınıfı sistemler arasında paylaşılan bir nesne değil, her Run çağrısında yenileniyor.
 */
public interface IUsesCommands
{
    Commands Commands
    {
        get; set;
    }
}
