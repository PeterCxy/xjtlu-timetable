Yet Another XJTLU Class Timetable Exporter
===

But without the need of passwords of the school account.

This exporter works purely in the frontend, which parses rich text pasted into a `contenteditable` element. By doing so, we can eliminate most of the attack surface of acquiring users' passwords and sending then through a server.

Served version available at <https://angry.im/xjtlu-timetable>. Click on this link if you only want to use this program.

Written in Rust, compiled with `asmjs-unknown-emscripten` target. This is my first attempt to write frontend code with Rust.

Building
===

1. Install Rust toolchain, and the `asmjs-unknown-emscripten` target.
2. Install [cargo-web](https://github.com/koute/cargo-web)
3. Clone this project, run `cargo web build`
4. You could preview the web page by running `cargo web start`
5. To build release version, use `sh deploy.sh` (not runnable under Windows), output will be in `target/deploy`