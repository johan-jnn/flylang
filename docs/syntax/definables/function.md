# Function

Functions are a set of instructions that takes some optional entry, does things with them and optionnaly return a value.

In flylang, the syntax for the functions is really simple:

```fly
fn my_function(
  || function's code
);
```

That's all.

## Arguments

If you want to include some arguments to your function, just name them before the function's code and separate them with a comma :

```fly
fn my_function(a, b,
  || function's code
);
```

## Anonymous functions

Function can be anonymous. To make a function anonymous, just do not specify a name for it.

```fly
fn(
  || anonymous function's code
);

fn(a, b, c,
  || and this time with arguments
)
```

## Returns

To return value from a function, use the [`return` breaker](../_breakers.md#return)

## Define the scope

To define the function's scope, place it before the oppenning block (`(`).

> Note: The scope must be a [named scope](../_scopes.md#named-scope).

```fly
fn @myscope(

);

|| With a named function
fn my_function @myscope(

);
```

## Modify a function

If you want to use [modifier](./_modifiers.md) for your function, you can include them above its definition :

```fly
#(myModifier, anotherOne)
fn(

);
```
