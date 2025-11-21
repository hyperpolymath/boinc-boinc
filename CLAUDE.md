# CLAUDE.md

This document provides context and guidance for AI assistants (like Claude) working with the BOINC codebase.

## Project Overview

BOINC (Berkeley Open Infrastructure for Network Computing) is an open-source middleware system for volunteer computing and grid computing. It allows researchers to tap into the processing power of thousands of volunteers' computers for scientific research projects.

### Key Components

1. **Client** - Runs on volunteer computers, manages work units
2. **Server** - Distributes work and collects results
3. **Web Interface** - Project websites for volunteers
4. **Manager** - GUI application for volunteers to control the client
5. **API** - Libraries for creating BOINC applications

## Technology Stack

- **Languages**: C++, PHP, Python, JavaScript
- **Databases**: MySQL/MariaDB
- **Build System**: Autotools (autoconf/automake) or CMake
- **Platforms**: Linux, Windows, macOS, Android

## Development Environment Setup

### Prerequisites

```bash
# Debian/Ubuntu
sudo apt-get install git build-essential autoconf automake libtool \
  pkg-config libssl-dev libcurl4-openssl-dev libmysqlclient-dev \
  libnotify-dev libx11-dev libxss-dev libxmu-dev freeglut3-dev \
  libwxgtk3.0-gtk3-dev

# macOS
brew install autoconf automake libtool pkg-config openssl curl mysql wxwidgets

# Fedora/RHEL
sudo dnf install gcc-c++ autoconf automake libtool openssl-devel \
  libcurl-devel mysql-devel libnotify-devel libX11-devel \
  libXScrnSaver-devel libXmu-devel freeglut-devel wxGTK-devel
```

### Building from Source

```bash
# Clone the repository (if not already cloned)
git clone https://github.com/BOINC/boinc.git
cd boinc

# Generate build files
./_autosetup

# Configure
./configure --disable-server

# Build
make

# Build with server components
./configure --enable-server
make
```

## Project Structure

```
boinc/
├── client/         # BOINC client code
├── clientgui/      # Manager GUI (wxWidgets)
├── api/            # BOINC API for applications
├── lib/            # Shared libraries
├── db/             # Database schemas and utilities
├── sched/          # Server scheduling components
├── html/           # Web interface (PHP)
├── py/             # Python utilities
├── tools/          # Server-side tools
├── samples/        # Example BOINC applications
├── android/        # Android client
├── mac_build/      # macOS-specific build files
├── win_build/      # Windows-specific build files
└── doc/            # Documentation
```

## Common Development Tasks

### Running Tests

```bash
# Run unit tests (if available)
make check

# Integration tests
cd tests
./run_tests.sh
```

### Code Style

- **C++**: Follow the existing code style (generally Google C++ Style Guide)
- **Indentation**: 4 spaces (no tabs)
- **Line length**: ~100 characters
- **Naming**:
  - Functions/methods: `snake_case`
  - Classes: `CamelCase` or `UPPER_CASE`
  - Variables: `snake_case`

### Building Specific Components

```bash
# Client only
./configure --disable-server --disable-manager
make

# Manager only
./configure --disable-server --enable-client --enable-manager
make

# Server components
./configure --enable-server
make
```

## Key Files and Their Purposes

- `configure.ac` - Autoconf configuration
- `Makefile.am` - Automake makefile templates
- `version.h` - Version information
- `client/client_state.cpp` - Main client state machine
- `sched/sched_*.cpp` - Server scheduling logic
- `db/boinc_db.cpp` - Database access layer
- `lib/` - Shared utility functions

## Database Schema

The BOINC server uses MySQL/MariaDB with tables including:
- `user` - Volunteer accounts
- `host` - Volunteer computers
- `workunit` - Work units to be computed
- `result` - Individual tasks/results
- `app` - Applications
- `app_version` - Platform-specific app versions

## Testing Changes

### Client Testing

```bash
# Run the client in standalone mode
cd client
./boinc --dir test_data --redirectio
```

### Server Testing

Set up a test project using the `make_project` script:

```bash
cd tools
./make_project --test_app --url_base http://localhost test_project
```

## Debugging Tips

1. **Client Debugging**: Use `--redirectio` to see log output in console
2. **Server Debugging**: Check logs in `project/log_*/`
3. **Database Issues**: Use `mysql` command-line to inspect tables
4. **Network Issues**: Check firewall settings and `cc_config.xml`

## Common Pitfalls

1. **Build Issues**: Make sure all dependencies are installed
2. **Version Mismatch**: Client and server versions should be compatible
3. **Database Migrations**: Run `update_versions` after schema changes
4. **Platform Differences**: Test on multiple platforms if changing core code

## Resources

- **Official Docs**: https://boinc.berkeley.edu/
- **Developer Docs**: https://github.com/BOINC/boinc/wiki
- **Forum**: https://boinc.berkeley.edu/dev/
- **Issue Tracker**: https://github.com/BOINC/boinc/issues

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Test thoroughly on relevant platforms
5. Commit with clear messages
6. Push and create a pull request

## When Working on Issues

1. **Read the issue carefully** - Understand the problem/feature request
2. **Reproduce the issue** - If it's a bug, reproduce it first
3. **Search the codebase** - Use grep/find to locate relevant code
4. **Check related code** - Look at similar implementations
5. **Test your changes** - Build and test on appropriate platforms
6. **Document your changes** - Update docs if needed

## Architecture Notes

### Client Architecture

- Event-driven state machine
- Manages multiple projects and tasks
- Handles scheduling, download/upload, computation
- Communicates with servers via HTTP/HTTPS

### Server Architecture

- Modular design with separate processes
- `feeder` - Feeds work to schedulers
- `transitioner` - Handles result validation
- `file_deleter` - Cleanup old files
- `validator` - Validates results
- `assimilator` - Processes validated results

## Security Considerations

- **Input Validation**: Always validate user input
- **SQL Injection**: Use prepared statements
- **File Access**: Validate file paths carefully
- **Network Security**: Use HTTPS where possible
- **Code Signing**: Applications should be signed

## Performance Considerations

- **Database**: Index frequently queried columns
- **Network**: Batch operations where possible
- **Client**: Minimize CPU usage when idle
- **Server**: Use database connection pooling

## AI Assistant Guidance

When working with this codebase:

1. **Search before creating** - BOINC has a large codebase; similar code likely exists
2. **Maintain compatibility** - Consider backward compatibility with existing projects
3. **Platform awareness** - Changes may need platform-specific code
4. **Test broadly** - Changes can affect client, server, or both
5. **Consult documentation** - Check wiki and docs before major changes
6. **Ask for clarification** - If requirements are unclear, ask the user

## Quick Reference Commands

```bash
# Clean build
make clean && make

# Rebuild configure
./_autosetup

# Check for common issues
./configure --help

# View client log
tail -f /var/lib/boinc-client/stdoutdae.txt

# Database access (server)
mysql boinc_project

# Server status
cd project/bin
./status
```

---

**Note**: This document is meant to help AI assistants understand the BOINC project structure and development workflow. It should be updated as the project evolves.
