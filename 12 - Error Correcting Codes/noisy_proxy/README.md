# usage
cargo build --release && cat MyVideoFile | ./target/release/noisy_proxy | vlc -

## OSX note
The vlc executable is probably in your `VLC.app`. You'll probably want to do:

```bash
alias vlc='/Applications/VLC.app/Contents/MacOS/VLC'
```

See https://wiki.videolan.org/Mac_OS_X/#Command_line for more details.
