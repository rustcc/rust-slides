title: Spirit
class: animation-fade
layout: true

<!--
This work is licensed under the Creative Commons Attribution-ShareAlike 4.0
International License. To view a copy of this license, visit
http://creativecommons.org/licenses/by-sa/4.0/ or send a letter to Creative
Commons, PO Box 1866, Mountain View, CA 94042, USA.
-->

.bottom-bar[
  {{title}}
]

---

class: impact

# {{title}}

.left-column[
Michal Vaner (michal.vaner@avast.com)
]

---

# About me

* Michal Vaner
* https://github.com/vorner
* Work in Avast
* Rust Evangelist

---

# [Spirit](https://crates.io/crates/spirit)

* Applications contain „meat“ and „boilerplate“
* Spirit helps with the boilerplate
  - Configuration reloading + application
  - Interaction with command line
  - Common signal handling
* Flexible
  - Allows deviating from recommended patterns
  - Allows extending
* Still a bit under construction
  - Especially docs
  - I'm happy for any help

???

* Introduce self
  - About Avast
  - Maintains some crates
* Example with a hello world
  - Needs configuration, like ports and what to say
  - Needs logging
  - Needs daemonization
* Suddenly takes much longer to develop
* Show the code
