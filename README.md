1. Build the iexample web component
* cd example-page
* wasm-pack build --target web

2. Build the example app
* trunk serve

3. Copy needed dependencies
* cp -r example-page/pkg/snippets/custom-elements-419327fa2779072f dist/snippets/.
* cp example-page/pkg/example\_page\_bg.wasm dist/snippets/rust-import-wasm-55055b11904e231e/example-page/pkg/.

4. Host the application
* python3 -m http.server -d dist

