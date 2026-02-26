#!/usr/bin/env bash
set -e

echo "Downloading Go 1.24.0..."
ARCH=$(uname -m)
if [ "$ARCH" = "x86_64" ]; then
    GOARCH="amd64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    GOARCH="arm64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

cd /tmp
curl -fLO "https://go.dev/dl/go1.24.0.linux-${GOARCH}.tar.gz"

echo "Installing Go to /usr/local/go..."
sudo rm -rf /usr/local/go
sudo tar -C /usr/local -xzf "go1.24.0.linux-${GOARCH}.tar.gz"

echo "Updating PATH in ~/.bashrc..."
if ! grep -q "/usr/local/go/bin" ~/.bashrc; then
    echo 'export PATH=/usr/local/go/bin:$PATH' >> ~/.bashrc
fi

echo "Go 1.24.0 installed successfully!"
echo "Please run the following command to update your current session:"
echo "source ~/.bashrc"
