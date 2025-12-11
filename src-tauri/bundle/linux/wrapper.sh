#!/bin/bash
# Wrapper script for win11-clipboard-history
# Cleans environment to avoid Snap library conflicts

# Binary location
BINARY="/usr/lib/win11-clipboard-history/win11-clipboard-history-bin"

# Check for Snap-polluted environment (VS Code Snap, etc.)
# Look for any snap-related paths in critical environment variables
if [[ -n "$SNAP" ]] || \
   [[ "$LD_LIBRARY_PATH" == */snap/* ]] || \
   [[ "$GTK_PATH" == */snap/* ]] || \
   [[ "$GIO_MODULE_DIR" == */snap/* ]] || \
   [[ -n "$GTK_EXE_PREFIX" && "$GTK_EXE_PREFIX" == */snap/* ]]; then
    
    # Run with clean environment, preserving only essential variables
    exec env -i \
        HOME="$HOME" \
        USER="$USER" \
        SHELL="$SHELL" \
        TERM="$TERM" \
        DISPLAY="${DISPLAY:-:0}" \
        XAUTHORITY="$XAUTHORITY" \
        WAYLAND_DISPLAY="$WAYLAND_DISPLAY" \
        XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" \
        XDG_SESSION_TYPE="$XDG_SESSION_TYPE" \
        XDG_CURRENT_DESKTOP="$XDG_CURRENT_DESKTOP" \
        XDG_SESSION_CLASS="$XDG_SESSION_CLASS" \
        DBUS_SESSION_BUS_ADDRESS="$DBUS_SESSION_BUS_ADDRESS" \
        PATH="/usr/local/bin:/usr/bin:/bin" \
        LANG="${LANG:-en_US.UTF-8}" \
        LC_ALL="${LC_ALL:-}" \
        "$BINARY" "$@"
else
    exec "$BINARY" "$@"
fi
