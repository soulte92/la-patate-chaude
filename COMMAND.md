### Command to execute the server
##### Go in the good directory who is "distribution/macos" for MacOS and "distribution/windows" for Windows and run :
```bash
./server
```
---
### Command to execute the server in debug mode 😖
```bash
./server --debug
```
---
### Command to execute the server in debug mode with recover secret challenge
```bash
./server --game recover-secret --round-time 20 --round 3 --complexity 3 --debug --monitor
```
---
### Command to execute the server in debug mode with recover md5-hashcash challenge
```bash
./server --game md5-hash-cash --round-time 20 --round 3 --complexity 3 --debug --monitor
```
---
### Command to execute the client 😄
```bash
cargo run --bin src
```