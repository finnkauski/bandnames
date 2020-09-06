# Bandname app

This app is designed to basically allow me to store ideas about any good band
album or song names that pop into my head.

## Purpose

The purpose of this app is to allow me to apply web development practices on a
applications that isn't a single weekend project and I can develop it over time
into something showable.

## Under the hood

Currently under the hood we have a Rocket web server and a React front end. The app
was redesigned several times including even trying to make this into a WASM based app
(`Rust` + `Yew`) which I decided not to pursue.

The design was developed (and will be developed) in Figma.

See demo: [here](https://a-bandn.herokuapp.com)

## TODO:

- [ ] Cargo Build scripts
- [ ] Protobuf [from here](https://dev.to/elshize/protobuf-code-generation-in-rust-1e35)
- [ ] Add SCSS
- [ ] Add authentication using JWT and Db
- [ ] Refactor closer to TatriX `Real World` example repo
- [ ] Implement deletion for authenticated users
- [ ] Implement voting for authenticated users
- [ ] Start tracking date of input
- [ ] Pagination for each page
- [ ] Implement `react-router` to handle zooming into names
- [ ] add author, details etc fields
- [ ] Do we still need nightly for proc macro stuff (Rocket)?
