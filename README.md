# rustBasics
Covers the code from rust_by_example and some more examples from different resources diving into each section throughly

### Installation in codespaces or Ubuntu 20.04 
1. check the os with the following command `lsb_release -a`. It should output to
```
No LSB modules are available.
Distributor ID: Ubuntu
Description:    Ubuntu 20.04.6 LTS
Release:        20.04
Codename:       focal
```
2. Update the ubuntu with `sudo apt-get update`. It will update the system repo links for any new/updated packages. 
3. Install rust with following command `sudo apt-get install cargo`. It should Ideally install both cargo and rust. If not you can install **rustc** with `sudo apt-get install rustc`
4. Check versions of cargo and rustc with following commands `cargo --version` and `rustc --version` respectively. 
