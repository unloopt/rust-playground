# Rust playground

[![Built with Devbox](https://jetpack.io/img/devbox/shield_galaxy.svg)](https://jetpack.io/devbox/docs/contributor-quickstart/)

A playground repository for learning the Rust programming language, exploring modern design patterns and cloud-native architectures and technologies, and experimenting with development workflows, in the context of Rust and its ecosystem. This repository is intended to be a reference repository for all the future Rust projects.

## Getting started

### Local development

#### Windows

1. Install WSL2 with the default Ubuntu distribution.
   
   ```
   wsl --install -d Ubuntu
   ```
3. Install Devbox.
   
   ```
   curl -fsSL https://get.jetpack.io/devbox | bash
   echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
   ```
5. Install Direnv.

   ```
   curl -sfL https://direnv.net/install.sh | bash
   ```
7. Configure Git.

   ```
   sudo apt-get install git
   git config --global user.name "Your Name"
   git config --global user.email "youremail@domain.com"
   git config --global pull.ff only
   ```

### Cloud development

Develop in Devbox Cloud from your browser by visiting [devbox.sh](https://devbox.sh), or click the button below to launch a quickstart shell.

[![Open In Devbox.sh](https://jetpack.io/img/devbox/open-in-devbox.svg)](https://devbox.sh/github.com/unloopt/rust-playground)
