namespace Fnp.Works;

public class CustomerBusiness : BusinessBase
{
    public bool OnTestMode {get; set;}
    public bool AddBonus(int customerId, double amount)
    {
        return true;
    }
    public Customer GetCustomer(int customerId){
        return null;
    }
}