# `ccase` is a command line utility for converting between string cases

Convert strings between snake case, kebab case, camel case, title case, pascal case, and so many more.

## Use Cases

```sh
$ ls ~/roms
donkey_kong_64.z64
kirby_64_the_crystal_shards.z64
super_mario_64.z64
$ ls ~/roms | cut -d '.' -f 1 | ccase -f snake -t title
Donkey Kong 64
Kirby 64 The Crystal Shards
Super Mario 64
```

```sh
$ ls xmas
'Family Christmas photo-1999.png'
Family_christmas_photo-2000.png
family-christmas-photo-2001.png
FamilyChristmasPhoto2002.png
$ for f in xmas/*; do mv "$f" $(ccase -f snake "$f"); done
$ ls xmas
family_christmas_photo_1999.png
family_christmas_photo_2000.png
family_christmas_photo_2001.png
family_christmas_photo_2002.png
```

## Install

You can install using `cargo`:
```
cargo install ccase
```
You can also install from the debian provided in the releases.

## Usage

### `-t, --to <case>`

Transform a string in**to** a certain string case.

```sh
$ ccase -t snake myVarName
my_var_name
$ ccase -t upper my-var-name
MY VAR NAME
$ ccase -t kebab my_var_name
my-var-name
$ ccase -t pascal "my var name"
MyVarName
```

### `-f, --from <case>`

Splits string based on the boundaries associated with that case.  For example, `snake` case splits on underscores `_` and `camel` splits based on lowercase characters followed by uppercase characters `aA`.

```sh
$ ccase -f camel -t snake myVar-Name
my_var-name
$ ccase -f snake -t snake myVar-name
myvar-name
$ ccase -f kebab -t snake myVar-name
myvar_name
```

### `-b, --boundaries <string>`

Specify precisely what conditions should be used for splitting a string into words.  Whatever boundaries are present in the boundary string will be used against the input.  Any example can do, but a nice way to specify is to separate boundaries with a `:`.  For example, `aA:a1:1a` will split based on lowercase followed by uppercase, lowercase followed by and preceded by a digit.

```sh
$ ccase -b "aA:a1" -t kebab myVar1
my_var_1
$ ccase -b "_:-: " -t camel my_Var-name
my-var-name
$ ccase -b "aAa" -t snake myVar
my_v_ar
$ ccase -b "AAa" -t snake MYVarName # special acronym boundary
my_var_name
```

### `-p, --pattern <pattern>`

Transforms the words of the input based on a pattern.  A pattern describes the order of "word cases".  For example, the camel pattern is a lowercase word followed by capitalized words, like in camel case, while the lower pattern is just lowercased words, like in snake case.

```sh
$ ccase -p camel my-var-name
myVarName
$ ccase -p capital my-var-name
MyVarName
$ ccase -p sentence my-var-name
Myvarname
```

### `-d, --delimeter <string>`

Join the words using a delimeter.  By default, the delimeter is an empty string, as used in camel case.

```sh
$ ccase -p camel -d _ my-var-name
my_Var_Name
$ ccase -p capital -d . my-var-name
My.Var.Name
$ ccase -p sentence -d __ my-var-name
My__var__name
```

# How Case Conversion Works

A _case_ can be defined as a _pattern_ joined with a _delimeter_.  Turning a list of words into a certain case happens in two steps.  First, each word is transformed by the pattern.  Then the words are joined together with the delimeter.

_Case conversion_ is splitting a single string into words, then performing the transformation and joining.  Input is split using _boundaries_.

## Step 1: Splitting Input into Words by Boundaries

Boundaries define how to split input.  There are three types of boundaries.  There are character based boundaries like hyphen `-`, underscore `_`, and space ` ` that are not part of the final word list, but split the input around the character.  There are also boundaries like lower-upper (`aA`) or digit-lower (`1a`) that split the input between the characters, and matching characters are included in the result.  Lastly there is the acryonym boundary (`AAa`) that splits between the two uppercase characters (an example of this is in `HTTPRequest`).

