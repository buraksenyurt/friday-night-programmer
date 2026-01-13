const std = @import("std");

const Part = struct {
    id: u32,
    name: []const u8,
    stock_level: i32,
    pub fn change_stock_level(self: *Part, change: i32) void {
        self.stock_level += change;
    }
};

// //CASE02 Nesne durumunu kendi metodu ile değiştirme
pub fn main() void {
    var part = Part{
        .id = 1,
        .name = "Widget",
        .stock_level = 10,
    };

    std.debug.print("Initial stock level of {s}: {d}\n", .{ part.name, part.stock_level });
    part.change_stock_level(5);
    std.debug.print("Stock level after adding 5: {d}\n", .{part.stock_level});
}

// const Part = struct {
//     id: u32,
//     name: []const u8,
//     stock_level: i32,
// };

// // //CASE01 Geriye yeni bir nesne döndürerek state değiştirme
// pub fn main() void {
//     var part = Part{
//         .id = 1,
//         .name = "Widget",
//         .stock_level = 10,
//     };

//     std.debug.print("Initial stock level of {s}: {d}\n", .{ part.name, part.stock_level });
//     part = change_stock_level(part, 5);
//     std.debug.print("Stock level after adding 5: {d}\n", .{part.stock_level});
// }

// fn change_stock_level(part: Part, change: i32) Part {
//     const new_part = Part{
//         .id = part.id,
//         .name = part.name,
//         .stock_level = part.stock_level + change,
//     };
//     return new_part;
// }

// //CASE00 Pointer yardımıyla state değiştirme

// pub fn main() void {
//     var part = Part{
//         .id = 1,
//         .name = "Widget",
//         .stock_level = 10,
//     };

//     std.debug.print("Initial stock level of {s}: {d}\n", .{ part.name, part.stock_level });
//     change_stock_level(&part, 5);
//     std.debug.print("Stock level after adding 5: {d}\n", .{part.stock_level});
// }

// fn change_stock_level(part: *Part, change: i32) void {
//     // part.*.stock_level += change; // Aşağıdaki kullanım ile aynı
//     part.stock_level += change;
// }
