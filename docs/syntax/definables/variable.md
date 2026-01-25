# Variables

Variable are convenient way of storing values.

## Define a variable

To define a variable, you firstly tell which name you want it has, then you specify its value, as follow:

```fly
|| "my_var" will have the value "true"
my_var: true;
```

### Constants

Sometimes you want to define a variable as a constant, to tell this to flylang, you can use the double ":" notation:

```fly
MY_CONSTANT:: "Here is the value !";
```

## Operations

Let's image you have a variable named `life` that you want to increment. Surely you're able to use `life: life - 2`, but you can also use the following syntax:

```fly
life -: 2;

|| And this can be used with any operations
sub: 20;
sub-:10; || toggler = 10

pow: 2;
pow **: 2**2; || pow = 8
```
