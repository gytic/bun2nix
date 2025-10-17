const std = @import("std");
const clap = @import("clap");
const mem = std.mem;
const path = std.path;
const wyhash = @import("./wyhash.zig").Wyhash11.hash;

const wyhash_seed: u64 = 0;

const cli_error = error{MissingOutDirFlag};

/// Tool for producing correctly named and positioned bun cache entries
///
/// Does the following:
/// - Create $out dir
/// - Calculates the correct output location for the package
/// - Symlinks the package contents to the calculated output location
/// - Creates any parent directories
///
pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    const params = comptime clap.parseParamsComptime(
        \\--help             Display this help and exit.
        \\--out <path>       The $out directory to create and write to
        \\--name <str>       The package name (and version) as found in `bun.lock`
        \\--package <path>   The contents of the package to copy
        \\
    );

    const parsers = comptime .{
        .path = clap.parsers.string,
        .str = clap.parsers.string,
    };

    var diag = clap.Diagnostic{};
    var res = clap.parse(clap.Help, &params, parsers, .{
        .diagnostic = &diag,
        .allocator = allocator,
        .assignment_separators = "=:",
    }) catch |err| {
        try diag.reportToFile(.stderr(), err);
        return err;
    };
    defer res.deinit();

    if (res.args.help != 0)
        return clap.usageToFile(.stdout(), clap.Help, &params);

    const out = res.args.out orelse
        return clap.usageToFile(.stdout(), clap.Help, &params);
    const name = res.args.name orelse
        return clap.usageToFile(.stdout(), clap.Help, &params);
    const package = res.args.package orelse
        return clap.usageToFile(.stdout(), clap.Help, &params);

    std.debug.print("out: {s}, name: {s}, package: {s}\n", .{ out, name, package });
}

/// Produce a correct bun cache folder name for a given npm identifier
///
/// Adapted from [here](https://github.com/oven-sh/bun/blob/134341d2b48168cbb86f74879bf6c1c8e24b799c/src/install/PackageManager/PackageManagerDirectories.zig#L288)
pub fn cached_npm_package_folder_print_basename(
    allocator: mem.Allocator,
    pkg: []const u8,
) ![]u8 {
    if (mem.indexOf(u8, pkg, "-")) |preIndex| {
        const name_and_ver = pkg[0..preIndex];
        const pre_and_build = pkg[preIndex + 1 ..];

        if (mem.indexOf(u8, pre_and_build, "+")) |buildIndex| {
            const pre = pre_and_build[0..buildIndex];
            const build = pre_and_build[buildIndex + 1 ..];

            return std.fmt.allocPrint(allocator, "{s}-{x}+{X}", .{
                name_and_ver,
                wyhash(wyhash_seed, pre),
                wyhash(wyhash_seed, build),
            });
        }

        return std.fmt.allocPrint(allocator, "{s}-{x}", .{
            name_and_ver,
            wyhash(wyhash_seed, pre_and_build),
        });
    }

    if (mem.indexOf(u8, pkg, "+")) |buildIndex| {
        const name_and_ver = pkg[0..buildIndex];
        const build = pkg[buildIndex + 1 ..];

        return std.fmt.allocPrint(allocator, "{s}+{X}", .{
            name_and_ver,
            wyhash(wyhash_seed, build),
        });
    }

    return allocator.dupe(u8, pkg);
}

const expectEqualSlices = std.testing.expectEqualSlices;
const testing_allocator = std.testing.allocator;

test "cached_npm_package_folder_print_basename functions" {
    const a = try cached_npm_package_folder_print_basename(
        testing_allocator,
        "react@1.2.3-beta.1+build.123",
    );
    const b = try cached_npm_package_folder_print_basename(
        testing_allocator,
        "tailwindcss@4.0.0-beta.9",
    );
    const c = try cached_npm_package_folder_print_basename(
        testing_allocator,
        "react@1.2.3+build.123",
    );
    const d = try cached_npm_package_folder_print_basename(
        testing_allocator,
        "react@1.2.3",
    );

    try expectEqualSlices(u8, "react@1.2.3-c0734e9369ab610d+F48F05ED5AABC3A0", a);
    try expectEqualSlices(u8, "tailwindcss@4.0.0-73c5c46324e78b9b", b);
    try expectEqualSlices(u8, "react@1.2.3+F48F05ED5AABC3A0", c);
    try expectEqualSlices(u8, "react@1.2.3", d);

    testing_allocator.free(a);
    testing_allocator.free(b);
    testing_allocator.free(c);
    testing_allocator.free(d);
}
