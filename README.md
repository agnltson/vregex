# vregex
A rust regular expression validator tool.

## Example

```rust
mod vregex;

fn main() {
    let regex = "(a+b+cd)*";
    let mut vregex = vregex::Vregex::new(regex);
    let input1 = "abbcdaacd";
    let input2 = "ababc";
    println!("{} is in the language defined by the regex {}: {}", input1, regex, vregex.validate(input));
    println!("{} is in the language defined by the regex {}: {}", input2, regex, vregex.validate(input));
}
```
This code returns:
```console
abbcdaacd is in the language defined by the regex (a+b+cd)*: true
ababc is in the language defined by the regex (a+b+cd)*: false
```
