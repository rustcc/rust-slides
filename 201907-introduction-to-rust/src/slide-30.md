
# Struct implementations: wrapping up


* no `self` argument: associated functions, like the `new` "constructor"

* `&self` argument: can use the values of the struct, but not change them

* `&mut self` argument: can modify the values

* `self` argument: will consume the value, which will move


