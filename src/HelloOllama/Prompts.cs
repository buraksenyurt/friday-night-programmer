namespace HelloOllama;

internal static class Prompts
{
    public static string GetLevel100(string codeFile)
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
    public static string GetLevel200(string codeFile)
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
}
