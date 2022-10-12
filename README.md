# amdgpu-fanc

## What does it do?

amdgpu-fanc is a lightweight and minimal utility that can modify the fan speed
of an AMD gpu in a linux system. It requires the amdgpu kernel module to be
loaded. It also requires administrator privileges since the utility interacts
with the kernel driver.

## How to use it?

1. Create a config file at your desired path that contains lines with the
following format: `temperature => fan_speed` e.g. `10 => 0.50`. The example
translates to "at 10°C, fan speed should be at 50%". There is also an example
config included in the repository.

2. Run `amdgpu-fanc find` to print out paths to valid gpu hwmon directories.
Copy the path of the gpu you want to control.

3. Run `sudo amdgpu-fanc run -c path/to/config -p path/to/gpu` with sudo
privileges and you are set!

## How does it work?

amdgpu-fanc uses the
[linux kernel amdgpu driver's interface](https://www.kernel.org/doc/html/latest/gpu/amdgpu/thermal.html)
to control the fans. First it enables manual control of the fans by writing '1'
to the `pwm1_enable` file. It reads the file `tempx_input` depending on which 
sensor is used. Then it transforms the temperature to a pwm value and writes it
to the `pwm1` file.

When the program closes, it tries to re-enable automatic fan control. If it is
unable to do so, a warning message gets printed to stderr.

## Disclaimer

Use of this software is entirely your own responsibility. With this software
you may be able to damage your hardware.

## Installation

Install [rust](https://www.rust-lang.org/tools/install) via the rustup method.
Then:
```
git clone https://github.com/remnantofcliff/amdgpu-fanc.git
cd amdgpu-fanc
cargo build --release
```
The executable will be placed in `./target/release/`

See section [How to use it?](#how-to-use-it) for how to use the generated
executable.

## Run at startup

If your system uses systemd, you can use the `amdgpu-fan-control.service` unit
file included in the root of the repository. Note that by default the unit file
runs the executable `/usr/local/bin/amdgpu-fanc` and uses the config file at
`/usr/local/etc/amdgpu-fanconfig`. You'll have to copy the executable and
config files to the aforementioned locations manually or change the paths in
the unit file.

```
sudo cp target/release/amdgpu-fanc /usr/local/bin/amdgpu-fanc
sudo cp example_config /usr/local/etc/amdgpu-fanconfig
sudo cp amdgpu-fan-control.service /etc/systemd/system/
sudo systemctl enable amdgpu-fan-control.service
sudo systemctl start amdgpu-fan-control.service
```
