const std = @import("std");

const Part = struct {
    id: u32,
    name: []const u8,
    stock_level: i32,
};

pub fn main() void {
    var part = Part{
        .id = 1,
        .name = "Widget",
        .stock_level = 10,
    };

    std.debug.print("Initial stock level of {s}: {d}\n", .{ part.name, part.stock_level });
    change_stock_level(&part, 5);
    std.debug.print("Stock level after adding 5: {d}\n", .{part.stock_level});
}

fn change_stock_level(part: *Part, change: i32) void {
    // part.*.stock_level += change; // Aşağıdaki kullanım ile aynı
    part.stock_level += change;
}
