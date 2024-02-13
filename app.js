import Jsonnet from "arakoo-jsonnet";
addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
  return new Response("Hello world", { status: 200 });
}
