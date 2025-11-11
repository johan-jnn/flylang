# Scopes

Scope are a way to save the emplacement of a statement.
After defining a [function's scope](./definables/function.md#define-the-scope), [loop's scope](./instructions/loops.md#define-the-scope) or even [condition's scope](./instructions/conditions.md#define-the-scope), you can reference them in multiple way :

The syntax of a scope is really simple :

## Named scope

When you define a scope, this one must be a named one.

```fly
@myscope
```

Where "`myscope`" must follow the variable naming rules
