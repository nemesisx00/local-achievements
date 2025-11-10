<h1 align="center">Local Achievements</h1>

<div align="center" width="100%">
	<img alt="GitHub" src="https://img.shields.io/github/license/nemesisx00/local-achievements" />
	<!-- <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/nemesisx00/local-achievements/rust.yml" /> -->
	<img alt="GitHub contributors" src="https://img.shields.io/github/contributors/nemesisx00/local-achievements" />
	<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/nemesisx00/local-achievements" />
</div>

<p align="center">
Local Achievements is a desktop application for viewing, tracking, and backing up achievements data.
</p>

&nbsp;

## Downloads

When Local Achievements releases, you will be able to download it from the Releases section.

## Table of Contents

- [Supported Platforms](https://github.com/nemesisx00/local-achievements#supported-platforms)
- [Inside Local Achievements](https://github.com/nemesisx00/local-achievements#inside-local-achievements)
- [Getting Started](https://github.com/nemesisx00/local-achievements#getting-started)
	- [Requirements](https://github.com/nemesisx00/local-achievements#requirements)
	- [Compiling, Running, and Testing](https://github.com/nemesisx00/local-achievements#compiling-running-and-testing)

## Supported Platforms

- [ ] [Epic Game Store](https://store.epicgames.com)
- [ ] [GOG](https://gog.com)
- [ ] [Playstation Network](https://www.playstation.com/playstation-network)
- [x] [RetroAchievements](https://retroachievements.org)
- [x] [RPCS3](https://rpcs3.net)
- [x] [Steam](https://store.steampowered.com)
- [ ] [XBox Live](https://www.xbox.com/live)

## Inside Local Achievements

Local Achievements is build in [Rust](https://rust-lang.org), relying on [Freya](https://freyaui.dev) to define the GUI.

Local Achievements is built for Linux first. However, it should be compatible with any platform that [Freya](https://freyaui.dev) supports.

## Getting Started

### Requirements

In order to compile Local Achievements, you will need the following tools installed:

- [Rust](https://rust-lang.org/learn/get-started)

Check [Freya](https://freyaui.dev) for any potential additional requirements.

Now you're ready to fork and clone the repository.

### Compiling, Running, and Testing

The project is primarily managed via [Cargo](https://doc.rust-lang.org/cargo). If you're new to [Rust](https://rust-lang.org), you should take some time to become more familiar with it.

Open a command line interface and navigate to the project's root. Run the project's tests by entering the following command:

```
cargo test
```

There are some tests which are marked to be ignored. These require additional environment variables to be set before the can run properly. See the individual test's doc comments for more information.

You can also run the release build with the following command:

```
cargo run --release
```

> [!IMPORTANT]
> At the moment, there is an issue with [Freya](https://freyaui.dev) where running the debug build will result in excessive lag in the UI. It still functions but, for the time being, it is recommended to only run the release build of Local Achievements.


If you want to generate the executable for the release build, use the following command:

```
cargo build --release
```

Once the build process is complete, you will find the resulting `local-achievements` executable in the `target/release` directory.
