---
title = "beaglebone black vocoder"
tags = ['c', 'embedded', 'dsp']

[links]
'source code' = 'https://github.com/group03-real-time/swe660-vocoder'
---

This is a vocoder (with built-in synthesizer) I built for my class SWE 660 (Software Engineering for Real-time Embedded Systems). It runs in real time on a BeagleBone Black, performing DSP on
top of embedded Linux (with PREEMPT_RT) while processing data using the on-board
PRU (bare metal).

The DSP itself is implemented with fixed-point math in order to meet the realtime
requirements.

The audio input is through the on-board ADC, while the output is through I2S. It also
supports a 24-button keyboard for playing the synthesizer and some analog knobs
for changing the synthesizer's voice.