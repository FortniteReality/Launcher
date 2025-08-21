# Reality Launcher

A feature-rich Tauri-based game launcher supporting Epic Games ChunkV4 manifests with comprehensive game management, social features, and Discord integration.

## Features

### Game Management
- **Installation & Updates**: Install and update games using Epic Games ChunkV4 manifest system
- **Verification**: Verify game file integrity and detect corrupted installations
- **Uninstallation**: Clean removal of games and associated files
- **Version Management**: Automatically fetch and install latest game versions

### Authentication & Security
- **Epic Games Integration**: Secure login using recreated Epic Games API endpoints
- **Token Management**: Handles authentication tokens with proper refresh mechanisms
- **Cached Authentication**: Stores login credentials securely for seamless experience

### Data Management
- **Install Cache**: Efficiently caches installation data to reduce download times
- **Game Data Fetching**: Retrieves comprehensive game information and metadata
- **Smart Caching**: Optimized caching system for faster launches and updates

### Social Features
- **Friends System**: View online friends and their game status
- **Friend Requests**: Send, accept, and decline friend requests
- **Social Integration**: Stay connected with your gaming community

### Additional Features
- **Discord Rich Presence**: Display current game status on Discord
- **C# Launcher Integration**: Utilizes a separate C# launcher component for game execution
- **Open Source Components**: C# launcher source code available for transparency

## Installation

1. Download the latest release from the releases page
2. Run the msi to install the application
3. Log in with your Reality account credentials

## Usage

### First Launch
1. **Login**: Enter your Reality credentials when prompted
3. **Install Reality**: Click install
4. **Launch**: Click play to launch games through the integrated C# launcher

### Managing Games
- **Install**: Click the install button next to any game in your library
- **Update**: Games with available updates will show an update button
- **Verify**: Right-click on installed games to verify file integrity
- **Uninstall**: Remove games through the context menu or game management page

### Social Features
- **Friends List**: Access friends through the social tab
- **Online Status**: See which friends are currently online and playing
- **Friend Requests**: Manage incoming and outgoing friend requests

## Architecture

### Core Components
- **Tauri Frontend**: Modern web-based UI with native performance
- **Rust Backend**: High-performance backend for game management and API communication
- **C# Launcher**: Separate component for game execution (source available)
- **ChunkV4 Handler**: Manages Epic Games manifest processing
- **Authentication Module**: Handles recreated Epic Games API communication
- **Cache System**: Manages local data storage and retrieval

### API Integration
- Uses recreated Epic Games API endpoints for authentication and game data
- Implements proper token refresh and session management

## Configuration

The launcher stores configuration and cache data in:
- **Windows**: `%APPDATA%/RealityLauncher/`
- **Settings**: User preferences and cached login data
- **Games**: Installation metadata and verification data

## Discord Integration

Reality Launcher features Discord Rich Presence showing:
- Currently playing game
- Game session duration
- Custom status messages

## Development

### Tech Stack
- **Frontend**: Modern web technologies (HTML, CSS, JavaScript/TypeScript) via Tauri
- **Backend**: Rust for high-performance system operations
- **Game Launcher**: C# component for game execution
- **Cross-platform**: Built with Tauri for native performance

### C# Launcher Component
The C# launcher source code is available for:
- Custom modifications
- Bug fixes and improvements
- Educational purposes
- Community contributions

### Building from Source
```bash
# Clone the repository
git clone https://github.com/FortniteReality/Launcher.git

# Install Tauri CLI
cargo install tauri-cli

# Install frontend dependencies
npm install

# Build the application
npm run tauri build
```

## System Requirements

### Minimum Requirements
- **OS**: Windows 10 (64-bit) or later
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 2GB for launcher, additional space for games
- **Network**: Stable internet connection required

### Recommended
- **OS**: Windows 11
- **RAM**: 16GB
- **Storage**: SSD with 100GB+ free space
- **Network**: Broadband internet connection

## Troubleshooting

### Common Issues
- **Login Problems**: Verify Epic Games credentials and internet connection
- **Installation Failures**: Check disk space and file permissions
- **Game Launch Issues**: Verify game files and check C# launcher logs
- **Discord RPC**: Ensure Discord is running and RPC is enabled

### Support
- Check the [Discord](https://discord.gg/reality) for known problems
- Submit bug reports with detailed information