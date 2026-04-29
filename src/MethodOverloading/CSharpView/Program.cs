Console.WriteLine("Method overloading demonstration");

var finder = new SubscriberFilter();
_ = finder.Find(123);
_ = finder.Find("john.doe@azon.none");
_ = finder.Find(new SocialSecurityNumber("123-45-678"));

public record SocialSecurityNumber(string Value)
{
    public bool IsValid()
    {
        return !string.IsNullOrEmpty(Value) && Value.Length == 11 && Value[3] == '-' && Value[6] == '-';
    }
}
public class Subscriber
{
    public Guid Id { get; set; }
    public string Email { get; set; }
    public SocialSecurityNumber Ssn { get; set; }
}
public class SubscriberFilter
{
    public Subscriber? Find(int id)
    {
        Console.Write("Finding subscriber with id: " + id);
        return new Subscriber();
    }
    public Subscriber? Find(string email)
    {
        Console.Write("Finding subscriber with email: " + email);
        return new Subscriber();
    }
    public Subscriber? Find(SocialSecurityNumber ssn)
    {
        Console.Write("Finding subscriber with SSN: " + ssn.Value);
        return new Subscriber();
    }
}
