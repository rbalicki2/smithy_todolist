const wasm = import("./smithy_site_routing");

wasm.then(module => {
  module.start("app");
});
