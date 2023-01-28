<div align="center">

# kubetui

#### A terminal-based UI for managing K8s clusters and resources, written in Rust 🦀

<br>
</div>

![preview](https://user-images.githubusercontent.com/47827286/215286233-ae454cda-487b-46a0-9654-cb79e8cdced9.png)

## Features

As this is a hobby project of mine, I'm not aiming to implement all the features that the official `kubectl` tool has. Instead, I'm focusing on the features that I personally find most useful. The following features are currently implemented and/or planned:

- **General cluster information:**
    - [x] Display current context
    - [ ] See version for k8s and local `kubectl`
- **Pods:**
    - [x] List all pods in cluster
    - [ ] See logs for any given pod
    - [ ] List all pods in current/specific namespace
- **Nodes:**
    - [x] List all nodes in cluster
- **Namespaces:**
    - [ ] List and switch between namespaces
- **Contexts:**
    - [ ] List and switch between contexts

## Installation

When the project is ready for release, pre-built binaries will be available for download either from the [releases page](https://github.com/elvejohansson/kubetui/releases) or from the various package managers. For now, as the project is still in its early stages, you will have to build it from source.

### Prerequisites

You will need to have `kubectl` installed and configured locally to access your cluster. You can find installation instructions [here](https://kubernetes.io/docs/tasks/tools/install-kubectl/).

As you might have guessed, you will also need to have Rust installed. You can find installation instructions [here](https://www.rust-lang.org/tools/install).

### From source

```bash
git clone https://github.com/elvejohansson/kubetui.git

cd kubetui

cargo install --path .
```

##  Usage

```bash
kubetui
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
