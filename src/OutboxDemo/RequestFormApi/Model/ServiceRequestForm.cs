namespace RequestFormApi.Model;

public class ServiceRequestForm
{
    public int Id { get; set; }
    public string CustomerFullName {get;set;}
    public int ServiceRepresentativeId {get;set;}
    public string Description {get;set;}
    public DateTime CreateDate {get;set;}
}
