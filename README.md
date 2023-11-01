# Mouse

## Name

In the factual history book of Douglas Adams, The hitchikers guide to the galaxy, the mice are the gatherers of information and calculators for the problem "The answer to life, universe and everything".
Hence mouse :D.

## Whut?

Mouse is a rust based facts gatherer that returns a list of facts of the host, it can be completely ran as user at this time and it's intended to stay that way.
Mouse does not alter the system, just gathers information and returns it in json or yaml (default). At this time I can only test on x86_64 and aarch64, so those are the only supported platforms for now.

## Why?

Yes we have ohai, facter, osquery,... 
But those won't let me learn Rust. 

## How?

Download the release or build locally and execute. Use --help for information.
example: `mouse -g system -o json` will output system information in json.

At this time with the latest release (v0.1.4) there are 4 gatherers:

* env
* ipaddr
* iproute
* system


## Contribute?

Why not, but don't feel obligated.

If you want to contribute, feel free to send your patches to mouse-patch@sejo-it dot be. 

## It doesn't work!

Aww, too bad, well you can always submit an issue or fix it yourself. It's open source afterall, isn't it?

## I have more questions:

matrix: @sejo:matrix.sejo-it.be  
discord: sejoit (legacy: sejo#5402)  
email: jochen@sejo-it.be
