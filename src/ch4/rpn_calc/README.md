# RPN(Reverse Polish Notation) Calc

Reverse Polish notation (RPN) Calc.

## Example

```
fn main() {
    let src = String::from("1 2 + 3 * ");
    let a = rpn_calc::eval(src).unwrap();
    println!("{}", a); // →9
}
```

```
fn main() {
    let result = rpn_calc::eval_str("1 2 3 * +");
    println!("{}", result); // →7
}
```

