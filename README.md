# imgcreator
Imgcreator is a cross-platform CLI program that quickly creates empty .img disk images. These disk image files can be used for a range of reasons, from virtual machine disks, to storing data, to creating ROMs, to store backups, to store entire disk data, etc

# Download
You can download the latest versions of imgcreator [here](https://github.com/spacebanana420/imgcreator/releases)

Once imgcreator comes out, it will be compiled for the following systems:
- Linux (x86_64) (both glibc and musl)
- NixOS (x86_64) (glibc) (coming version 3.0)
- Windows (x86_64)

If the regular Linux binary doesn't work, try the musl version

# Compiling from source
Imgcreator will work on any operative system and architecture that supports Rust's compiler

Install cargo, Rust's package manager, and open a terminal in the project's directory and run ``` cargo build -r ```
