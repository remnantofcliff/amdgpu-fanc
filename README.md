# amdgpu-fanc

## What it does?

amdgpu-fanc is a lightweight and minimal utility that can modify the fan speed
of an AMD gpu in a linux system. It requires the amdgpu kernel module to be
loaded. Requires administrator privileges since the utility interacts with a
kernel driver.

## How to use it?

1. Create a config file at your desired path that contains lines with the
following format: `temperature => fan_percentage` e.g. `10 => 50`. The example
translates to "at 10°C, fan speed should be at 50%"

2. Run `amdgpu-fanc find` to print out paths to valid gpu hwmon directories.
Copy the path of the gpu you want to control.

3. Run `amdgpu-fanc run -c path/to/config -p path/to/gpu` and you are set!

## How does it work?

amdgpu-fanc uses the
[linux kernel amdgpu driver's interface](https://www.kernel.org/doc/html/latest/gpu/amdgpu/thermal.html)
to control the fans. First it enables manual control of the fans by writing '1'
to the `pwm1_enable` file. It reads the file `temp2_input` or `temp1_input` if
the former is unavailable for the current temperature. Then it transforms it to
a pwm value and writes it to the `pwm1` file.

When the program closes, it tries to re-enable automatic fan control. If it is
unable to do so, a warning message gets printed to stderr.

## Instructions

Install [rust](https://www.rust-lang.org/tools/install) via the rustup method.
Then:
```
git clone https://github.com/remnantofcliff/amdgpu-fanc.git
cd amdgpu-fanc
cargo build --release
```
The executable will be placed in ./target/release/

See section [How to use it?](#h2-how-to-use-it?) for how to use the generated
executable.
