using Microsoft.AspNetCore.Mvc;

namespace OverloadApi.Controllers;

[ApiController]
[Route("api/[controller]")]
public class SubscriberController : ControllerBase
{
    [HttpGet("find")]
    public IActionResult Find([FromQuery] int id)
    {
        return Ok(new { Message = $"Finding subscriber with id: {id}" });
    }
    //[HttpGet("find")]
    //public IActionResult Find([FromQuery] string email)
    //{
    //    return Ok(new { Message = $"Finding subscriber with email: {email}" });
    //}
    //[HttpGet("find")]
    //public IActionResult Find([FromQuery] SocialSecurityNumber ssn)
    //{
    //    return Ok(new { Message = $"Finding subscriber with SSN: {ssn.Value}" });
    //}

    /*
        Find metodunu overload edebilsek bile, HTTP GET isteği için 
        aynı route ve farklı parametrelerle çalıştırmak mümkün değildir.
     */

    [HttpGet("find-by-email")]
    public IActionResult Find([FromQuery] string email)
    {
        return Ok(new { Message = $"Finding subscriber with email: {email}" });
    }
    [HttpGet("find-by-ssn")]
    public IActionResult Find([FromQuery] SocialSecurityNumber ssn)
    {
        return Ok(new { Message = $"Finding subscriber with SSN: {ssn.Value}" });
    }
}

public record SocialSecurityNumber(string Value)
{
    public bool IsValid()
    {
        return !string.IsNullOrEmpty(Value) && Value.Length == 11 && Value[3] == '-' && Value[6] == '-';
    }
}
