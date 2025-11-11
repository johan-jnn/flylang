# Definables

A definable is an specific type of instruction that actually returns a just-created element.
For exemple, when you create a [function](./function.md), that created function will be returned from the create instruction.

This means that you can [call a function](../expressions/call.md) with a function that you create in the calling function's arguments (if the function accept a function is its arguments).

Also, any definable element can be [modified by a modifier](./_modifiers.md). By doing this you can easily edit the behavior of any definables.

## List of valid definables

- [Variables](./variable.md)
- [Functions](./function.md)
- [Classes](./class.md)

## Naming rules

When you create definables, you will surely naming them. This naming is pretty cool by allowing you a pretty good range of characters. But there is some rules and conventions.

### Language's naming rules

- A definable cannot be named as a language's keyword
  - `cs`
  - `fn`
  - `while`
  - `unless`
  - `if`
  - `else`
  - ...
- A definable first's character must be a alphabetical character.

### Conventions (optionnal)

- [Variables](./variable.md) & [properties](./class.md#properties): [snake_case](https://laconsole.dev/blog/cases-camel-pascal-snake-kebab-upper#-snake_case)
  - [Constants](./variable.md#constants): [UPPER_CASE](https://laconsole.dev/blog/cases-camel-pascal-snake-kebab-upper#-upper_case)
- [Functions](./function.md) & [methods](./class.md#methods): [camelCase](https://laconsole.dev/blog/cases-camel-pascal-snake-kebab-upper#-camelcase)
- [Classes](./class.md): [PascalCase](https://laconsole.dev/blog/cases-camel-pascal-snake-kebab-upper#-pascalcase)
