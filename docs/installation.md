# Installation
Currently installation is only supported on Unix-based OS's
## Requirements
- [Cargo](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)


## Using Curl
```sh
curl -s https://raw.githubusercontent.com/orionbell/yrnu/main/install.sh | sh
```
## Using Wget
```sh
wget -qO - https://raw.githubusercontent.com/orionbell/yrnu/main/install.sh | sh
```
## Using git
1. Clone the repository
```sh
git clone -depth 1 https://github.com/orionbell/yrnu.git

```
2. cd into the project directory
```sh
cd yrnu

```
3. Comment the first two lines in the `script.sh` file
```sh
# printf "Downloading..."
# (git clone --depth 1 https://github.com/orionbell/yrnu.git -q && printf "done!\n") || (printf "failed!\n" && exit 1)
# cd ./yrnu
```
4. Run the install script
```sh
sudo install.sh
```
