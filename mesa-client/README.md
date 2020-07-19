# Spreadsheet UI

This is the browser-based UI for Mesa X.

## Getting started

Getting started with the web UI

* Install stuff with npm: `npm install`
* Compile the gRPC stuff: `npm run protoc`
* Start things up: `npm start`

Note... before doing `npm start`, run the `helloworld-server` from Rust and the 
gRPC-Web to gRPC proxy in the `3rd_party_bin` directory.


## Libraries

[React Datasheet](https://github.com/nadbm/react-datasheet)


## Set-up and errors

You will need to have recent (e.g., NodeJS 12.x and npm 6.x) installed
to run the client. If you're on Ubuntu 18.04 or such, please see
https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-18-04

If you get `Error: ENOSPC: System limit for number of file watchers reached` execute
`echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf && sudo sysctl -p`
