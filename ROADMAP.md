# xfetch Roadmap

## Phase 1 · Foundation & Core
<!-- phase:phase-1:foundation -->

- [x] Initialize Rust project with dependencies
- [x] Implement cross-platform OS detection (Linux, Windows, macOS)
- [x] Create system information gathering module
- [x] Implement configuration system with JSONC support
- [x] Build UI rendering engine with crossterm

## Phase 2 · System Information Modules
<!-- phase:phase-2:system-modules -->

- [x] OS Name & Architecture display
- [x] Kernel version detection
- [x] Hostname resolution
- [x] Shell detection and display
- [x] Terminal emulator detection
- [x] CPU model & frequency information
- [x] GPU detection (discrete & integrated)
- [x] Memory and RAM usage display
- [x] Disk usage statistics
- [x] Battery status and percentage
- [x] System uptime calculation
- [x] Package count for multiple managers (pacman, dpkg, scoop)
- [x] Desktop Environment / Window Manager detection

## Phase 3 · Visual Customization & Layouts
<!-- phase:phase-3:visual-features -->

- [x] Custom ASCII art support from text files
- [x] Image/SVG logo support via viuer
- [x] ANSI color codes in ASCII logos
- [x] Icon customization per module (Nerd Fonts)
- [x] Color customization per module
- [x] Default layout (side-by-side)
- [x] Pac-Man layout with custom header/footer
- [x] Side-block layout implementation
- [x] Tree layout for hierarchical display
- [x] Section layout for grouped information
- [x] Color palette display with style options

## Phase 4 · Documentation & Examples
<!-- phase:phase-4:documentation -->

- [x] Installation guide (INSTALLATION.md)
- [x] Configuration guide (CONFIGURATION.md)
- [x] Quick install script for Linux/macOS
- [x] PowerShell install script for Windows
- [x] Create 20+ example configurations
- [x] Create sample logos (text and SVG)
- [x] Setup uninstallation scripts
- [x] Layout documentation (LAYOUTS.md)

## Phase 5 · Package Manager Expansion
<!-- phase:phase-5:package-managers -->

- [ ] Add RPM package manager support (Fedora, RHEL)
- [ ] Add APK package manager support (Alpine)
- [ ] Add Nix package manager support
- [ ] Add Homebrew package manager support (macOS/Linux)
- [ ] Add Chocolatey package manager support (Windows)
- [ ] Detect multiple installed package managers
- [/] Optimize package count detection performance

## Phase 6 · Network & Connectivity
<!-- phase:phase-6:network -->

- [ ] Implement local IP address detection
- [ ] Fetch public IP address (with privacy option)
- [ ] Add IPv6 support
- [ ] Display network interface information
- [ ] Add option to disable IP fetching for privacy

## Phase 7 · Enhanced Modules
<!-- phase:phase-7:enhanced-modules -->

- [ ] Implement music player integration (MPD support)
- [ ] Add Spotify current track display
- [ ] Implement weather module with location API
- [ ] Add timezone and world clock display
- [ ] Implement user info and login status
- [ ] Add display resolution and refresh rate
- [ ] Add theme and color scheme detection

## Phase 8 · Additional Layouts
<!-- phase:phase-8:additional-layouts -->

- [ ] Implement compact layout for minimal output
- [ ] Implement horizontal layout variant
- [ ] Implement bottom layout with logo below info
- [ ] Implement minimal layout (text-only)
- [ ] Add layout preview documentation

## Phase 9 · Performance Optimization
<!-- phase:phase-9:performance -->

- [ ] Parallelize slow hardware probes
- [ ] Implement caching for module data
- [ ] Optimize GPU detection for multi-GPU systems
- [ ] Add lazy loading for optional modules
- [ ] Benchmark and profile performance

## Phase 10 · CI/CD & Distribution
<!-- phase:phase-10:cicd -->

- [ ] Setup GitHub Actions for automated builds
- [ ] Create binary releases for Linux x86_64
- [ ] Create binary releases for macOS (Intel & ARM)
- [ ] Create binary releases for Windows
- [ ] Setup AUR package for Arch Linux
- [ ] Setup Homebrew tap for macOS
- [ ] Setup PyPI or cargo registry for distribution
- [ ] Setup automated changelog generation

## Phase 11 · Community & Ecosystem
<!-- phase:phase-11:ecosystem -->

- [ ] Create themes repository / registry
- [ ] Implement theme download manager
- [ ] Create online theme preview tool
- [ ] Setup community theme contributions process
- [ ] Create plugin system for custom modules
- [ ] Implement plugin configuration validation
- [ ] Setup community issue templates
- [ ] Create contribution guidelines (CONTRIBUTING.md)

## Phase 12 · Testing & Quality Assurance
<!-- phase:phase-12:testing -->

- [ ] Implement unit tests for info module
- [ ] Implement unit tests for config module
- [ ] Implement integration tests for layouts
- [ ] Setup linting with clippy
- [ ] Setup code formatter (rustfmt)
- [ ] Implement platform-specific tests for each OS
- [ ] Add cross-platform testing suite
- [ ] Setup code coverage reporting

## Phase 13 · Advanced Features
<!-- phase:phase-13:advanced -->

- [ ] Implement custom module scripting language / support
- [ ] Add conditional module display based on system state
- [ ] Implement theme system with variables
- [ ] Add animation support for transitional elements
- [ ] Implement real-time stats updates / daemon mode
- [ ] Add config hot-reload capability
- [ ] Implement telemetry (optional, privacy-respecting)
- [ ] Add accessibility features (high contrast themes)

## Phase 14 · Documentation & Marketing
<!-- phase:phase-14:marketing -->

- [ ] Create comprehensive user manual
- [ ] Create video tutorials
- [ ] Setup project website with showcase
- [ ] Create developer documentation
- [ ] Publish blog posts about features
- [ ] Create comparison guide with similar tools
- [ ] Setup Discord/Slack community channel
- [ ] Create contribution bounty program
