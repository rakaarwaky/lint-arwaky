"""Interactive TUI monitor for Graph Loop Pipeline (rich-based)."""
from __future__ import annotations

import json
import os
import shutil
import signal
import sys
import time
from datetime import datetime
from pathlib import Path
from typing import Optional

from rich.console import Console
from rich.layout import Layout
from rich.live import Live
from rich.panel import Panel
from rich.table import Table
from rich.text import Text

SCRIPT_DIR = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(SCRIPT_DIR))

from src.config import Config

console = Console()

# ── State icons ────────────────────────────────────────────────
ICON = {
    "completed": "[green]\u2705[/green]",
    "running": "[yellow]\U0001f504[/yellow]",
    "pending": "[dim]\u23f3[/dim]",
    "idle": "[red]\u274c[/red]",
}

STATE_COLOR = {
    "IDLE": "dim",
    "DISPATCHING": "yellow",
    "ANALYZING": "yellow",
    "ARCHITECT": "cyan",
    "DEVELOPER": "blue",
    "QUALITY-ANALYSIS": "magenta",
    "MERGED": "green",
    "FAILED": "red",
    "BLOCKED": "red bold",
    "WAITING_HUMAN": "red underline",
    "ESCALATED": "red bold underline",
}

FEAT_COLOR = {
    "PENDING": "yellow",
    "LOCKED": "blue",
    "ACTIVE": "green",
    "DONE": "green dim",
    "FAILED": "red",
    "BLOCKED": "red",
    "MERGED": "green dim",
}


def _load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except (json.JSONDecodeError, FileNotFoundError):
        return {}


