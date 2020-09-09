addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  try {
    const { request_handler } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const response = await request_handler()
    return new Response(response, {status: 200})
  } catch (err) {
    // Return the error stack as the response
    return new Response(err.stack || err)
  }
}


async function get_list() {
  
  const value = await TestData.list();

  const keyNames = value.keys.map(e => e.name)

  return ",".concat(keyNames);
}

function get_array() {
  return ["hello", "there"];
}

function get_dict() {
  return { a: "test", b: { c: "hello", d: "world" }, e: "blarg" }
}

async function list_debug(prefix, limit, cursor) {

  console.log("list_debug")
  console.log("prefix", prefix)
  console.log("limit", limit)
  console.log("cursor", cursor)

  return TestData.list(prefix, limit, cursor);
}