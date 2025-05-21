#!/bin/bash
set -e

DEFAULT_BASE_URL="http://192.168.0.198:8080"
IPCAM_SCRIPT="/usr/local/bin/start-ipcam.sh"
SERVICE_FILE="/etc/systemd/system/ipcam-stream.service"
PATH_FILE="/etc/systemd/system/ipcam-stream.path"
PATH_OVERRIDE_DIR="/etc/systemd/system/ipcam-stream.path.d"
PATH_OVERRIDE_CONF="$PATH_OVERRIDE_DIR/override.conf"
CONFIG_FILE="/etc/ipcam.conf"

echo "Checking and installing required packages..."
for pkg in v4l2loopback-dkms v4l2loopback-utils; do
    if dpkg -s "$pkg" &>/dev/null; then
        installed_version=$(dpkg -s "$pkg" | grep '^Version:' | awk '{print $2}')
        candidate_version=$(apt-cache policy "$pkg" | grep 'Candidate:' | awk '{print $2}')
        if [ "$installed_version" = "$candidate_version" ]; then
            echo "$pkg is already installed and up to date (version $installed_version)."
        else
            echo "$pkg installed version $installed_version, but $candidate_version available. Upgrading..."
            apt install -y "$pkg" && echo "$pkg upgrade done."
        fi
    else
        echo "$pkg is not installed. Installing..."
        apt update
        apt install -y "$pkg" && echo "$pkg installation done."
    fi
done

echo "Creating IPCam config file ($CONFIG_FILE) if not exists..."
if [ ! -f "$CONFIG_FILE" ]; then
    echo "IPCAM_URL=$DEFAULT_BASE_URL" > "$CONFIG_FILE"
    echo "Created default config at $CONFIG_FILE"
fi

echo "Creating streaming script ($IPCAM_SCRIPT)..."
cat > "$IPCAM_SCRIPT" <<'EOF'
#!/bin/bash
set -e

# Read IPCAM_URL from environment or /etc/ipcam.conf or fallback default
if [ -n "$IPCAM_URL" ]; then
    BASE_URL="$IPCAM_URL"
elif [ -f /etc/ipcam.conf ]; then
    source /etc/ipcam.conf
    if [ -z "$IPCAM_URL" ]; then
        echo "Error: /etc/ipcam.conf exists but IPCAM_URL variable not set."
        exit 1
    fi
    BASE_URL="$IPCAM_URL"
else
    BASE_URL="$DEFAULT_BASE_URL"
    echo "No IPCAM_URL found in env or /etc/ipcam.conf. Using default: $BASE_URL"
fi

echo "Reloading v4l2loopback module..."
sudo modprobe -r v4l2loopback || true
sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="IPCam" exclusive_caps=1

echo "Starting ffmpeg stream from $BASE_URL/video to /dev/video10"
ffmpeg -i "${BASE_URL}/video" -vcodec rawvideo -pix_fmt yuv420p -f v4l2 /dev/video10
EOF

chmod +x "$IPCAM_SCRIPT"

echo "Creating systemd service ($SERVICE_FILE)..."
cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=Android IP Camera to V4L2 Virtual Device
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=$IPCAM_SCRIPT
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

echo "Creating systemd path unit ($PATH_FILE)..."
cat > "$PATH_FILE" <<EOF
[Unit]
Description=Watch /etc/ipcam.conf and restart ipcam-stream.service on changes

[Path]
PathChanged=$CONFIG_FILE

[Install]
WantedBy=multi-user.target
EOF

echo "Creating systemd path override to trigger service restart ($PATH_OVERRIDE_CONF)..."
mkdir -p "$PATH_OVERRIDE_DIR"
cat > "$PATH_OVERRIDE_CONF" <<EOF
[Path]
Unit=ipcam-stream.service
EOF

echo "Reloading systemd daemon and enabling units..."
systemctl daemon-reload
systemctl enable ipcam-stream.service
systemctl enable ipcam-stream.path

echo "Starting ipcam-stream service and path watcher..."
systemctl start ipcam-stream.service
systemctl start ipcam-stream.path

echo "Setup complete! Service is running and will auto-restart on $CONFIG_FILE changes."


#--------------------------------------------------------------Complete Cleanup Scrpt--------------------------------------------------------------
#!/bin/bash
set -e

# SERVICE="ipcam-stream.service"
# PATHUNIT="ipcam-stream.path"
# PATH_OVERRIDE_DIR="/etc/systemd/system/ipcam-stream.path.d"
# PATH_OVERRIDE_CONF="$PATH_OVERRIDE_DIR/override.conf"
# IPCAM_SCRIPT="/usr/local/bin/start-ipcam.sh"
# CONFIG_FILE="/etc/ipcam.conf"

# echo "Stopping service and path watcher..."
# sudo systemctl stop $SERVICE $PATHUNIT || true

# echo "Disabling service and path watcher..."
# sudo systemctl disable $SERVICE $PATHUNIT || true

# echo "Removing systemd units and override..."
# sudo rm -f /etc/systemd/system/$SERVICE
# sudo rm -f /etc/systemd/system/$PATHUNIT
# sudo rm -rf "$PATH_OVERRIDE_DIR"

# echo "Reloading systemd daemon..."
# sudo systemctl daemon-reload

# echo "Removing streaming script..."
# sudo rm -f "$IPCAM_SCRIPT"

# echo "Removing config file (optional, comment out if you want to keep)..."
# sudo rm -f "$CONFIG_FILE"

# echo "Uninstalling packages (optional, comment out if you want to keep)..."
# sudo apt remove -y v4l2loopback-dkms v4l2loopback-utils

# echo "Cleanup complete."

