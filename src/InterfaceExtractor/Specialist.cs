using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Syntax;
using System.Text;

namespace InterfaceExtractor;

public static class Specialist
{
    public static void ProcessFiles(string sourceFolder, string outputFolder)
    {
        foreach (var file in Directory.GetFiles(sourceFolder, "*.cs", SearchOption.AllDirectories))
        {
            ProcessFile(file, outputFolder);
        }
    }

    static void ProcessFile(string filePath, string outputDirectory)
    {
        Console.WriteLine($"Processing: {filePath}");

        try
        {
            var syntaxTree = CSharpSyntaxTree.ParseText(File.ReadAllText(filePath));
            var semanticModel = CreateSemanticModel(syntaxTree);
            var classDeclarations = syntaxTree.GetRoot().DescendantNodes().OfType<ClassDeclarationSyntax>();

            foreach (var classDeclaration in classDeclarations)
            {
                ExtractInterface(classDeclaration, semanticModel, outputDirectory);
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error processing {filePath}: {ex.Message}");
        }
    }

    static SemanticModel CreateSemanticModel(SyntaxTree syntaxTree)
    {
        var compilation = CSharpCompilation.Create("InterfaceGenerator")
            .AddReferences(
                MetadataReference.CreateFromFile(typeof(object).Assembly.Location),
                MetadataReference.CreateFromFile(typeof(Enumerable).Assembly.Location)
            )
            .AddSyntaxTrees(syntaxTree);

        return compilation.GetSemanticModel(syntaxTree);
    }

    static string GetNamespace(ClassDeclarationSyntax classDeclaration) => classDeclaration.Ancestors().OfType<NamespaceDeclarationSyntax>().FirstOrDefault()?.Name.ToString() ??
               classDeclaration.Ancestors().OfType<FileScopedNamespaceDeclarationSyntax>().FirstOrDefault()?.Name.ToString() ??
               string.Empty;

    static void ExtractInterface(ClassDeclarationSyntax classDeclaration, SemanticModel semanticModel, string outputDirectory)
    {
        string interfaceCode = GenerateInterface(classDeclaration, semanticModel);

        if (!string.IsNullOrWhiteSpace(interfaceCode))
        {
            string namespaceName = GetNamespace(classDeclaration);
            string interfaceName = $"I{classDeclaration.Identifier.Text}";
            string outputPath = Path.Combine(outputDirectory, $"{interfaceName}.cs");

            string outputContent = !string.IsNullOrEmpty(namespaceName)
                ? $"namespace {namespaceName}\n{{\n{interfaceCode}\n}}"
                : interfaceCode;

            File.WriteAllText(outputPath, outputContent);
            Console.WriteLine($"Interface generated: {outputPath}");
        }
    }

    static string GenerateInterface(ClassDeclarationSyntax classDeclaration, SemanticModel semanticModel)
    {
        var classSymbol = semanticModel.GetDeclaredSymbol(classDeclaration);
        if (classSymbol == null)
            return string.Empty;

        string interfaceName = $"I{classDeclaration.Identifier.Text}";
        StringBuilder sb = new();
        sb.AppendLine($"public interface {interfaceName}");
        AppendTypeParameters(classDeclaration, sb);
        sb.AppendLine("{");
        AppendProperties(classDeclaration, sb);
        AppendIndexers(classDeclaration, sb);
        AppendEvents(classDeclaration, sb);
        AppendMethods(classDeclaration, sb);
        sb.AppendLine("}");

        return sb.ToString();
    }

    static void AppendTypeParameters(ClassDeclarationSyntax classDeclaration, StringBuilder sb)
    {
        if (classDeclaration.TypeParameterList != null)
        {
            sb.Append(classDeclaration.TypeParameterList.ToString());

            foreach (var constraintClause in classDeclaration.ConstraintClauses)
            {
                sb.AppendLine();
                sb.Append($"\t{constraintClause}");
            }
            sb.AppendLine();
        }
    }

    static void AppendProperties(ClassDeclarationSyntax classDeclaration, StringBuilder sb)
    {
        foreach (var property in classDeclaration.Members.OfType<PropertyDeclarationSyntax>().Where(IsPublicInstanceMember))
        {
            string accessors = string.Join(" ", property.AccessorList?.Accessors.Select(a => a.Kind() == SyntaxKind.GetAccessorDeclaration ? "get;" : "set;") ?? []);
            sb.AppendLine($"\t{property.Type} {property.Identifier.Text} {{{accessors} }}");
        }
    }

    static void AppendIndexers(ClassDeclarationSyntax classDeclaration, StringBuilder sb)
    {
        foreach (var indexer in classDeclaration.Members.OfType<IndexerDeclarationSyntax>().Where(IsPublicInstanceMember))
        {
            string accessors = string.Join(" ", indexer.AccessorList?.Accessors.Select(a => a.Kind() == SyntaxKind.GetAccessorDeclaration ? "get;" : "set;") ?? []);
            sb.AppendLine($"\t{indexer.Type} this{indexer.ParameterList} {{{accessors} }}");
        }
    }

    static void AppendEvents(ClassDeclarationSyntax classDeclaration, StringBuilder sb)
    {
        foreach (var eventDecl in classDeclaration.Members.OfType<EventDeclarationSyntax>().Where(IsPublicInstanceMember))
        {
            sb.AppendLine($"    event {eventDecl.Type} {eventDecl.Identifier};");
        }
    }

    static void AppendMethods(ClassDeclarationSyntax classDeclaration, StringBuilder sb)
    {
        foreach (var method in classDeclaration.Members.OfType<MethodDeclarationSyntax>().Where(IsValidMethod))
        {
            sb.AppendLine($"\t{method.ReturnType} {method.Identifier.Text}{method.TypeParameterList}{method.ParameterList};");
        }
    }

    static bool IsPublicInstanceMember(MemberDeclarationSyntax member) => member.Modifiers.Any(SyntaxKind.PublicKeyword) && !member.Modifiers.Any(SyntaxKind.StaticKeyword) && !member.Modifiers.Any(SyntaxKind.OverrideKeyword);

    static bool IsValidMethod(MethodDeclarationSyntax method) => IsPublicInstanceMember(method) && method.Identifier.Text is not ("ToString" or "GetHashCode" or "Equals") &&
               !method.Identifier.Text.StartsWith("get_") && !method.Identifier.Text.StartsWith("set_");
}