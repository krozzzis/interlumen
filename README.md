# Simple and dumb ray tracer

<img src="images/preview.git"/>

## Build

```bash
$ cargo build --release
```

## Usage

### Console mode:

Run:
```bash
$ ./target/release/interlumen
```

- `p` - pause/resume animations
- `q` / `ESC` - quit

### GUI mode:

Run:
```bash
$ ./target/release/interlumen gui
```

- `q` - quit


## TODO
- [x] Runs in terminal
- [x] Runs in GUI
- [ ] Saves image to file
- [x] Multithreading
- [ ] Monte-Carlo sampling
- [ ] PBR rendering
- [ ] Loads scene from file
