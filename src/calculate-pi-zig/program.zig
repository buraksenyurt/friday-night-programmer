const std = @import("std");

// Monte Carlo yöntemiyle Pi değeri hesaplanan fonksiyon
// ilk parametre iterasyon sayısını alır ki bizim örneğimizde 1 milyar / işlemci-çekirdek sayısı kadardır.
// İkinci parametre pointer olarak gelir ve in_circle değişkenine atomik olarak ekleme yapar.
// Atomik seviyede ekleme yapmak hızlıdır çünkü doğrudan işlemci instruction'larıyla yapılır ve kilitlenmeye gerek kalmaz.
fn monteCarloCalculation(iterations: usize, result: *usize) void {
    var seed: u64 = undefined;
    std.crypto.random.bytes(std.mem.asBytes(&seed)); // Rastgele bir seed oluşturuyoruz, böylece her çalıştırmada farklı sonuçlar elde edebiliriz.

    // Xoshiro256 türünden bir PRNG-Pseudo Random Number Generator- başlatıyoruz
    // Bu epey hızlı çalışır
    var rng = std.Random.DefaultPrng.init(seed);
    const random = rng.random();

    var localCount: usize = 0;
    var i: usize = 0;

    // normal iterasyon döngümüz
    while (i < iterations) : (i += 1) {
        // f64 yerine f32 kullanarak işlemci üzerindeki yükü yarı yarıya düşürebiliriz.
        const x = random.float(f32);
        const y = random.float(f32);
        // çember içinde olup olmadığını kontrol ediyoruz
        if (x * x + y * y <= 1.0) {
            localCount += 1;
        }
    }

    // İşlem bitince tek bir atomik yazma işlemi
    // İlk parametre veri türü, ikinci parametre hedef değişken,
    // üçüncü parametre çağırılacak instruction komutu,
    // dördüncü parametre eklenecek değer
    // ve en nihayetinde beşinci parametre bellek sıralama türü
    _ = @atomicRmw(usize, result, .Add, localCount, .monotonic);
}

pub fn main() !void {
    const totalIterations: usize = 1_000_000_000;
    const threadCount = try std.Thread.getCpuCount(); // işlemci çekirdek sayısını alıyoruz

    const iterPerThread = totalIterations / threadCount; // thread başına iterasyon değeri

    // Allocator'sız olmaz :D GPA nispeten daha iyi performans gösterir
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(); // tedbir amaçlı deinit çağırıyoruz, böylece program sonunda kaynaklar düzgün şekilde serbest bırakılır
    const allocator = gpa.allocator();

    // thread'ler için bellek ayırıyoruz, her thread monteCarloCalculation fonksiyonunu çalıştıracak
    const threads = try allocator.alloc(std.Thread, threadCount);
    defer allocator.free(threads); // Yine tedbir amaçlı thread'ler için ayrılan belleği serbest bırakıyoruz

    // Standart deney döngümüz. 10 kez çalıştırılacak
    for (0..10) |_| {
        var in_circle: usize = 0;

        const start = std.time.nanoTimestamp();

        // Her test adımında thread'ler oluşturup, monteCarloCalculation fonksiyonunu çalıştırıyoruz
        for (threads) |*thread| {
            thread.* = try std.Thread.spawn(
                .{},
                monteCarloCalculation,
                .{ iterPerThread, &in_circle },
            );
        }

        // Burada thread'lerin bitmesini bekliyoruz,
        // her thread'in join edilmesi gerekiyor ki sonuçlar doğru şekilde toplanabilsin
        // Burası aynı zamanda en son çalışma zamanının neden yüksek çıktığının bir sebebi olabilir
        for (threads) |thread| {
            thread.join();
        }

        const elapsed_ns = std.time.nanoTimestamp() - start;
        const elapsed_ms = @divTrunc(elapsed_ns, std.time.ns_per_ms);

        const pi = 4.0 * @as(f64, @floatFromInt(in_circle)) / @as(f64, @floatFromInt(totalIterations));
        std.debug.print("Pi = {d:.6}  ({d} ms)\n", .{ pi, elapsed_ms });
    }
}
