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
* Let's first get the raspberry pi stood up and see what that's like
* Then take a look at the ada fruit items to figure out how to best connect everything together
* 