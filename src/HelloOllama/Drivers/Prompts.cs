namespace HelloOllama.Drivers;

enum PromptType
{
    Beginner,
    Advanced,
    OnlyMetrics
}
internal static class Prompts
{
    public static string GetPrompt(PromptType promptType,string codeFile)
    {
        return promptType switch
        {
            PromptType.Beginner => GetLevel100(codeFile),
            PromptType.Advanced => GetLevel200(codeFile),
            PromptType.OnlyMetrics => CalculateMetrics(codeFile),
            _ => string.Empty,
        };
    }
    static string GetLevel100(string codeFile)
    {
        return $$"""
                You are an expert in analyzing C# source code. Your task is to quickly summarize the given code file.

                ## Expected Response:
                - **Purpose**: A one-sentence description of what this code does.
                - **Main Components**: A list of important classes and methods with a short explanation.
                - **Potential Issues**: Mention one or two possible problems, if any.
    
                ## Response Format:
                ```json
                {
                    "purpose": "Short description of the code's functionality.",
                    "main_components": [
                        {
                            "name": "ClassName",
                            "type": "class",
                            "description": "Short explanation."
                        },
                        {
                            "name": "MethodName",
                            "type": "method",
                            "description": "Short explanation."
                        }
                    ],
                    "potential_issues": [
                        "Brief mention of possible issues (if any)."
                    ]
                }
                ```

                ## C# Code:
                {{File.ReadAllText(codeFile)}}
                """;
    }
    static string GetLevel200(string codeFile)
    {
        return $$"""
                You are an expert in analyzing and evaluating C# source code. You will receive a C# code file as input, and your task is to analyze it and produce a structured JSON response that includes:

                1. **Functionality Summary**: A brief description of what the code does.
                2. **Key Components**: A list of major classes, methods, and their responsibilities.
                3. **Potential Issues**: A list of possible issues such as security vulnerabilities, performance bottlenecks, or bad coding practices.
                4. **Code Quality Score**: A rating (1-10) based on readability, maintainability, and adherence to best practices.
                5. **Recommendations**: Concrete suggestions to improve the code quality.

                ## Important Notes:
                - Provide **only** a strict RFC8259 compliant JSON response.
                - Do **not** modify or infer missing parts of the code.
                - If the code is incomplete, specify it in the `"notes"` section.

                ## JSON Format Example:
                ```json
                {
                    "functionality_summary": "Brief description of what the code does.",
                    "key_components": [
                        {
                            "name": "ClassName",
                            "type": "class",
                            "description": "Purpose of this class"
                        },
                        {
                            "name": "MethodName",
                            "type": "method",
                            "description": "What this method does"
                        }
                    ],
                    "potential_issues": [
                        "List of possible security risks, performance issues, or bad practices"
                    ],
                    "code_quality_score": 8,
                    "recommendations": [
                        "Improve variable naming",
                        "Refactor long methods into smaller functions"
                    ],
                    "notes": "Additional comments if necessary"
                }
                ```

                ## C# Code:
                {{File.ReadAllText(codeFile)}}
                """;
    }

    static string CalculateMetrics(string codeFile)
    {
        return $$"""
                You are an expert in analyzing and evaluating C# source code. You will receive a C# code file as input, and your task is to analyze its complexity and structure, then provide a structured JSON response containing the following code metrics:

                ## **Code Metrics to Calculate:**
                1. **Cognitive Complexity Score** (1-10): A measure of how difficult the code is to understand (higher means more complex).
                2. **Cyclomatic Complexity**: The number of independent execution paths in the code (higher values indicate higher complexity).
                3. **Code Duplication Percentage** (0-100%): The estimated percentage of duplicated code within the file.
                4. **Maintainability Index** (0-100): A score indicating how easy the code is to maintain (higher values are better).
                5. **Code Quality Score** (1-10): A general rating based on readability, maintainability, and best practices.

                ## **Important Notes:**
                - Provide **only** a strict RFC8259 compliant JSON response.
                - Do **not** include any explanations, analysis, or extra information.
                - Do **not** modify or infer missing parts of the code.
                - If the code is incomplete, specify it in the `"notes"` section.
    
                ## Response Format:
                ```json
                {
                    "code_metrics": {
                        "cognitive_complexity_score": 6,
                        "cyclomatic_complexity": 12,
                        "code_duplication_percentage": 15,
                        "maintainability_index": 75,
                        "code_quality_score": 8
                    },
                    "notes": "The code appears complete, but further analysis is recommended."
                }
                "
                ```

                ## C# Code:
                {{File.ReadAllText(codeFile)}}
                """;
    }
}
