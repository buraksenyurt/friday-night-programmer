namespace InterfaceExtractor;

class Program
{
    static void Main(string[] args)
    {
        if (!ValidateArguments(args, out string sourceFolder, out string outputFolder))
            return;

        Specialist.ProcessFiles(sourceFolder, outputFolder);
    }

    static bool ValidateArguments(string[] args, out string sourceFolder, out string outputFolder)
    {
        sourceFolder = outputFolder = string.Empty;

        if (args.Length != 2)
        {
            Console.WriteLine("Argument error");
            return false;
        }

        sourceFolder = args[0];
        outputFolder = args[1];

        if (!Directory.Exists(sourceFolder) || !Directory.Exists(outputFolder))
        {
            Console.WriteLine("Directory error");
            return false;
        }

        return true;
    }
}