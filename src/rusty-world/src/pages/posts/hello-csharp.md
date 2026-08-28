---
layout: ../../layouts/MarkdownPostLayout.astro
title: Hello World in C#
pubDate: 2026-01-12
description: This is a simple "Hello World" program written in C#. It demonstrates the basic syntax and structure of a C# program.
author: Burak Selim Şenyurt
image:
    url: 'https://docs.astro.build/assets/rose.webp'
    alt: 'The Astro logo on a dark background with a pink glow.'
tags: ['csharp', 'hello world', 'programming']
---
Try to run this code in your local machine.

```csharp
using System;

class Program
{
    static void Main(string[] args)
    {
        string iconicName = "Ferris Bueller";
        string greeting = $"Hello, {iconicName}!";
        Console.WriteLine(greeting);
    }
}
```

In this example, we define a `Program` class with a `Main` method, which is the entry point of the program. We create a string variable `iconicName` and use string interpolation to create a greeting message. Finally, we print the greeting to the console using `Console.WriteLine()`.
