const std = @import("std");

const Subscriber = struct {};

const Id = struct { value: i32 };
const Email = struct { value: []const u8 };
const Ssn = struct { value: []const u8 };
const Uuid = struct { value: []const u8 };

const SubscriberFoundation = struct {
    pub fn find(self: *const SubscriberFoundation, search_param: anytype) ?Subscriber {
        _ = self;

        switch (comptime @TypeOf(search_param)) {
            Id => {
                std.debug.print("search by id: {d}\n", .{search_param.value});
            },
            Email => {
                std.debug.print("search by email: {s}\n", .{search_param.value});
            },
            Ssn => {
                std.debug.print("search by ssn: {s}\n", .{search_param.value});
            },
            Uuid => {
                std.debug.print("search by unique nick name: {s}\n", .{search_param.value});
            },
            else => {
                @compileError("Unsupported search type for Subscriber");
            },
        }

        return null;
    }
};

pub fn main() void {
    const foundation = SubscriberFoundation{};

    _ = foundation.find(Id{ .value = 1195 });
    _ = foundation.find(Email{ .value = "bss@none" });
    _ = foundation.find(Uuid{ .value = "550e8400-e29b-41d4-a716-446655440000" });
    _ = foundation.find(Ssn{ .value = "123-45-6789" });

    // Aşağıdaki şekilde kullanamayız. Tipin belli olması gerekir.
    // _ = foundation.find(1195);
}
