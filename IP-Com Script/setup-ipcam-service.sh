#!/bin/bash
set -e


SERVICE="ipcam-stream.service"
PATHUNIT="ipcam-stream.path"
IPCAM_SCRIPT="/usr/local/bin/start-ipcam.sh"

CONFIG_FILE="/etc/ipcam.conf"
SCRIPT_FILE="/usr/local/bin/start-ipcam.sh"
SERVICE_FILE="/etc/systemd/system/ipcam-stream.service"
PATH_FILE="/etc/systemd/system/ipcam-stream.path"
PATH_OVERRIDE_DIR="/etc/systemd/system/ipcam-stream.path.d"
PATH_OVERRIDE_CONF="$PATH_OVERRIDE_DIR/override.conf"


echo "🔍 Checking and removing IPCAM components..."

# Stop systemd units
if systemctl is-active --quiet "$SERVICE"; then
    echo "🛑 Stopping $SERVICE..."
    sudo systemctl stop "$SERVICE"
fi

if systemctl is-active --quiet "$PATHUNIT"; then
    echo "🛑 Stopping $PATHUNIT..."
    sudo systemctl stop "$PATHUNIT"
fi

# Disable systemd units
if systemctl is-enabled --quiet "$SERVICE"; then
    echo "🚫 Disabling $SERVICE..."
    sudo systemctl disable "$SERVICE"
fi

if systemctl is-enabled --quiet "$PATHUNIT"; then
    echo "🚫 Disabling $PATHUNIT..."
    sudo systemctl disable "$PATHUNIT"
fi

# Remove unit files if they exist
if [ -f "/etc/systemd/system/$SERVICE" ]; then
    echo "🗑 Removing $SERVICE..."
    sudo rm -f "/etc/systemd/system/$SERVICE"
fi

if [ -f "/etc/systemd/system/$PATHUNIT" ]; then
    echo "🗑 Removing $PATHUNIT..."
    sudo rm -f "/etc/systemd/system/$PATHUNIT"
fi

# Remove path override config
if [ -d "$PATH_OVERRIDE_DIR" ]; then
    echo "🧹 Removing path override config..."
    sudo rm -rf "$PATH_OVERRIDE_DIR"
fi

# Reload systemd
echo "🔁 Reloading systemd daemon..."
sudo systemctl daemon-reexec
sudo systemctl daemon-reload

# Remove streaming script
if [ -f "$IPCAM_SCRIPT" ]; then
    echo "🧽 Removing script: $IPCAM_SCRIPT"
    sudo rm -f "$IPCAM_SCRIPT"
fi

# Remove config file (optional)
if [ -f "$CONFIG_FILE" ]; then
    echo "🧽 Removing config: $CONFIG_FILE"
    sudo rm -f "$CONFIG_FILE"
fi

# Uninstall packages (optional)
echo "📦 Checking installed packages..."
for pkg in v4l2loopback-dkms v4l2loopback-utils; do
    if dpkg -s "$pkg" &>/dev/null; then
        echo "🧯 Removing $pkg..."
        sudo apt remove -y "$pkg"
    else
        echo "✅ $pkg not installed."
    fi
done

echo "🎉 Cleanup complete."


# Step 1: Ensure required packages
echo "==> Checking and installing required packages..."
for pkg in v4l2loopback-dkms v4l2loopback-utils; do
    if dpkg -s "$pkg" &>/dev/null; then
        installed_version=$(dpkg -s "$pkg" | grep '^Version:' | awk '{print $2}')
        candidate_version=$(apt-cache policy "$pkg" | grep 'Candidate:' | awk '{print $2}')
        if [ "$installed_version" = "$candidate_version" ]; then
            echo "✅ $pkg is already installed and up to date (version $installed_version)."
        else
            echo "⬆️  $pkg installed: $installed_version, upgrading to $candidate_version..."
            apt install -y "$pkg" && echo "✔️ $pkg upgraded."
        fi
    else
        echo "📦 Installing $pkg..."
        apt update
        apt install -y "$pkg" && echo "✔️ $pkg installed."
    fi
done

# Step 2: Create /etc/ipcam.conf
if [ ! -f "$CONFIG_FILE" ]; then
    echo "IPCAM_URL=http://192.168.0.198:8080" > "$CONFIG_FILE"
    echo "ℹ️ Default IPCAM_URL written to $CONFIG_FILE."
fi

# Step 3: Create /usr/local/bin/start-ipcam.sh
echo "==> Creating script: $SCRIPT_FILE"
cat > "$SCRIPT_FILE" <<'EOF'
#!/bin/bash
set -e

CONFIG_FILE="/etc/ipcam.conf"

# Load IPCAM_URL from config
if [ -f "$CONFIG_FILE" ]; then
    source "$CONFIG_FILE"
fi

: "${IPCAM_URL:=http://192.168.0.198:8080}"

# Check if IP cam is online before starting
if curl --silent --fail --head "$IPCAM_URL/video" > /dev/null; then
    echo "✅ Camera found at $IPCAM_URL/video, starting ffmpeg..."

    sudo modprobe -r v4l2loopback || true
    sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="IPCam" exclusive_caps=1

    exec ffmpeg -re -i "$IPCAM_URL/video" -vcodec rawvideo -pix_fmt yuv420p -f v4l2 /dev/video10
else
    echo "❌ Camera not reachable at $IPCAM_URL/video. Exiting."
    exit 42
fi
EOF

chmod +x "$SCRIPT_FILE"

# Step 4: Create systemd service
echo "==> Creating systemd service: $SERVICE_FILE"
cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=Android IP Camera to V4L2 Virtual Device
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=$SCRIPT_FILE
Restart=on-failure
RestartSec=10
StartLimitInterval=60
StartLimitBurst=6
SuccessExitStatus=42
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

# Step 5: Create systemd path to watch /etc/ipcam.conf
echo "==> Creating systemd path: $PATH_FILE"
cat > "$PATH_FILE" <<EOF
[Unit]
Description=Watch /etc/ipcam.conf and restart ipcam-stream.service on changes

[Path]
PathChanged=$CONFIG_FILE

[Install]
WantedBy=multi-user.target
EOF

mkdir -p "$PATH_OVERRIDE_DIR"
cat > "$PATH_OVERRIDE_CONF" <<EOF
[Path]
Unit=ipcam-stream.service
EOF

# Step 6: Enable and start systemd units
echo "==> Enabling and starting services..."
systemctl daemon-reload
systemctl enable ipcam-stream.service
systemctl enable ipcam-stream.path
systemctl start ipcam-stream.path
systemctl restart ipcam-stream.service

echo "✅ Setup complete. Service is running. Config changes to $CONFIG_FILE will auto-restart the stream."



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


