const std = @import("std");

// CASE 01 : Mutext ile ortak veri üzerinde thread işlemi
pub fn main() !void {
    var guard = std.Thread.Mutex{};
    var calculation_result: f64 = 0.0;

    const handle_1 = try std.Thread.spawn(
        .{},
        calcSqrt,
        .{ &calculation_result, &guard },
    );

    const handle_2 = try std.Thread.spawn(
        .{},
        calcLn,
        .{ &calculation_result, &guard },
    );

    handle_1.join();
    handle_2.join();

    std.debug.print("Calculation result {d}\n", .{calculation_result});
}

fn calcSqrt(value: *f64, guard: *std.Thread.Mutex) void {
    for (1..100) |i| {
        guard.lock();
        value.* += std.math.sqrt(@as(f64, @floatFromInt(i)));
        std.time.sleep(50 * std.time.ns_per_ms);
        guard.unlock();
    }
}

fn calcLn(value: *f64, guard: *std.Thread.Mutex) void {
    for (1..100) |i| {
        guard.lock();
        defer guard.unlock();
        value.* += std.math.log2(@as(f64, @floatFromInt(i)) + 1.0);
        std.time.sleep(50 * std.time.ns_per_ms);
    }
}

//CASE 00 : Mutext olmadan ortak veri üzerinde thread işlemi

// pub fn main() !void {
//     var calculation_result: f64 = 0.0;

//     const handle_1 = try std.Thread.spawn(
//         .{},
//         calcSqrt,
//         .{&calculation_result},
//     );

//     const handle_2 = try std.Thread.spawn(
//         .{},
//         calcLn,
//         .{&calculation_result},
//     );

//     handle_1.join();
//     handle_2.join();

//     std.debug.print("Calculation result {d}\n", .{calculation_result});
// }

// fn calcSqrt(value: *f64) void {
//     for (1..100) |i| {
//         value.* += std.math.sqrt(@as(f64, @floatFromInt(i)));
//         std.time.sleep(50 * std.time.ns_per_ms);
//     }
// }

// fn calcLn(value: *f64) void {
//     for (1..100) |i| {
//         value.* += std.math.log2(@as(f64, @floatFromInt(i)) + 1.0);
//         std.time.sleep(50 * std.time.ns_per_ms);
//     }
// }
