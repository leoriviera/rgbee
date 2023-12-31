# rgbee

## TODO

- [ ] Improve documentation
- [ ] Improve error handling in Rust

sudo apt install libasound2-dev

Build image for this

- Add user to `audio` and `gpio` groups
- On every reboot, create loopback device with `sudo modprobe snd-aloop`
- Install Shairplay and NQPTP
- Set volume to 100 (amixer sset 'PCM' 100%)

Add to boot config

- `dtparam=spi=on`

Start command is `shairport-sync -v --statistics --output="alsa" -- -d "hw:3,1,1"`

- Some sort of colour correction for LEDs would be useful
- Tune quantiser to get rid of the large amount of white pixels
- Handle when audio is paused
- Switch to running on two threads, one for calulating colour and another for setting it (reduce latency)
- Handle audio overrun (ALSA EPIPE)
- Make Shairplay automatically recover from failure
- Tie iOS volume to brightness of lights
- Fix occasional bright white flash of all LEDs
