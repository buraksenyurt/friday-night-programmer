using Microsoft.SemanticKernel;
using System.ComponentModel;

namespace DmsAIAssistant.Plugins;

internal class VehicleServicePlugin
{
    private record VehicleInfo(string Model, string Status, string Note);
    private readonly Dictionary<string, VehicleInfo> _database = new()
    {
        {"34ABC123",new VehicleInfo("Lamborcini Gayyardo","Repairing","Tire changes") },
        {"34DEF456",new VehicleInfo("Ferrari F40","Ready","") },
        {"34GHI789",new VehicleInfo("Porsche 911","Repairing","Engine issue") },
    };

    [KernelFunction]
    [Description("Get the status of a vehicle by its license plate.")]
    public string GetVehicleStatus([Description("The plate number of the vehicle")] string licensePlate)
    {
        var trimmedPlate = licensePlate.Replace(" ", "").ToUpper();
        if (_database.TryGetValue(trimmedPlate, out var info))
        {
            return $"Model: {info.Model}, Status: {info.Status}, Note: {info.Note}";
        }
        return "Vehicle not found.";
    }

    public string CreateServiceAppointment(
        [Description("The plate number of the vehicle")] string licensePlate
        , [Description("Appointment date (Day/Month)")] string date
        , [Description("The reason")] string reason)
    {
        if (_database.TryGetValue(licensePlate, out var info))
        {
            return $"Service appointment created for {info.Model} on {date} for reason: {reason}.";
        }
        return "Vehicle not found. Cannot create appointment.";
    }
}
