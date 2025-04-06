
<!-- Markdownify readme used as inspiration -->
<h1 align="center">
  <br>
  <a href="https://github.com/MarshallEvergreen/zapp"><img src="https://raw.githubusercontent.com/MarshallEvergreen/zapp/refs/heads/main/static/images/zapp.webp" alt="Zapp" width="400"></a>
</h1>

<h4 align="center">A fast python public API generator built in rust 🦀.</h4>

<!-- Add badges here -->

<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#credits">Credits</a> •
  <a href="#related">Related</a> •
  <a href="#license">License</a>
</p>

# Key Features

Zapp is a Python tool to automatically generate and maintain python module interfaces, written in Rust.

Zapp is inspired by the [modular monolith](https://www.milanjovanovic.tech/blog/what-is-a-modular-monolith) architecture and is designed to compliment the awesome [tach](https://github.com/gauge-sh/tach) package.

Zapp is:

- 🌎 Open source
- 🐍 Installable via pip
- 🦀 Implemented in rust

#  Why?

I love Python as a language; however, after working across several large enterprise code bases in several languages (C++ 💻, C# ⚙️, TypeScript 🌐, Rust 🦀, Python 🐍, ...) I have found pythons flexibility, simplicity and
rich ecosystem is great for rapid prototyping and development but recurringly introduces challenges in enforcing clean module boundaries and interfaces, especially as projects grow.

Python has no native interface enforcement between whether objects are public, protected, private and whether
the object is exported in a modules interface. Instead, all attributes and methods are public by default - there's only a naming convention (_single_underscore) to signal "this is internal", but nothing stops anyone from using it.
As a python project grows you often end up with deep directory structures (`package.submodule.subsubmodule`) and I commonly see this lead to:

- ♻️ Circular dependencies from sibling modules importing each other
- 🧩 Lots of internal utility functions that aren’t clearly distinguished from the public API.

Without clear boundaries developers often start importing from anywhere:

```python
from package.deeply.nested.internal_module import kinda_private_function
```

Now you're coupled to internals — and changes in one submodule ripple across the codebase; making things harder to maintain.

## 👷 So what can we do?

First of all check out [tach](https://github.com/gauge-sh/tach) - it adds a lot of features to help tackle the aforementioned issues.

So where does ⚡️ Zapp 🐍 fit in?

In python you can use `__all__ = [...]` and `__init__.py` to signal the public api and then you can use tach to enforce that consumers of your library
within a monolithic code base this public api. However, for a deeply nested lib it can still be arduous to manually keep in sync these at the top level with the submodules in the lib itself.

Zapp makes this process more ergonomic by recursively iterating through the path to a provided python directory and automatically
populating the init files based on the `__all__ = [...]` in each individual python file. If the file does not contain this list
then the public api will be implicitly determined based on the assumption that object prefixed with a underscore are private.

# How To Use

Install via pip:

```console
pip install python-zapp
```

Run directly from your python environment:

```console
zapp my_python_package
```

And then package will be recursed and the init files will be populated with the public api.

# Credits

This software uses the following open source packages:

- [Maturin](https://github.com/PyO3/maturin)

# Related

- [tach](https://github.com/gauge-sh/tach)

# License

MIT

---

> [abiemarshall.com](https://www.abiemarshall.com) &nbsp;&middot;&nbsp;
> GitHub [@MarshallEvergreen](https://github.com/MarshallEvergreen)
