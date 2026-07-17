using Avalonia.Controls;

namespace LabSpikeAvalonia;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        OpenBtn.Click += (_, _) =>
            Status.Text = "Interactive open uses --measure for Gate A timing. Shell focus/keyboard controls are present.";
        FilterBtn.Click += (_, _) =>
            Status.Text = $"Filter UI ready (prefix='{FilterBox.Text}'). Equal workflow measured via Rust harness.";
        HashBtn.Click += (_, _) => Status.Text = "Hash start control present (Rust owns job in measure mode).";
        CancelBtn.Click += (_, _) => Status.Text = "Cancel control present.";
        CrashBtn.Click += (_, _) => Status.Text = "Crash-worker control present.";
        ExportBtn.Click += (_, _) => Status.Text = "Export control present.";
        Rows.ItemsSource = new[]
        {
            "0  synthetic://preview  (UI shell only — open --measure for 1M-row core workflow)",
        };
        Detail.Text = "Detail pane present for G6 smoke (keyboard-focusable controls in toolbar).";
    }
}
