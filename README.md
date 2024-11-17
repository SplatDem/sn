# SN (Splat Notes)

Simple utility for notes (because I forget about the kettle too often)

`sn --help` or `sn -h`
```
Usage: sn [OPTIONS]

Options:
  -n, --notify <NOTIFY>        
  -d, --date <DATE>            
  -c, --clear                  
  -i, --important <IMPORTANT>  
  -h, --help                   Print help
  -V, --version                Print version
```

Now you can install it from AUR
```
yay -Sy sn
```

To build from GitHub:
```
git clone https://github.com/SplatDem/sn
cd sn
cargo build --release
cd target/release
sudo cp sn /bin
```
