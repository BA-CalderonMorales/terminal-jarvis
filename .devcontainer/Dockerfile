FROM rust:1.87

# Install essential development tools
RUN apt-get update && apt-get install -y \
    curl \
    git \
    git-lfs \
    build-essential \
    pkg-config \
    libssl-dev \
    nodejs \
    npm \
    python3 \
    python3-pip \
    python3-venv \
    ca-certificates \
    sudo \
    && curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
    && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
    && apt-get update \
    && apt-get install -y gh \
    && rustup component add clippy rustfmt \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create vscode user for better compatibility with VS Code
RUN groupadd --gid 1000 vscode \
    && useradd --uid 1000 --gid vscode --shell /bin/bash --create-home vscode \
    && echo 'vscode ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Install uv for the vscode user
USER vscode
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/home/vscode/.cargo/bin:$PATH"

WORKDIR /workspaces/terminal-jarvis