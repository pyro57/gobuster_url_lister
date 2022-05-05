# gobuster_url_lister
Takes raw gobuster output and pulls out all the urls.


# Usage
gobuster_url_lister /path/to/gobuster/output/file /path/to/save/urls/

# Building and installing
```
git clone https://github.com/pyro57/gobuster_url_lister.git
cd gobuster_url_lister
cargo build
sudo cp ./target/debug/gobuster_url_lister /usr/bin/
```
