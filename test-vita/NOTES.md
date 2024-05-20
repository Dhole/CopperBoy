Build
```
cargo +nightly vita build vpk -- --release
```

Build, upload to vita & run
```
cargo +nightly vita build eboot --update --run -- --release
```

Loop
```
echo "destroy" | nc 192.168.0.129 1338 && VITA_IP=192.168.0.129 cargo +nightly vita build eboot --update --run -- --release
```

With another terminal:
```
while [ true ]; do
  date && nc -l 8888
done
```
