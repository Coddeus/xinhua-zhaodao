![](https://img.shields.io/github/actions/workflow/status/Coddeus/xinhua-zhaodao/Rust-OS-check.yml)  ![](https://img.shields.io/github/v/release/Coddeus/xinhua-zhaodao)

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

### Database
The used database, hanyu.db, is an edited merge of a 新华字典(xinhua zidian) database, a decomposition database and a frequency list. (see  [>Thanks](#thanks))  
Note that most of the DB isn't used yet, but all tables have a purpose for next releases.


## Setup
Using Cargo, it should work instantly when cloning.  

Default repo structure : 
```
xinhua-zhaodao
├─ .github
│   └─ workflows
│       ├─ release_bin.yml
│       └─ Rust-OS-check.yml
├─ data
│   └─ hanyu.db
├─ src
│   └─ main.rs
├─ .gitignore
├─ Cargo.toml
└─ README.md
```
Main checks: 
- `sqlite` is in Cargo.toml dependancies  
- `hanyu.db`'s path is the same as the one in main.rs, line 5.


## Uses
This tool may be useful for: 
- Those who want to discover new characters (and remember them) through radicals. (original purpose)
- Those who prefer scrolling thousands of Chinese characters (because it's beautiful) instead of social media content.


## Thanks
Data for hanyu.db:
- [Characters decomposition](https://commons.wikimedia.org/wiki/Commons:Chinese_characters_decomposition) on Wikimedia Commons
- [Computerized 新华字典(xinhua zidian) scraper](https://github.com/pwxcoo/chinese-xinhua) on Github
- [Characters frequency](https://lingua.mtsu.edu/chinese-computing/statistics/char/list.php?Which=MO) from Jun Da's website


## License
[CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) 


Coddeus  
2023
