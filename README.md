# Holochain Basic Chat

A simple chat app designed to get new users up, running and developing on Holochain

![Alt text](doc/screen.png?raw=true)

# Get started using nix-shell

Nix shell is a way to set up a dev environment that is easy to replicate on different machines. See https://github.com/holochain/holonix for more info.

- Install nix tooling on mac/linux
```
  curl https://nixos.org/nix/install | sh
```

- Start a new nix-shell. This uses the local `default.nix` file to set up the correct versions of the holochain binaries and build tools.

```
  nix-shell
```

- You should end up with a `[nix-shell:` prompt in your shell.

## Building from Source

### Holochain DNA

There are some helpers scripts in the root package.json for building, running and testing the holochain code.

```
npm run hc:build
```

or use the CLI directly

```
mkdir -p dna
cd dna-src
hc package -o ../dna/peer-chat.hcpkg
```

*Be careful!* If you are trying to network with other agents it is best to distribute a single compiled dna file. Any differences in compiler configuration may lead to the DNA hash being different and the nodes will not be able to communicate.

### UI

```
cd ui-src
npm install
npm start
```

## Built With

* [Holochain](https://developer.holochain.org/)
* [React](https://reactjs.org/)

A huge acknowledgement to Pusher for providing an open source React chat UI (https://github.com/pusher/react-slack-clone)

## Authors

* **Willem Olding** - *Initial work* - [willemolding](https://github.com/willemolding)

## License

This project is licensed under the GPL-3 License - see the [LICENSE.md](LICENSE.md) file for details