def _elapsed_str(started_at: Optional[str]) -> str:
    if not started_at:
        return "-"
    try:
        dt = datetime.fromisoformat(started_at)
        diff = datetime.now().astimezone() - dt
        mins = int(diff.total_seconds() // 60)
        secs = int(diff.total_seconds() % 60)
        if mins >= 60:
            return f"{mins // 60}h {mins % 60}m"
        return f"{mins}m {secs}s"
    except (ValueError, TypeError):
        return "-"


def _tail_log(log_path: Path, n: int = 12) -> list[str]:
    if not log_path.exists():
        return ["(no log file)"]
    lines = log_path.read_text().splitlines()
    return lines[-n:] if lines else ["(empty log)"]


# ── Panel builders ─────────────────────────────────────────────
def build_pipeline_panel(state: dict) -> Panel:
    p = state.get("pipeline", {})
    current = p.get("current_state", "IDLE")
    color = STATE_COLOR.get(current, "white")

    tbl = Table(show_header=False, box=None, padding=(0, 2))
    tbl.add_column("key", style="bold")
    tbl.add_column("value")
    tbl.add_row("State", f"[{color}]{current}[/{color}]")
    tbl.add_row("Feature", p.get("feature") or "[dim]none[/dim]")
    tbl.add_row("Pipeline", p.get("id") or "[dim]none[/dim]")
    tbl.add_row("Correlation", p.get("correlation_id") or "[dim]none[/dim]")
    tbl.add_row("Elapsed", _elapsed_str(p.get("started_at")))
    rej = p.get("rejection_loop_counter", 0)
    it = p.get("pipeline_iteration_counter", 0)
    rej_style = "red" if rej >= 3 else "yellow" if rej > 0 else "green"
    it_style = "red" if it >= 5 else "yellow" if it > 0 else "green"
    tbl.add_row("Rejection", f"[{rej_style}]{rej}/3[/{rej_style}]")
    tbl.add_row("Iteration", f"[{it_style}]{it}/5[/{it_style}]")

    if p.get("failure"):
        f = p["failure"]
        tbl.add_row("Failure", f"[red]{f.get('reason', '?')}[/red]")
    if p.get("escalation"):
        e = p["escalation"]
        tbl.add_row("Escalation", f"[red bold]{e.get('reason', '?')}[/red bold]")

    return Panel(tbl, title="[bold]Pipeline Status[/bold]", border_style="cyan")


def build_nodes_panel(state: dict) -> Panel:
    p = state.get("pipeline", {})
    nodes = p.get("parallel_nodes", {})
    current = p.get("current_state", "IDLE")

    tbl = Table(show_header=True, box=None, padding=(0, 1))
    tbl.add_column("Node", style="bold", width=18)
    tbl.add_column("Status", width=14)
    tbl.add_column("Duration", width=12)

    for name, label in [("business-analyst", "Business-Analyst"),
                         ("tech-lead", "Tech-Lead")]:
        n = nodes.get(name, {})
        st = n.get("status", "idle")
        icon = ICON.get(st, st)
        dur = "-"
        if n.get("started_at"):
            if n.get("completed_at"):
                try:
                    s = datetime.fromisoformat(n["started_at"])
                    e = datetime.fromisoformat(n["completed_at"])
                    diff = e - s
                    dur = f"{int(diff.total_seconds() // 60)}m {int(diff.total_seconds() % 60)}s"
                except (ValueError, TypeError):
                    dur = _elapsed_str(n["started_at"])
            else:
                dur = _elapsed_str(n["started_at"]) + "..."
        tbl.add_row(label, f"{icon} {st}", dur)

    order = ["ARCHITECT", "DEVELOPER", "QUALITY-ANALYSIS"]
    idx = order.index(current) if current in order else -1
    for i, label in enumerate(("Architect", "Developer", "Quality-Analysis")):
        if current == "MERGED":
            st_text = f"[green]\u2705 DONE[/green]"
            dur = "-"
        elif idx == i:
            st_text = f"[yellow]\U0001f504 ACTIVE[/yellow]"
            dur = _elapsed_str(p.get("started_at")) + "..."
        elif idx > i:
            st_text = f"[green]\u2705 DONE[/green]"
            dur = "-"
        else:
            st_text = "[dim]\u23f3 WAITING[/dim]"
            dur = "-"
        tbl.add_row(label, st_text, dur)

    return Panel(tbl, title="[bold]Node Status[/bold]", border_style="blue")


def build_features_panel(features: dict) -> Panel:
    feats = features.get("features", {})
    total = len(feats)

    # status counts
    counts: dict[str, int] = {}
    for f in feats.values():
        s = f.get("status", "UNKNOWN")
        counts[s] = counts.get(s, 0) + 1

    tbl = Table(show_header=False, box=None, padding=(0, 2))
    tbl.add_column("status", style="bold")
    tbl.add_column("count")
    for status in ("PENDING", "LOCKED", "ACTIVE", "DONE", "FAILED", "BLOCKED", "MERGED"):
        c = counts.get(status, 0)
        if c > 0:
            color = FEAT_COLOR.get(status, "white")
            tbl.add_row(f"[{color}]{status}[/{color}]", str(c))
    tbl.add_row("[bold]Total[/bold]", str(total))

    # progress bar
    done = counts.get("DONE", 0) + counts.get("MERGED", 0)
    if total > 0:
        pct = done * 100 // total
        bar_width = 30
        filled = bar_width * done // total
        empty = bar_width - filled
        bar = f"[green]{'█' * filled}[/green][dim]{'░' * empty}[/dim] {done}/{total} ({pct}%)"
    else:
        bar = "[dim]no features[/dim]"

    # active feature detail
    detail = ""
    for name, f in feats.items():
        if f.get("status") in ("LOCKED", "ACTIVE"):
            detail += f"\n  [bold]{name}[/bold] — {f.get('status')} (pipeline: {f.get('pipeline_id', '?')})"

    content = Text.from_markup(bar) if "[" in bar else bar
    tbl.add_row("", "")
    tbl.add_row("[bold]Progress[/bold]", bar)
    if detail:
        tbl.add_row("[bold]Active[/bold]", detail.strip())

    return Panel(tbl, title="[bold]Feature Queue[/bold]", border_style="green")


def build_activity_panel(log_path: Path) -> Panel:
    lines = _tail_log(log_path, 10)
    tbl = Table(show_header=False, box=None, padding=(0, 1), expand=True)
    tbl.add_column("log", overflow="fold")
    for line in lines:
        # color-code by component
        if "[ERROR" in line or "error" in line.lower():
            tbl.add_row(f"[red]{line}[/red]")
        elif "WARN" in line or "warn" in line.lower():
            tbl.add_row(f"[yellow]{line}[/yellow]")
        elif "transition" in line.lower() or "completed" in line.lower():
            tbl.add_row(f"[green]{line}[/green]")
        elif "spawn" in line.lower() or "dispatch" in line.lower():
            tbl.add_row(f"[cyan]{line}[/cyan]")
        else:
            tbl.add_row(line)
    return Panel(tbl, title="[bold]Recent Activity[/bold]", border_style="yellow")


def build_health_panel(config: Config) -> Panel:
    checks: list[tuple[str, str, str]] = []

    # engine process
    pid_file = SCRIPT_DIR / "engine.pid"
    engine_ok = False
    if pid_file.exists():
        try:
            pid = int(pid_file.read_text().strip())
            os.kill(pid, 0)
            checks.append(("Engine", "RUNNING", f"PID {pid}"))
            engine_ok = True
        except (ValueError, ProcessLookupError, PermissionError, OSError):
            checks.append(("Engine", "DEAD", "stale PID"))
    else:
        try:
            import subprocess
            r = subprocess.run(["systemctl", "is-active", "graph-loop.service"],
                               capture_output=True, text=True, timeout=5)
            if r.stdout.strip() == "active":
                checks.append(("Engine", "RUNNING", "systemd"))
                engine_ok = True
            else:
                checks.append(("Engine", "STOPPED", "no PID, no service"))
        except Exception:
            checks.append(("Engine", "UNKNOWN", ""))

    # agent processes
    agent_count = 0
    try:
        import subprocess
        r = subprocess.run(["pgrep", "-f", "qwen -p"], capture_output=True, text=True, timeout=5)
        agent_count = len([l for l in r.stdout.splitlines() if l.strip()])
    except Exception:
        pass
    checks.append(("Agents", f"{agent_count} active" if agent_count else "none", ""))

    # disk
    try:
        usage = shutil.disk_usage(str(config.project_root))
        pct = usage.used * 100 // usage.total
        disk_label = f"{pct}%"
        if pct >= 90:
            disk_label = f"[red]{pct}%[/red]"
        elif pct >= 80:
            disk_label = f"[yellow]{pct}%[/yellow]"
        checks.append(("Disk", disk_label, ""))
    except Exception:
        checks.append(("Disk", "?", ""))

    # log size
    log = config.log_file
    if log.exists():
        kb = log.stat().st_size // 1024
        log_label = f"{kb}KB"
        if kb >= 10240:
            log_label = f"[yellow]{kb}KB[/yellow]"
        checks.append(("Log", log_label, ""))
    else:
        checks.append(("Log", "none", ""))

    tbl = Table(show_header=False, box=None, padding=(0, 2))
    tbl.add_column("name", style="bold", width=10)
    tbl.add_column("value", width=16)
    tbl.add_column("detail", width=20)
    for name, val, detail in checks:
        tbl.add_row(name, val, f"[dim]{detail}[/dim]" if detail else "")

    return Panel(tbl, title="[bold]Health[/bold]", border_style="magenta")


# ── Main TUI loop ──────────────────────────────────────────────
def run_tui(refresh_seconds: int = 5) -> None:
    config = Config()

    def build_layout() -> Layout:
        layout = Layout()
        layout.split_column(
            Layout(name="top", ratio=3),
            Layout(name="mid", ratio=3),
            Layout(name="bottom", ratio=4),
        )
        layout["top"].split_row(
            Layout(name="pipeline", ratio=2),
            Layout(name="nodes", ratio=3),
        )
        layout["mid"].split_row(
            Layout(name="features", ratio=2),
            Layout(name="activity", ratio=3),
        )
        layout["bottom"].split_row(
            Layout(name="health", ratio=1),
        )
        return layout

    def render() -> Layout:
        layout = build_layout()
        state = _load_json(config.state_file)
        features = _load_json(config.features_file)

        layout["pipeline"].update(build_pipeline_panel(state))
        layout["nodes"].update(build_nodes_panel(state))
        layout["features"].update(build_features_panel(features))
        layout["activity"].update(build_activity_panel(config.log_file))
        layout["health"].update(build_health_panel(config))
        return layout

    # handle resize
    def on_resize(signum, frame):
        pass  # rich handles resize internally

    signal.signal(signal.SIGWINCH, on_resize)

    with Live(render(), console=console, refresh_per_second=1,
              screen=True, transient=False) as live:
        try:
            while True:
                time.sleep(refresh_seconds)
                live.update(render())
        except KeyboardInterrupt:
            pass


def cli(argv: list[str]) -> int:
    refresh = 5
    if argv and argv[0] in ("-r", "--refresh"):
        if len(argv) > 1:
            try:
                refresh = max(2, int(argv[1]))
            except ValueError:
                pass
    run_tui(refresh_seconds=refresh)
    return 0
