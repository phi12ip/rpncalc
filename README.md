# rpncalc

A Reverse Polish Notation calculator modeled after the HP 50g. 

<img width="50px"  src="https://cdn.shopify.com/s/files/1/0424/3639/5165/products/HP1_1024x1024@2x.jpg" alt="HP 50g Calculator"/>

## Usage

To run the cli:

```sh
cargo run
```

To run the gui:

TODO: gui not implemented yet


## Spec

Operations Currently Implemented:

```
 Op           Function
----         ----------
 +              Add
 -              Subtract
 *              Multiply
 /              Divide
 pow            Power
 root           top of stack is exponent
 f              Floor
 c              Ceiling
 d              Duplicate
 clear          Clear All
 rm             Pop
 
```

To implement:

- sin - Sin function
- cos - Cosine function
- tan - tangent function
- sqrt - square root
- inv - inverse function
- enter - enter input or duplicate stack if no input * ( satisfied by d )
- spc - space, create space between a multi-input string
- dot - the dot for decimals ( not needed in the cli )

This will make a 4 x 6 grid of buttons needed to start the gui impl.
