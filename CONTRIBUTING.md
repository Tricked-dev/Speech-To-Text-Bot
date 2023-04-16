# Development

This document outlines the development process for Speech to Text Bot.

## To-Do

- [ ] decode the file or at least dont write a ogg to disk in order to use ffmpeg on it to change bitrate & filetype
- [ ] improve usability
- [ ] Improve speed
- [ ] create a new logo

## Development

To run the bot locally, you will need to have Rust and Cargo installed on your computer. Once you have those installed, follow these steps:

- Clone the repo
- Navigate to the root directory of the project
- Download the whisper model

```sh
# This runs the bin/download_model file
cargo download_model
```

- Add a TOKEN variable to your shell and run `cargo run`

```sh
TOKEN=.... cargo run
```

## Deployment

To deploy Speech to Text Bot to a production environment, we recommend using a cloud hosting service with good CPU's in order to be able to transcribe faster:

- follow the steps from above
- run `cargo run --release` instead of `cargo run` in order to run the faster release version

## Contributing

If you would like to contribute to Speech to Text Bot, please follow these guidelines:

- Fork the repository to your own account.
- Create a new branch for your changes.
- Make your changes and test them locally.
- Submit a pull request with a clear description of your changes.
