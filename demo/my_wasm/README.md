## Step

### Install wasm-pack

https://rustwasm.github.io/wasm-pack/installer/

```sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Create project

```sh
wasm-pack new my_wasm
```

### Build wasm

```sh
wasm-pack build --target web
```


### Add index.html


```html
<html>
  <head>
    <title>my_wasm</title>
  </head>
  <body>
    <div>hello my_wasm</div>
    <script type="module">
      import init, { greet } from "./pkg/my_wasm.js";
      async function main() {
        await init();
        greet();
      }
      main();
    </script>
  </body>
</html>
```

### Start A HTTP Server 

```sh
python -m SimpleHTTPServer
```

### Visit Browser

http://127.0.0.1:8000
