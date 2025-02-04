using System.Net.Mail;

SendEmail("burak.selim@azon.com", "<h1>Challenge Accepted</h1><p>This is a sample e-mail from dotnet client</p>", true);
SendEmail("thereisnomail@nowhere.nowhere", "<h1>Error Test</h1><p>We are waiting for 421 error.</p>", true);

static void SendEmail(string to, string body, bool isHtml)
{
    try
    {
        var client = new SmtpClient("127.0.0.1", 2525)
        {
            DeliveryMethod = SmtpDeliveryMethod.Network
        };

        var message = new MailMessage("test.pilot@azon.com", to, "Demonstration Subject", body)
        {
            IsBodyHtml = isHtml
        };

        Console.WriteLine($"Sending email to {to}...");
        client.Send(message);
        Console.WriteLine("Email sent successfully!\n");
    }
    catch (SmtpException ex)
    {
        Console.WriteLine($"SMTP Error: {ex.StatusCode} - {ex.Message}\n");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"General Error: {ex.Message}\n");
    }
}