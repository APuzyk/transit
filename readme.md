# Transit Rust
Given a yaml configuration file (eventually) will print out a list of stops as well as
the expected wait times.

In addition we have colored the expected arrival times to be blue for normal lines and red for express buses.



# To Do
## Features
* Do we present same order for lines or do we sort based on time to arrival?
* How do we handle errors?  Do we continue to show the last available data?  
* Can we make a pretty way to print a table?

## Code Health
* Need to get some better error handling in here
* Split up the functions a bit this thing is ugly as sin
* Do we need unused vars?  E.g stop point ref and destination display?

## Display Work

### Steps
* ~~Write v1 of the arrival time ui~~
* Set up Raspberry Pi
 * ~~Flash microsd card~~
 * Spin up raspberry pi and set up ssh
 * write down static IPs and the like
* Set up Matrix Display
 * Solder on hat
 * Set up martrix in array
 * Test v1 display
 * see if we need to use ratatui or some other terminal ui library to make this look better
* Sync with Laurel on display and how to update

### Documents
* [Main article on raspberry pi with led displays](https://learn.adafruit.com/raspberry-pi-led-matrix-display/overview)
* [Raspberry pi hat overview](https://learn.adafruit.com/adafruit-rgb-matrix-plus-real-time-clock-hat-for-raspberry-pi)
* [led matrix basics overview](https://learn.adafruit.com/32x16-32x32-rgb-led-matrix/overview)
* [Raspberry pi getting started](https://www.raspberrypi.com/documentation/computers/getting-started.html)

### My Stuff
* Raspberry Pi 3 B+

### Need to get
1. Powersource
