addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  let vars = "";
  let code = "./test.jsonnet";

  let result = __jsonnet_evaluate_snippet(vars, code);
  return new Response(
    result,

    { status: 200, statusText: "OK" }
  );
}
