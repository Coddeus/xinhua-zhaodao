![](https://img.shields.io/github/actions/workflow/status/Coddeus/xinhua-zhaodao/rust.yml)  ![](https://img.shields.io/github/v/release/Coddeus/xinhua-zhaodao)

# xinhua-zhaodao

Finds simplified Mandarin words containing a given radical.

## General info
### Radicals
The given radical can be any Unicode character (like ⿕，加 or 丿).  
Thus it is not limited to any radical classification, like Kangxi Radicals.  

Note that it will look for the exact character that was passed in - it doesn't recognize alternative forms (yet).  
Therefore no derivative of an alternative form will be found (水 has 36 results, but 氵 has 765).

### Output
The derivatives are characters which exist in the xinhua dictionary.

---
## Setup
Using Cargo, it should work instantly when cloning.  

Default repo structure : 
```
xinhua-zhaodao
├─ .github
│   └─ workflows
│       └─ rust.yml
├─ src
│   ├─ data
│   │   └─ hanyu.db
│   └─ main.rs
├─ .gitignore
├─ cargo.toml
└─ README.md
```
Main checks: 
- `sqlite` is in Cargo.toml dependancies  
- `hanyu.db`'s path is the same as the one in main.rs, line 5.

---
### Uses
This tool may be useful for: 
- Those who want to discover new characters (and remember them) through radicals. (original purpose)
- Those who prefer scrolling thousands of Chinese characters (because it's beautiful) instead of social media content.

---
### License
[CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) 

---
Coddeus  
2023
