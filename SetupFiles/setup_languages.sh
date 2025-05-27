#!/bin/bash

# This shell script will set up the following programming languages and their VS Code environments on an Ubuntu/Debian-based system.

# ------------------------
# Languages Installed
# ------------------------
# 01  Python        apt         python3, python3-pip
# 02  C++           apt         g++
# 03  Rust          curl        rustup (Rust toolchain)
# 04  Node.js       apt         nodejs
# 05  TypeScript    npm         typescript (tsc)
# 06  Java          apt         default-jdk
# 07  Go            apt         golang-go
# 08  Julia         snap        julia
# 09  Modular       curl        Modular AI runtime
# 10  Swift         apt         swift
# 11  R             apt         r-base
# 12  PowerShell    apt+wget    powershell
# 13  C (Clang)     apt         clang, libcurl4-openssl-dev, libicu-dev, libssl-dev
# 14  Elixir        apt         elixir
# 15  Erlang        apt         erlang
# 16  ShellCheck    apt         shellcheck (for shell script linting)
# 17  Terraform     apt         terraform
# 18  Ansible       apt         ansible
# 19  Kotlin        SDKMAN      kotlinc
# 20  Dart          snap        dart
# 21  Flutter       snap        flutter
# 22  Groovy        apt         groovy
# 23  Ruby          apt         ruby
# 24  Solidity      npm         solc

# ------------------------
# VS Code Extensions Installed
# ------------------------
# Refer to EXTENSIONS array below.

set -e

# -----------------------------------
# Install Visual Studio Code
# -----------------------------------
sudo apt update
sudo apt install -y wget

wget -qO- https://packages.microsoft.com/keys/microsoft.asc \
  | gpg --dearmor \
  | sudo tee /usr/share/keyrings/packages.microsoft.gpg > /dev/null

echo "deb [arch=amd64 signed-by=/usr/share/keyrings/packages.microsoft.gpg] \
  https://packages.microsoft.com/repos/code stable main" \
  | sudo tee /etc/apt/sources.list.d/vscode.list > /dev/null

sudo apt update
sudo apt install -y code

# -----------------------------------
# Declare Language Installer Commands
# -----------------------------------
declare -A kv_pairsForCompilerInstall

# Language setup (compiler/runtime install commands)
kv_pairsForCompilerInstall[python3]="sudo apt install -y python3 python3-pip"
kv_pairsForCompilerInstall[rustc]="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
kv_pairsForCompilerInstall[g++]="sudo apt install -y g++"
kv_pairsForCompilerInstall[node]="sudo apt install -y nodejs"
kv_pairsForCompilerInstall[npm]="sudo apt install -y npm"
kv_pairsForCompilerInstall[tsc]="sudo npm install -g typescript"
kv_pairsForCompilerInstall[java]="sudo apt install -y default-jdk"
kv_pairsForCompilerInstall[go]="sudo apt install -y golang-go"
kv_pairsForCompilerInstall[julia]="sudo snap install julia --classic"
kv_pairsForCompilerInstall[modular]="curl https://get.modular.com | sh"
kv_pairsForCompilerInstall[swift]="sudo apt install -y swift"
kv_pairsForCompilerInstall[R]="sudo apt install -y r-base"
kv_pairsForCompilerInstall[clang]="sudo apt install -y clang libcurl4-openssl-dev libicu-dev libssl-dev && systemctl daemon-reload"
kv_pairsForCompilerInstall[elixir]="sudo apt install -y elixir"
kv_pairsForCompilerInstall[erl]="sudo apt install -y erlang"
kv_pairsForCompilerInstall[shellcheck]="sudo apt install -y shellcheck"
kv_pairsForCompilerInstall[terraform]="sudo apt install -y terraform"
kv_pairsForCompilerInstall[ansible]="sudo apt install -y ansible"
kv_pairsForCompilerInstall[kotlinc]="curl -s https://get.sdkman.io | bash && source \$HOME/.sdkman/bin/sdkman-init.sh && sdk install kotlin"
kv_pairsForCompilerInstall[dart]="sudo snap install dart --classic"
kv_pairsForCompilerInstall[flutter]="sudo snap install flutter --classic"
kv_pairsForCompilerInstall[groovy]="sudo apt install -y groovy"
kv_pairsForCompilerInstall[ruby]="sudo apt install -y ruby-full"
kv_pairsForCompilerInstall[solc]="sudo npm install -g solc"
kv_pairsForCompilerInstall[pwsh]="sudo apt install -y wget apt-transport-https software-properties-common && \
  wget -q https://packages.microsoft.com/config/ubuntu/\$(lsb_release -rs)/packages-microsoft-prod.deb && \
  sudo dpkg -i packages-microsoft-prod.deb && \
  sudo apt update && \
  sudo apt install -y powershell"

# -----------------------------------
# Install Each Language Tool
# -----------------------------------
for key in "${!kv_pairsForCompilerInstall[@]}"; do
  echo "$key => ${kv_pairsForCompilerInstall[$key]}"
  echo "----------------------------------------------------------------------------------"
  CMD="$key"
  INSTALL_CMD="${kv_pairsForCompilerInstall[$key]}"
  if ! command -v "$CMD" &> /dev/null; then
    echo "Installing $CMD..."
    eval "$INSTALL_CMD"
  else
    echo "$CMD is already installed."
  fi
  echo "----------------------------------------------------------------------------------"
done

# -----------------------------------
# VS Code Extensions to Install
# -----------------------------------
EXTENSIONS=(
    "ms-python.python"
    "ms-vscode.cpptools"
    "rust-lang.rust-analyzer"
    "ms-vscode.vscode-typescript-next"
    "vscjava.vscode-java-pack"
    "golang.go"
    "julialang.language-julia"
    "ms-vscode.powershell"
    "jakebecker.elixir-ls"
    "pgourlain.erlang"
    "timonwong.shellcheck"
    "hashicorp.terraform"
    "redhat.ansible"
    "fwcd.kotlin"
    "dart-code.dart-code"
    "dart-code.flutter"
    "vmware.vscode-boot-dev-pack"
    "rebornix.Ruby"
    "JuanBlanco.solidity"
    "ms-azuretools.vscode-docker"
    "ms-kubernetes-tools.vscode-aks-tools"
    "ms-kubernetes-tools.vscode-kubernetes-tools"
    "ms-ossdata.vscode-postgresql"
    "ms-vscode-remote.remote-ssh"
    "ms-vscode-remote.remote-ssh-edit"
    "ms-vscode.remote-explorer"
    "github.copilot"
    "github.copilot-chat"
    "github.vscode-pull-request-github"
    "visualstudioexptteam.intellicode-api-usage-examples"
    "visualstudioexptteam.vscodeintellicode"
)

# Install Extensions if not already present
for EXTENSION in "${EXTENSIONS[@]}"; do
    if ! code --list-extensions | grep -q "$EXTENSION"; then
        echo "Installing VS Code extension: $EXTENSION"
        code --install-extension "$EXTENSION"
    else
        echo "VS Code extension $EXTENSION is already installed."
    fi
done
