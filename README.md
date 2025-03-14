## 🌎 What is Linker?

Linker is a tiny and simple link shortening service configurable with `.toml` files.
The main goal is to provide maximum convenience in creating redirects for your links, so Linker does not use complex interfaces, constructs, configs or anything else.


## 📦 How to set up?

Configs are written as `.toml` files.
They are located in the `links` and `refs` directories.
In `links` are directly the redirects you want to provide, and in `refs` are your referrals to track the number of uses.
We'll give an example below and then everything will fall into place.

```toml
# /links/gh.toml

[config]
path = "gh"
to = "https://github.com/orewaee"
```

```toml
# /refs/website.toml

[metrics]
uses = 0
```

This config will cause route `/gh` to redirect the request to the specified address, and if `?ref=website` is appended to it, the counter will be incremented.
