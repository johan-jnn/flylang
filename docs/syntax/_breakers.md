# Breakers

Breakers are special keyword that will stop a specific process.

## Return

This is used to stop the execution of a [function](./definables/function.md).
It needs to be used inside the function (you cannot stop a function outside itself).

The syntax is really simple :

```fly
fn(
  return;
);
```

### Returning a value

You may want to return a value from this function.
To do this, just pass the value after the return keyword :

```fly
fn(
  return 0;
);
```

### Multiscope return

Finally, if you have nested functions, you can return at multiple's level function by specify [the scope](./_scopes.md) just after the `return` keyword (and before the value) :

```fly
fn(
  fn killer(
    || Note that the scope can also be named ("return @<name>")
    return @-1 true;
  );

  killer();
  std.out("This will never be printed.");
);
```
