#!/usr/bin/env python3

"""
Python 3 script to build an AppImage for the Calcurus application.
This is a translation of the provided bash script.
"""

import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path


def main():
    if platform.system() == "Linux":
        build_appimage()
    else:
        print("Error: This script is designed to run only on Linux.", file=sys.stderr)
        print(f"Detected OS: {platform.system()}")
        sys.exit(1)


def run_cmd(command_list, env=None):
    """
    Helper function to run an external command, print it,
    and exit if it fails.
    """
    # Use os.fspath to handle Path objects gracefully
    cmd_str_list = [os.fspath(item) for item in command_list]
    print(f"--- Running: {' '.join(cmd_str_list)}")
    try:
        _ = subprocess.run(cmd_str_list, check=True, env=env)
    except (subprocess.CalledProcessError, FileNotFoundError) as e:
        print(f"Error executing command: {e}", file=sys.stderr)
        sys.exit(1)


def build_appimage():
    # --- 1. Architecture Check ---
    arch = platform.machine()
    if arch == "x86_64":
        tool_arch = "x86_64"
    elif arch == "aarch64":
        tool_arch = "aarch64"
    else:
        print(f"Unsupported architecture: {arch}", file=sys.stderr)
        sys.exit(1)

    appdir = Path("AppDir")

    # --- 2. Create AppDir Structure ---
    print("Creating AppDir structure...")
    dirs_to_create = [
        appdir / "usr/bin",
        appdir / "usr/share/applications",
        appdir / "usr/share/icons/hicolor/4000x4000/apps",
        appdir / "usr/share/icons/hicolor/scalable/apps",
    ]

    for d in dirs_to_create:
        d.mkdir(parents=True, exist_ok=True)

    # --- 3. Copy Files ---
    print("Copying files...")

    # Define file mappings (source: destination)
    files_to_copy = [
        ("./target/release/Calcurus", appdir / "usr/bin/"),
        ("./resources/AppRun", appdir / "AppRun"),
        ("./resources/calcurus.desktop", appdir / "calcurus.desktop"),
        ("./resources/calcurus.desktop", appdir / "usr/share/applications/"),
        (
            "./resources/icons/hicolor/4000x4000/apps/calcurus.png",
            appdir / "calcurus.png",
        ),
        (
            "./resources/icons/hicolor/4000x4000/apps/calcurus.png",
            appdir / "usr/share/icons/hicolor/4000x4000/apps/calcurus.png",
        ),
        (
            "./resources/icons/hicolor/scalable/apps/calcurus.svg",
            appdir / "usr/share/icons/hicolor/scalable/apps/calcurus.svg",
        ),
    ]

    try:
        for src, dst in files_to_copy:
            _ = shutil.copy(src, dst)
    except FileNotFoundError as e:
        print(f"Error copying file: {e}", file=sys.stderr)
        print("Please ensure all source files exist.")
        sys.exit(1)
    except shutil.SameFileError:
        # This can happen if the script is run in a weird state
        pass

    # --- 4. Set Permissions ---
    files_to_make_executable = [appdir / "AppRun", appdir / "usr/bin/Calcurus"]

    for f in files_to_make_executable:
        f.chmod(0o755)  # Equivalent to chmod +x

    # --- 5. Download appimagetool ---
    tool_filename = f"appimagetool-{tool_arch}.AppImage"
    tool_path = Path(tool_filename)
    tool_url = f"https://github.com/AppImage/AppImageKit/releases/download/continuous/{tool_filename}"

    if not tool_path.is_file():
        print(f"Downloading {tool_filename}...")
        run_cmd(["wget", "-q", tool_url])
        tool_path.chmod(0o755)
    else:
        print(f"{tool_filename} already exists, skipping download.")

    # --- 6. Build AppImage ---
    print("Building AppImage...")

    # Create a copy of the current environment and add the ARCH variable
    build_env = os.environ.copy()
    build_env["ARCH"] = tool_arch

    run_cmd([f"./{tool_filename}", appdir, "Calcurus.AppImage"], env=build_env)

    # --- 7. Cleanup ---
    print(f"Cleaning up {appdir}...")
    try:
        shutil.rmtree(appdir)
    except OSError as e:
        print(f"Error removing {appdir}: {e}", file=sys.stderr)

    print("\nAppImage build complete!")


if __name__ == "__main__":
    main()
