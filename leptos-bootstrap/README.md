# leptos-bootstrap

This library provides bootstrap styled components for your leptos webapps. Demos of the components can be found [here](https://jamesgopsill.github.io/leptos-bootstrap/).

# Support

Please consider supporting the crate by:

- Downloading and using the crate.
- Raising issues, feature requests and improvements.
- Recommending the crate to others.
- ⭐ the crate on GitHub.
- Sponsoring the [maintainer](https://github.com/sponsors/jamesgopsill).

# Usage

To use, please add the bootstrap css, js and icon set cdn links to your index html. E.g.,

```html
<link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.7/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-LN+7fdVzj6u52u30Kp6M/trliBMCMKTyK833zpbD+pXdCLuTusPj697FH4R/5mcr" crossorigin="anonymous">
```

```html
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.7/dist/js/bootstrap.bundle.min.js" integrity="sha384-ndDqU0Gzau9qJ1lfW4pNLlhNTkCfHzAVBReH9diLvGRem5+R9g2FzA8ZGN954O5Q" crossorigin="anonymous"></script>
```

And for icons, add:

```html
<link
  rel="stylesheet"
  href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.13.1/font/bootstrap-icons.min.css"
/>
```


Your `index.html` should then look something like this:

```html
<!doctype html>
<html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>Your Amazing Webapp</title>
        <link
            rel="stylesheet"
            href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.13.1/font/bootstrap-icons.min.css"
        />
        <link
            href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.7/dist/css/bootstrap.min.css"
            rel="stylesheet"
            integrity="sha384-LN+7fdVzj6u52u30Kp6M/trliBMCMKTyK833zpbD+pXdCLuTusPj697FH4R/5mcr"
            crossorigin="anonymous"
        />
        <script
            src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.7/dist/js/bootstrap.bundle.min.js"
            integrity="sha384-ndDqU0Gzau9qJ1lfW4pNLlhNTkCfHzAVBReH9diLvGRem5+R9g2FzA8ZGN954O5Q"
            crossorigin="anonymous"
        ></script>
    </head>
    <body></body>
</html>
```

# Acknowledgements

- GeminiAI for doing the heavy lifting on creating the Icon enum.

# Useful Links

- [Leptos](https://www.leptos.dev/)
- [Trunk](https://trunkrs.dev/)
- [Rust Webassembly](https://www.rust-lang.org/what/wasm)
