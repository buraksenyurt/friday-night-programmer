using System;

namespace RequestFormApi.Model;

public class OutboxMessage
{
    public Guid Id { get; set; }
    public string EventType { get; set; }      
    public string Payload {get;set;}
    public bool IsSent {get;set;}
    public DateTime CreateDate {get;set;}
    public DateTime? SendDate {get;set;}
}
