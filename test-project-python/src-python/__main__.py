# AES022: surface role violation — entry point with business logic
# AES009: no class
# AES014: noqa, pass
import sys  # noqa: F401

def main():
    args = sys.argv[1:]
    if not args:
        print("Usage: arwaky <command> [options]")
        return
    cmd = args[0]
    if cmd == "check":
        from surfaces.cli import main as cli_main  # type: ignore
        cli_main()
    elif cmd == "scan":
        path = args[1] if len(args) > 1 else "."
        print(f"Scanning {path}...")
        pass
    elif cmd == "help":
        print("Available: check, scan, help")
    else:
        print(f"Unknown: {cmd}")

if __name__ == "__main__":
    main()
