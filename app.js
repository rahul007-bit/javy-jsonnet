addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  __jsonnet_evaluate_snippet();
  return new Response("Hello world", { status: 200, statusText: "OK" });
}
