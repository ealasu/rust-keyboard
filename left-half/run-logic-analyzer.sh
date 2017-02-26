#!/usr/bin/env bash

sr=10000000

ssh b bash -c "'echo $sr > /sys/devices/virtual/misc/beaglelogic/samplerate'"
ssh b dd if=/dev/beaglelogic of=o.bin bs=1M count=5
scp b:~/o.bin ./
sigrok-cli -I binary:numchannels=8:samplerate=$sr -i o.bin -o o.sr
