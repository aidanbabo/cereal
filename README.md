# LC4 Simulator

This project roughly attempts to replicate functionality present in PennSim, a program used by Penn courses to work with a ficticious assembly language, LC4.
This assembly language is used across multiple courses from both angles: writing software targetting LC4 as an ISA and writing hardware compliant to LC4 as an ISA.
Additionally, this project attempts to include a C compiler that targets LC4, which was not included in PennSim when I was at Penn.

This project is a reverse-engineering effort. No decompiling of the original .jar has been done, only observing and mimicking behavior.

The simulator includes many interfaces present in the original PennSim:
- loading and running programs
- scripts
- memory-mapped IO and graphics support
- register and memory viewer
- setting and pausing on breakpoints
- manually stopping a programming, stepping through it and resuming execution
- outputting a trace

Some features to come include:
- dumping memory
- loading hex fileshelp
