#!/bin/bash

#This shell script will set up the following programming languages and their VS Code environments on an Ubuntu/Debian-based system.

# Languages Installed

# Sn   Language      Package Manager     Installed Package

# 01   Python        apt                 python3, python3-pip
# 02   C++	         apt                 g++
# 03   Rust	         curl                rustup (Rust toolchain)
# 04   Node.js	     apt                 nodejs
# 05   TypeScript    npm                 typescript (tsc)
# 06   Java          apt                 default-jdk
# 07   Go            apt                 golang-go
# 08   Julia	     snap                julia
# 09   Modular       curl                Modular AI runtime
# 10   Swift	     apt                 swift
# 11   R	         apt                 r-base
# 12   PowerShell	 apt                 + wget	powershell
# 13   C (Clang)	 apt                 clang, libcurl4-openssl-dev, libicu-dev, libssl-dev
# 14   Elixir        apt                 elixir
# 15   Erlang        apt                 erlang
# 16   ShellCheck    apt                 shellcheck (for shell script linting)
# 17   Terraform     apt                 terraform
# 18   Ansible       apt                 ansible
# 19   Kotlin	     SDKMAN              kotlinc
# 20   Dart          snap                dart
# 21   Flutter       snap                Flutter
# 22   Groovy        apt                 groovy
# 23   Ruby          apt                 ruby
# 24   Solidity      npm                 solc


# VS Code Extensions Installed

# 01  Python         ms-python.python
# 02  C++            ms-vscode.cpptools
# 03  Rust           rust-lang.rust-analyzer
# 04  TypeScript     ms-vscode.vscode-typescript-next
# 05  Java           vscjava.vscode-java-pack
# 06  Go             golang.go
# 07  Julia          julialang.language-julia
# 08  Elixir         jakebecker.elixir-ls
# 09  Erlang         pgourlain.erlang
# 10  ShellCheck     timonwong.shellcheck
# 11  PowerShell     ms-vscode.powershell
# 12  Terraform      hashicorp.terraform
# 13  Ansible        redhat.ansible
# 14  Dart           dart-code.dart-code
# 15  Flutter        dart-code.flutter
# 16  Groovy         vmware.vscode-boot-dev-pack
# 17  Ruby           rebornix.Ruby
# 18  Kotlin         fwcd.kotlin
# 19  Solidity       JuanBlanco.solidity


 

sudo apt update 

sudo apt install -y wget 

wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor | sudo tee /usr/share/keyrings/packages.microsoft.gpg > /dev/null 

echo "deb [arch=amd64 signed-by=/usr/share/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main" | sudo tee /etc/apt/sources.list.d/vscode.list > /dev/null 

sudo apt update 

sudo apt install -y code 



# Function to check if a command exists and install if missing
check_and_install() {
echo "----------------------------------------------------------------------------------"    
    CMD=$1
    INSTALL_CMD=$2
    if ! command -v $CMD &> /dev/null; then
        echo "Installing $CMD..."
        eval $INSTALL_CMD
    else
        echo "$CMD is already installed."
    fi
}

# Function to install VS Code extensions
install_vscode_extension() {
    EXTENSION=$1
    if ! code --list-extensions | grep -q "$EXTENSION"; then
        echo "Installing VS Code extension: $EXTENSION"
        code --install-extension "$EXTENSION"
    else
        echo "VS Code extension $EXTENSION is already installed."
    fi
}

echo "Updating system packages..."
sudo apt update && sudo apt upgrade -y

# Install Python
check_and_install "python3" "sudo apt install -y python3 python3-pip"

# Install C++ (G++)
check_and_install "g++" "sudo apt install -y g++"

# Install Rust
check_and_install "rustc" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"

# Install Node.js & TypeScript
check_and_install "node" "sudo apt install -y nodejs"
check_and_install "npm" "sudo apt install -y npm"
check_and_install "tsc" "sudo npm install -g typescript"

# Install Java (JDK)
check_and_install "java" "sudo apt install -y default-jdk"

# Install Go
check_and_install "go" "sudo apt install -y golang-go"

# Install Julia
check_and_install "julia" "sudo snap install julia --classic"

# Install Modular
check_and_install "modular" "curl https://get.modular.com | sh"

# Install Swift
check_and_install "swift" "sudo apt install -y swift"

# Install R
check_and_install "R" "sudo apt install -y r-base"

# Install PowerShell
check_and_install "pwsh" "sudo apt install -y wget apt-transport-https software-properties-common && \
    wget -q https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb && \
    sudo dpkg -i packages-microsoft-prod.deb && \
    sudo apt update && \
    sudo apt install -y powershell"

# Install Clang & Dependencies
check_and_install "clang" "sudo apt install -y clang libcurl4-openssl-dev libicu-dev libssl-dev && systemctl daemon-reload"

# Install Elixir
check_and_install "elixir" "sudo apt install -y elixir"

# Install Erlang
check_and_install "erl" "sudo apt install -y erlang"

# Install ShellCheck
check_and_install "shellcheck" "sudo apt install -y shellcheck"

# Install Terraform
check_and_install "terraform" "sudo apt install -y terraform"

# Install Ansible
check_and_install "ansible" "sudo apt install -y ansible"

# Install Kotlin
check_and_install "kotlinc" "curl -s https://get.sdkman.io | bash && source ~/.sdkman/bin/sdkman-init.sh && sdk install kotlin"

# Install Dart & Flutter
check_and_install "dart" "sudo snap install dart --classic"
check_and_install "flutter" "sudo snap install flutter --classic"

# Install Groovy
check_and_install "groovy" "sudo apt install -y groovy"

# Install Ruby
check_and_install "ruby" "sudo apt install -y ruby-full"

# Install Solidity
check_and_install "solc" "sudo npm install -g solc"

# List of VS Code extensions to check and install
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

# Loop through each extension and install if necessary
for EXT in "${EXTENSIONS[@]}"; do
    install_vscode_extension "$EXT"
done


echo "All required environments and VS Code extensions have been installed successfully!"