In `ccase`, one can specify boundaries using the `--boundaries` option, or they can define a case to convert _from_ using `--from`.  Inputs that are transformed into a certain case will have boundaries that associated with the result.  For example, snake case is joined with underscores, so only underscores will be used as boundaries.  Strings in camel case on the other hand have boundaries between lowercase and uppercase letters, so the lower-upper (`aA`) boundary is used.

## Step 2: Transforming Words with Pattern

A pattern is a series of _word cases_ that describe how a single word is transformed.  For example, the lower word case makes all characters lowercase, the upper word case makes all characters uppercase, and the capital word case makes the first letter uppercase and the remaining lowercase.  For example, snake case uses a pattern where all word should be lowercased.  This is the lowercase pattern.  Camel case uses a pattern where the first word is lowercased, and the remaining words are capitalized.  This is called the camel pattern.

In `ccase`, a pattern can directly specified with `--pattern` or whatever pattern is associated with the case in `--to` option will be used.

## Step 3: Joining Words with a Delimeter

Lastly, words are joined with a string specified as a delimeter.  Cases like snake, kebab, and lower use character strings `_`, `-`, and space respectively.  The delimeter can also be an empty string, like in camel case.

In `ccase`, the delimeter can be specified with `--delimeter` or whatever delimeter is associated with the case in the `--to` option.  If no delimeter is supplied, the delimeter defaults to an empty string.

# Cases, Patterns, and Boundaries

## List of Cases

| Case           | Example              |
| --             | --                   |
| upper          | UPPER CASE           |
| lower          | lower case           |
| title          | Title Case           |
| toggle         | tOGGLE cASE          |
| camel          | camelCase            |
| pascal         | PascalCase           |
| uppercamel     | UpperCamelCase       |
| snake          | snake_case           |
| uppersnake     | UPPER_SNAKE_CASE     |
| screamingsnake | SCREAMING_SNAKE_CASE |
| kebab          | kebab-case           |
| cobol          | COBOL-CASE           |
| upperkebab     | UPPER-KEBAB-CASE     |
| train          | Train-Case           |
| flat           | flatcase             |
| upperflat      | UPPERFLATCASE        |
| alternating    | aLtErNaTiNg CaSe     |
| random         | RANDOm cAsE          |
| pseudorandom   | PseUdO rAnDoM cASe   |

## List of Patterns

| Pattern     | Definition                    |
| --          | --                            |
| upper       | UPPER, UPPER, ...             |
| lower       | lower, lower, ...             |
| capital     | Capital, Capital, ...         |
| sentence    | Capital, lower, lower, ...    |
| camel       | lower, Capital, Capital, ...  |

## List of Boundaries

| Type       | Example String |
| --         | --             |
| Hyphen     | `-`            |
| Underscore | `_`            |
| Space      | ` `            |
| UpperLower | `Aa`           |
| LowerUpper | `aA`           |
| DigitUpper | `1A`           |
| UpperDigit | `A1`           |
| DigitLower | `1a`           |
| LowerDigit | `a1`           |
| Acronym    | `AAa`          |

# Rust Library

`ccase` is a command line utility built on top of a rust library `convert_case`([github](https://github.com/rutrum/convert-case), [crates.io](https://crates.io/crates/convert_case)).  This library and its documentation are the source for the list of cases, patterns, boundaries, and default many behaviors found in this command line utility.

# Changlog

## 0.5.1

- Moved CLI tests to integration tests for compatibility with newer cargo/assert_cmd
- Updated dependencies (clap, assert_cmd, etc.)
- Fixed Nix build by updating lockfiles

## 0.5.0

- Updated `convert_case` dependency to 0.11 (and add `convert_case_extras` as a result)
- Removed case aliases
- Case and patterns must now be specified in all lowercase
- Improve piped input handling
