# muslimdb
A non-agnostic or non-atheist database, which means a theistic (Muslim) database.

## TODO
- parser for options.
- database options.
- set magic byte.
- read scripts from file.
- shell library.

![muslimcat](https://github.com/user-attachments/assets/2457ceb2-7920-42d9-acd1-34cc65caca28)

# Usage.
```sh
muslimdb "x = hello ; y = world ; z = val(x) val(y)" "file.mdb"
muslimdb "z" "file.mdb"
```

## Smol usefull scripts.

- simple template example.

```
num = 0
str = "this is string"
arr = [1, 2, 3, 4]
```

- using with bash/dash.
```sh
. "lib/bash.sh" # or dash.sh
if mlib_ifeq "num" "0" "file.mdb"
then
	echo "num is equal 0"
fi
```

# Contributing.
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

# License
[GPLv3](https://choosealicense.com/licenses/gpl-3.0/)
