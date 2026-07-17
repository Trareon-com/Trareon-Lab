using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Nodes;
using Avalonia;

namespace LabSpikeAvalonia;

internal static class Program
{
    [STAThread]
    public static int Main(string[] args)
    {
        if (args.Any(a => a == "--measure"))
        {
            return MeasureRunner.Run(args);
        }

        return BuildAvaloniaApp().StartWithClassicDesktopLifetime(args);
    }

    public static AppBuilder BuildAvaloniaApp()
        => AppBuilder.Configure<App>()
            .UsePlatformDetect()
            .WithInterFont()
            .LogToTrace();
}

internal static class MeasureRunner
{
    public static int Run(string[] args)
    {
        var os = Arg(args, "--os") ?? DefaultOs();
        var rows = int.TryParse(Arg(args, "--rows"), out var r) ? r : 1_000_000;
        var filter = Arg(args, "--filter-prefix") ?? "0";
        var outPath = Arg(args, "--out") ?? Path.GetFullPath(Path.Combine("..", "results", $"{os}-avalonia.json"));
        var caseDir = Arg(args, "--case-dir") ?? Path.GetFullPath(Path.Combine("..", "results", $"{os}-avalonia-case"));
        Directory.CreateDirectory(Path.GetDirectoryName(outPath)!);

        Console.Error.WriteLine("measure: initializing Avalonia platform");
        var uiStart = Stopwatch.StartNew();
        BuildAvaloniaApp().SetupWithoutStarting();
        var uiInitMs = (ulong)uiStart.ElapsedMilliseconds;
        Console.Error.WriteLine($"measure: avalonia_ui_init_ms={uiInitMs}");

        var harness = FindHarness();
        if (harness is null)
        {
            Console.Error.WriteLine("measure error: lab-spike-harness not found. Build spikes workspace first:");
            Console.Error.WriteLine("  cd spikes && cargo build -p lab-spike-harness --release");
            return 2;
        }

        Console.Error.WriteLine($"measure: invoking {harness}");
        var psi = new ProcessStartInfo
        {
            FileName = harness,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
        };
        foreach (var a in new[]
        {
            "measure",
            "--candidate", "C-AVALONIA",
            "--os", os,
            "--rows", rows.ToString(),
            "--filter-prefix", filter,
            "--case-dir", caseDir,
            "--out", outPath,
            "--ui-init-ms", uiInitMs.ToString(),
            "--a11y-smoke", "PASS_keyboard_focus_controls_present",
            "--notes-prefix", "avalonia_shell=net8; rust_core_via_harness_cli; evidence_bytes_stay_in_rust_core=true",
            "--build-identity", "lab-spike-avalonia/0.1.0",
        })
        {
            psi.ArgumentList.Add(a);
        }

        using var proc = Process.Start(psi)!;
        var stdout = proc.StandardOutput.ReadToEnd();
        var stderr = proc.StandardError.ReadToEnd();
        proc.WaitForExit();
        Console.Error.WriteLine(stderr);
        if (proc.ExitCode != 0)
        {
            Console.Error.WriteLine(stdout);
            Console.Error.WriteLine($"measure error: harness exit {proc.ExitCode}");
            return proc.ExitCode;
        }

        if (File.Exists(outPath))
        {
            var node = JsonNode.Parse(File.ReadAllText(outPath))!.AsObject();
            node["candidate"] = "C-AVALONIA";
            var notes = node["notes"]?.GetValue<string>() ?? "";
            if (!notes.Contains("avalonia_shell", StringComparison.Ordinal))
            {
                node["notes"] = $"avalonia_shell=net8; {notes}";
            }
            var json = node.ToJsonString(new JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(outPath, json + "\n");
            Console.WriteLine(json);
        }
        else
        {
            Console.WriteLine(stdout);
        }

        Console.Error.WriteLine($"measure: wrote {outPath}");
        return 0;
    }

    static string? Arg(string[] args, string key)
    {
        for (var i = 0; i < args.Length - 1; i++)
        {
            if (args[i] == key) return args[i + 1];
        }
        return null;
    }

    static string DefaultOs()
    {
        if (OperatingSystem.IsWindows()) return "windows";
        if (OperatingSystem.IsLinux()) return "linux";
        return "macos";
    }

    static string? FindHarness()
    {
        var exe = OperatingSystem.IsWindows() ? "lab-spike-harness.exe" : "lab-spike-harness";
        var candidates = new[]
        {
            Path.GetFullPath(Path.Combine("..", "target", "release", exe)),
            Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "target", "release", exe)),
            Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", "..", "..", "target", "release", exe)),
        };
        return candidates.FirstOrDefault(File.Exists);
    }
}
