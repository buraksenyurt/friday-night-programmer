```

BenchmarkDotNet v0.15.8, Windows 11 (10.0.26200.8117/25H2/2025Update/HudsonValley2)
12th Gen Intel Core i7-1255U 1.70GHz, 1 CPU, 12 logical and 10 physical cores
.NET SDK 10.0.201
  [Host]     : .NET 10.0.5 (10.0.5, 10.0.526.15411), X64 RyuJIT x86-64-v3
  Job-RLSDCR : .NET 10.0.5 (10.0.5, 10.0.526.15411), X64 RyuJIT x86-64-v3

Runtime=.NET 10.0  InvocationCount=112  IterationCount=10  
WarmupCount=3  

```
| Method                                         | Culture | Key             | Mean            | Error          | StdDev         | Ratio | RatioSD | Allocated | Alloc Ratio |
|----------------------------------------------- |-------- |---------------- |----------------:|---------------:|---------------:|------:|--------:|----------:|------------:|
| &#39;MemoryCache (FrozenDictionary)&#39;               | de-DE   | button_save     |        42.68 ns |       1.394 ns |       0.922 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Hybrid ( Level1 -&gt; Level 2 -&gt; Level 3, warm)&#39; | de-DE   | button_save     |       270.24 ns |       6.048 ns |       3.599 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Redis (single key)&#39;                           | de-DE   | button_save     |   733,373.93 ns | 294,165.710 ns | 194,572.404 ns | 0.704 |    0.19 |     496 B |        0.16 |
| &#39;PostgreSQL (no cache)&#39;                        | de-DE   | button_save     | 1,051,584.71 ns | 205,259.996 ns | 107,354.947 ns | 1.009 |    0.14 |    3022 B |        1.00 |
|                                                |         |                 |                 |                |                |       |         |           |             |
| &#39;MemoryCache (FrozenDictionary)&#39;               | de-DE   | welcome_message |        52.68 ns |       6.997 ns |       4.164 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Hybrid ( Level1 -&gt; Level 2 -&gt; Level 3, warm)&#39; | de-DE   | welcome_message |       314.29 ns |      41.261 ns |      24.554 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Redis (single key)&#39;                           | de-DE   | welcome_message |   659,207.09 ns | 264,839.730 ns | 157,601.875 ns | 0.711 |    0.18 |     504 B |        0.16 |
| &#39;PostgreSQL (no cache)&#39;                        | de-DE   | welcome_message |   939,895.63 ns | 190,529.039 ns | 113,380.775 ns | 1.014 |    0.17 |    3170 B |        1.00 |
|                                                |         |                 |                 |                |                |       |         |           |             |
| &#39;MemoryCache (FrozenDictionary)&#39;               | en-US   | button_save     |        45.27 ns |       8.443 ns |       5.585 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Hybrid ( Level1 -&gt; Level 2 -&gt; Level 3, warm)&#39; | en-US   | button_save     |       221.43 ns |       6.580 ns |       3.442 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Redis (single key)&#39;                           | en-US   | button_save     |   474,298.21 ns | 112,236.560 ns |  66,790.177 ns | 0.669 |    0.11 |     496 B |        0.14 |
| &#39;PostgreSQL (no cache)&#39;                        | en-US   | button_save     |   716,641.16 ns | 125,565.800 ns |  83,054.002 ns | 1.012 |    0.15 |    3523 B |        1.00 |
|                                                |         |                 |                 |                |                |       |         |           |             |
| &#39;MemoryCache (FrozenDictionary)&#39;               | en-US   | welcome_message |        64.58 ns |      21.730 ns |      12.931 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Hybrid ( Level1 -&gt; Level 2 -&gt; Level 3, warm)&#39; | en-US   | welcome_message |       242.96 ns |      19.798 ns |      11.781 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Redis (single key)&#39;                           | en-US   | welcome_message |   611,825.98 ns | 158,720.398 ns | 104,983.716 ns | 0.540 |    0.14 |     504 B |        0.17 |
| &#39;PostgreSQL (no cache)&#39;                        | en-US   | welcome_message | 1,179,268.21 ns | 371,467.479 ns | 245,702.737 ns | 1.040 |    0.30 |    2985 B |        1.00 |
|                                                |         |                 |                 |                |                |       |         |           |             |
| &#39;MemoryCache (FrozenDictionary)&#39;               | tr-TR   | button_save     |        40.67 ns |       7.082 ns |       4.214 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Hybrid ( Level1 -&gt; Level 2 -&gt; Level 3, warm)&#39; | tr-TR   | button_save     |       290.38 ns |      59.079 ns |      35.157 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Redis (single key)&#39;                           | tr-TR   | button_save     |   618,747.50 ns | 177,637.174 ns | 117,495.992 ns | 0.733 |    0.15 |     496 B |        0.14 |
| &#39;PostgreSQL (no cache)&#39;                        | tr-TR   | button_save     |   852,865.62 ns | 129,511.326 ns |  85,663.723 ns | 1.010 |    0.14 |    3533 B |        1.00 |
|                                                |         |                 |                 |                |                |       |         |           |             |
| &#39;MemoryCache (FrozenDictionary)&#39;               | tr-TR   | welcome_message |        32.84 ns |       1.458 ns |       0.868 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Hybrid ( Level1 -&gt; Level 2 -&gt; Level 3, warm)&#39; | tr-TR   | welcome_message |       278.27 ns |       4.919 ns |       2.927 ns | 0.000 |    0.00 |         - |        0.00 |
| &#39;Redis (single key)&#39;                           | tr-TR   | welcome_message |   517,565.18 ns | 164,111.238 ns | 108,549.423 ns | 0.749 |    0.16 |     504 B |        0.14 |
| &#39;PostgreSQL (no cache)&#39;                        | tr-TR   | welcome_message |   693,429.66 ns |  66,129.158 ns |  39,352.401 ns | 1.003 |    0.08 |    3540 B |        1.00 |
