# Modifiers

Modifiers are symply [functions](./function.md) that takes as the first argument a [definable](./readme.md) element (variable's value, function's reference, class' constructor), and as the second one the arguments passed to this definable, and does things with it.

The modifier's behavior is not the same depending on what definer is modifier :

```fly
fn debug(def, args,
  std.out(def, args);
  return def;
);
```

## Variables definers

> If the modifier is used for a variable, the modifier is directly called.
> The definer's first argument will be a string contening the variable's name, and the arguments is firstly a boolean indicating if the variable is set as constant and the second is the variable's value.

```fly
#(debug)
my_var: true;
assert(my_var = "my_var");
```

## Functions definers

First argument is the function reference and the second are the parameters passed to this function.

```fly
#(debug)
fn myFunc();
assert(my_func() = my_func);
```

## Class definers

First argument is the reference to the class and the second one is the arguments passed to the constructor.

```fly
#(debug)
cs MyClass();
assert(new MyClass() = MyClass);
```
