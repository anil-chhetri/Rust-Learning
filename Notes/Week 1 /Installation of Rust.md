# Week 1: Rust Fundamentals & Mental Model Shift

Focus Areas:
- Rust installation and setup
- Basic syntax and data types
- Variables, mutability, and ownership basics
- Cargo package manager
-  First programs


## Rust Installation and setup

This command is downloading and running the Rust installation script. Let's  break it down step by step:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. `curl` is a command-line tool used to transfer data to or from a server.

2. The options used with curl:
   - `--proto '=https'` restricts curl to only use HTTPS protocol for security
   - `--tlsv1.2` ensures curl uses at least TLS 1.2 (a secure encryption protocol)
   - `-s` makes curl operate in silent mode (no progress meter)
   - `-S` shows error messages if something goes wrong
   - `-f` (fail) makes curl exit with an error status if the server returns an error

3. `https://sh.rustup.rs` is the URL to the Rustup installation script.

4. The pipe symbol `|` takes the output from the curl command (which is the installation script) and feeds it as input to the next command.

5. `sh` is the shell that will execute the installation script received from curl.

In simple terms, this command:
1. Downloads the Rustup installer script securely from the official Rust website
2. Passes the script directly to your shell to execute it
3. The script then installs Rustup, which will manage your Rust toolchain

This is a common pattern for installing development tools—downloading an installer script and executing it directly. The security options (HTTPS only, TLS 1.2) are there to ensure you're downloading from a legitimate source using secure protocols.

When you run this command, the script will start an interactive installation process that lets you choose installation options for Rust.


Run following command to verify if rust is installed on the system.:

```bash
rustc --version
cargo --version
```

### What is the Rust Toolchain?
The Rust toolchain is a complete set of components needed to compile, run, and manage Rust code. The core components include:

**_rustc_**: The Rust compiler that turns your Rust code into executable binaries  
**_cargo_**: The package manager and build system  
**_rustup_**: The toolchain installer and version manager  
**_rustfmt_**: A tool for formatting Rust code according to style guidelines
**_clippy_**: A collection of lints to catch common mistakes and improve your code  
**_rust-analyzer_**: A language server implementation providing IDE features  


### Introduction to `RustC`

**_rustup_** is the official toolchain installer and manager for Rust. It is not just for installing Rust—it’s designed to manage multiple versions of the Rust compiler (rustc), tools, and platforms. Think of it as a Swiss Army knife for Rust development environments.

Rustup is the official Rust toolchain installer and version manager. Think of it as the control center for managing everything related to Rust on your system. It lets you install, update, and switch between different versions of Rust and its components.

#### Core Purpose of Rustup

Rustup solves several important problems:
- Installing Rust tools consistently across different operating systems
- Managing multiple versions of Rust on the same machine
- Keeping your Rust installation up to date
- Adding support for different compilation targets

#### Common Rustup Commands

Here are the most frequently used rustup commands:

#### Installation and Updates

```bash
# Update rustup itself and all installed toolchains
rustup update

# Update only a specific toolchain
rustup update stable

# Check for updates but don't install them
rustup check
```

#### Managing Toolchains

```bash
# Install a specific version of Rust
rustup install stable
rustup install nightly
rustup install 1.70.0

# List all installed toolchains
rustup toolchain list

# Remove a toolchain
rustup toolchain uninstall nightly

# Set your default toolchain
rustup default stable
```

#### Working with Components

```bash
# List available components
rustup component list

# Add a specific component (like rust-analyzer, clippy, etc.)
rustup component add rust-analyzer
rustup component add clippy
rustup component add rustfmt

# Remove a component
rustup component remove rust-src
```

#### Target Management (for cross-compilation)

```bash
# List available targets
rustup target list

# List installed targets
rustup target list --installed

# Add a target for cross-compilation
rustup target add x86_64-pc-windows-msvc
rustup target add wasm32-unknown-unknown

# Remove a target
rustup target remove arm-unknown-linux-gnueabihf
```

#### Override Management

```bash
# Set a directory-specific toolchain
rustup override set nightly

# See what override is set for current directory
rustup override list

# Remove an override
rustup override unset
```

#### Information Commands

```bash
# Show current configuration
rustup show

# Show the Rust version in use
rustup run stable rustc --version

# Show where rustup stores its files
rustup home

# Show documentation in your browser
rustup doc
rustup doc --std  # Standard library docs
```

#### Setting Profiles

```bash
# Set minimal profile (smaller download, fewer components)
rustup set profile minimal

# Set default profile
rustup set profile default

# Set complete profile (all components)
rustup set profile complete
```

### **Core Features of `rustup`**
1. **Toolchain Management**  
   Install, switch, and manage Rust versions (`stable`, `beta`, `nightly`).
2. **Component Management**  
   Add/remove tools like `clippy`, `rustfmt`, or `rust-analyzer`.
3. **Cross-Compilation**  
   Compile for other platforms (e.g., Windows → Linux).
4. **Project-Specific Overrides**  
   Set a Rust version per project using `.rust-toolchain`.
5. **Self-Update**  
   Update `rustup` itself to the latest version.

---

### **Essential `rustup` Commands**

#### **1. Installing/Updating Toolchains**
| Command | Purpose | Example |
|---------|---------|---------|
| `rustup install` | Install a toolchain | `rustup install nightly` |
| `rustup update` | Update all toolchains | `rustup update` |
| `rustup toolchain list` | List installed toolchains | `rustup toolchain list` |

