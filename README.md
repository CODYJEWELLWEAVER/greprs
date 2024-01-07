# greprs v0.3.0
[![Rust](https://github.com/CODYJEWELLWEAVER/greprs/actions/workflows/rust.yml/badge.svg)](https://github.com/CODYJEWELLWEAVER/greprs/actions/workflows/rust.yml) [![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://opensource.org/licenses/) 

---
Implementation of grep using rust. I decided it would be fun to try to fully implement the functionality of grep using Rust. As of v0.3.0, greprs supports basic string and pattern searches of multiple files and currently supports the options: case-sensitive, invert match, and output line counting. You can find more information about the original grep [here](https://www.gnu.org/software/grep/).

### Documentation
##### Build and Install (Linux)
Clone git repo and build
```bash
git clone https://github.com/CODYJEWELLWEAVER/greprs.git
cd greprs
cargo build & cargo test
```


You can install the binary to your path by using 
```bash
cargo install --path <path/to/greprs>
```

##### Usage
greprs can be run using the following syntax
```bash
greprs [-q:<query>,...] [<path/to/file>,...] [<option>,...]
```
You can use the following for single query searches
```bash
greprs query [<path/to/file>,...] [<options>,...]
```
Note that to use this syntax the query should be before all files and options.

You can specify multiple queries with the following notations
```bash
-q:<query> -q:<query> ... or -q:<query>:<query>:...
```

###### Usage Example
```bash
greprs -q:is -q:Thee res/test/*
Searching for: ["is", "Thee"]
In: <res/test/haiku.txt> <res/test/poem.txt> 

<res/test/poem.txt>     And often is his gold complexion dimm'd;
<res/test/poem.txt>     Nor shall death brag thou wander’st in his shade,
<res/test/poem.txt>       So long lives this, and this gives life to thee.

<res/test/haiku.txt>    This world of dew
<res/test/haiku.txt>    is a world of dew,
```

##### Options
##### Info Options
###### Help Message
```bash
greprs help 
# or
greprs --help
```
*Displays help message to user.*

###### Version Info
```bash
greprs version
# or
greprs --version
```
*Prints greprs version.*

##### Search Options
###### Case Sensitive
```bash
-i 
```
```bash
--ignore-case
```

###### Invert Matching
```bash
-v
```
```bash
--invert-match
```
*Reverses matching logic, lines that do not match any patterns will match. (off by default)*

###### Word Matching
```bash
-w
```
```bash
--word-match
```
*Patterns only match when they form whole words, i.e. they are surrounded with word boundaries.*

##### Output Options
###### Count Matching Lines
```bash
-c
```
```bash
--count
```
*Displays the number of matching lines in each file.*

---
##### Licensed under [GPL v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html#license-text)

---
##### Contact
* Cody J. Weaver - cody.weaver@colorado.edu 
