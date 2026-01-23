const std = @import("std");

pub fn main() !void {
    var calculation_result: f64 = 0.0;

    const handle_1 = try std.Thread.spawn(
        .{},
        calcSqrt,
        .{&calculation_result},
    );

    const handle_2 = try std.Thread.spawn(
        .{},
        calcLn,
        .{&calculation_result},
    );

    handle_1.join();
    handle_2.join();

    std.debug.print("Calculation result {d}\n", .{calculation_result});
}

fn calcSqrt(value: *f64) void {
    for (1..100) |i| {
        value.* += std.math.sqrt(@as(f64, @floatFromInt(i)));
        std.time.sleep(50 * std.time.ns_per_ms);
    }
}

fn calcLn(value: *f64) void {
    for (1..100) |i| {
        value.* += std.math.log2(@as(f64, @floatFromInt(i)) + 1.0);
        std.time.sleep(50 * std.time.ns_per_ms);
    }
}