#### **2. Default Toolchain**
| Command | Purpose | Example |
|---------|---------|---------|
| `rustup default` | Set default toolchain | `rustup default nightly` |
| `rustup show` | Show active toolchain | `rustup show` |

#### **3. Components (Tools)**
| Command | Purpose | Example |
|---------|---------|---------|
| `rustup component add` | Install a component | `rustup component add clippy` |
| `rustup component remove` | Remove a component | `rustup component remove rust-docs` |
| `rustup component list` | List available components | `rustup component list` |

#### **4. Cross-Compilation Targets**
| Command | Purpose | Example |
|---------|---------|---------|
| `rustup target add` | Add a target platform | `rustup target add wasm32-unknown-unknown` |
| `rustup target list` | List installed targets | `rustup target list` |

#### **5. Project-Specific Overrides**
| Command/Practice | Purpose | Example |
|------------------|---------|---------|
| `.rust-toolchain` file | Auto-switch toolchain per project | Create a file with `nightly` |
| `rustup override` | Manually set toolchain for a directory | `rustup override set nightly` |

#### **6. Advanced Commands**
| Command | Purpose | Example |
|---------|---------|---------|
| `rustup run` | Run a command with a specific toolchain | `rustup run nightly cargo build` |
| `rustup self update` | Update `rustup` itself | `rustup self update` |
| `rustup man` | View manual pages | `rustup man cargo` |

---

### **Real-World Use Cases**

#### **1. Working with Nightly Features**
```bash
# Install nightly and enable a feature (e.g., `async` traits)
rustup install nightly
cargo +nightly new async_project
cd async_project
echo '#![feature(async_fn_in_trait)]' >> src/main.rs
cargo +nightly run
```

#### **2. Cross-Compiling for WebAssembly**
```bash
# Add WASM target and build
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

#### **3. CI/CD Setup**
Use a `.rust-toolchain` file to ensure consistent builds:
```toml
# .rust-toolchain.toml
[toolchain]
channel = "1.75.0"
components = ["rustfmt", "clippy"]
```

#### **4. Linting and Formatting**
```bash
# Add tools globally
rustup component add clippy rustfmt
# Lint your project
cargo clippy
# Format code
cargo fmt
```

---

### **Key Configuration Files**
1. **`~/.rustup/settings.toml`**  
   Global settings (e.g., default profile, proxies).
2. **`.rust-toolchain.toml`**  
   Project-specific toolchain and components.

---

### **Environment Variables**
| Variable | Purpose |
|----------|---------|
| `RUSTUP_TOOLCHAIN` | Override the default toolchain | 
| `RUSTUP_HOME` | Change `rustup` installation directory | 
| `RUSTUP_DIST_SERVER` | Use a custom update server (for enterprises) |

---

### **Best Practices**
1. **Pin Toolchain Versions**  
   Use exact versions in `.rust-toolchain.toml` for critical projects:
   ```toml
   [toolchain]
   channel = "1.75.0"
   ```
2. **Clean Unused Toolchains**  
   ```bash
   rustup toolchain uninstall nightly
   ```
3. **Use `+toolchain` Syntax**  
   Run commands with a specific toolchain without changing defaults:
   ```bash
   cargo +nightly test
   ```

---

### **Troubleshooting**
- **Broken Toolchain**: Run `rustup toolchain uninstall <name>` and reinstall.
- **Missing Components**: Use `rustup component add <component>`.
- **Proxy Issues**: Configure proxies via `RUSTUP_HTTP_PROXY`.

---

### **References**
1. **Official `rustup` Docs**: [https://rust-lang.github.io/rustup/](https://rust-lang.github.io/rustup/)  
2. **Rustup Components**: [https://rust-lang.github.io/rustup/components.html](https://rust-lang.github.io/rustup/components.html)  
3. **Cross-Compilation Guide**: [https://github.com/japaric/rust-cross](https://github.com/japaric/rust-cross)  

---

### **Exercise**


Let's solve these exercises step by step.

## Exercise 1: Install the beta toolchain and run cargo +beta --version

To complete this exercise:

```bash
# Step 1: Install the beta toolchain
rustup install beta

# Step 2: Run cargo with the beta toolchain
cargo +beta --version
```

The first command installs the beta version of Rust. The second command runs the cargo command specifically using the beta toolchain you just installed. The `+beta` syntax tells rustup to use the beta toolchain for this specific command.

When you run this, you should see output showing the version of cargo from the beta channel, something like:
```
cargo 1.xx.0-beta (xxxxxxxx 2023-xx-xx)
```

## Exercise 2: Create a .rust-toolchain file in a project to use nightly and rustfmt

For this exercise:

1. Navigate to your project directory (or create one if needed)
   ```bash
   mkdir -p rust_project
   cd rust_project
   ```

2. Create a `.rust-toolchain.toml` file with the following content:
   ```toml
   [toolchain]
   channel = "nightly"
   components = ["rustfmt"]
   ```

Alternatively, for a simpler (but older) approach, you could create a plain text file named `.rust-toolchain` containing:
```
nightly
```

But the TOML format is preferred as it allows you to specify components.

With this file in place, whenever you run Rust commands in this directory, rustup will automatically use the nightly toolchain and ensure rustfmt is installed. This means you can run commands like `cargo build` or `cargo fmt` without explicitly specifying `+nightly`.

The `.rust-toolchain.toml` file is a project-specific configuration that overrides your global Rust toolchain settings just for this project.