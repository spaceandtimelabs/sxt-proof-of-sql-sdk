

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
};

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}
/**
 * @param {string} query
 * @param {(TableRefAndCommitment)[]} commitments
 * @returns {ProverQueryAndQueryExpr}
 */
function plan_prover_query_dory(query, commitments) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(query, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(commitments, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.plan_prover_query_dory(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return ProverQueryAndQueryExpr.__wrap(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {any} prover_response_json
 * @param {any} query_expr_json
 * @param {(TableRefAndCommitment)[]} commitments
 * @returns {any}
 */
function verify_prover_response_dory(prover_response_json, query_expr_json, commitments) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayJsValueToWasm0(commitments, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.verify_prover_response_dory(retptr, addHeapObject(prover_response_json), addHeapObject(query_expr_json), ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

const ProverQueryAndQueryExprFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_proverqueryandqueryexpr_free(ptr >>> 0, 1));

class ProverQueryAndQueryExpr {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ProverQueryAndQueryExpr.prototype);
        obj.__wbg_ptr = ptr;
        ProverQueryAndQueryExprFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ProverQueryAndQueryExprFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_proverqueryandqueryexpr_free(ptr, 0);
    }
    /**
     * @returns {any}
     */
    get prover_query_json() {
        const ret = wasm.__wbg_get_proverqueryandqueryexpr_prover_query_json(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @param {any} arg0
     */
    set prover_query_json(arg0) {
        wasm.__wbg_set_proverqueryandqueryexpr_prover_query_json(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
     * @returns {any}
     */
    get query_expr_json() {
        const ret = wasm.__wbg_get_proverqueryandqueryexpr_query_expr_json(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @param {any} arg0
     */
    set query_expr_json(arg0) {
        wasm.__wbg_set_proverqueryandqueryexpr_query_expr_json(this.__wbg_ptr, addHeapObject(arg0));
    }
}

const TableRefAndCommitmentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tablerefandcommitment_free(ptr >>> 0, 1));

class TableRefAndCommitment {

    static __unwrap(jsValue) {
        if (!(jsValue instanceof TableRefAndCommitment)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TableRefAndCommitmentFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tablerefandcommitment_free(ptr, 0);
    }
    /**
     * @param {string} table_ref
     * @param {string} table_commitment_hex
     */
    constructor(table_ref, table_commitment_hex) {
        const ptr0 = passStringToWasm0(table_ref, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(table_commitment_hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.tablerefandcommitment_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret >>> 0;
        TableRefAndCommitmentFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const imports = {
    __wbindgen_placeholder__: {
        __wbindgen_is_undefined: function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbg_tablerefandcommitment_unwrap: function(arg0) {
            const ret = TableRefAndCommitment.__unwrap(takeObject(arg0));
            return ret;
        },
        __wbindgen_string_get: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_parse_51ee5409072379d3: function() { return handleError(function (arg0, arg1) {
            const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        }, arguments) },
        __wbg_stringify_eead5648c09faaf8: function() { return handleError(function (arg0) {
            const ret = JSON.stringify(getObject(arg0));
            return addHeapObject(ret);
        }, arguments) },
        __wbindgen_throw: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
    },

};

const wasmRequest = await Functions.makeHttpRequest({
  url: "https://raw.githubusercontent.com/spaceandtimelabs/sxt-proof-of-sql-sdk/feat/wasm-bindgen/crates/proof-of-sql-sdk-wasm/pkg/sxt_proof_of_sql_sdk_wasm_bg.wasm",
  method: "GET",
  responseType: "arraybuffer",
});


if (wasmRequest.error || wasmRequest.status !== 200) {
  throw new Error("Error retrieving wasm file");
}

// Convert the response data to a Uint8Array for WebAssembly instantiation
const wasmCode = new Uint8Array(wasmRequest.data);

const wasmInstance = (await WebAssembly.instantiate(wasmCode, imports)).instance;
const wasm = wasmInstance.exports;
const __wasm = wasm;


// Ensure the API key is available
if (!secrets.apiKey) {
  throw Error("API Key Not Found");
}

// Construct a payload to fetch an accesstoken to be used for
// api access to the prover. It is a required component of the payload to 
// receive a proof.
// Set the secrets field to an apiKey that you own for your sxt account.

// Execute the API request using Functions.makeHttpRequest
const apiResponse = await Functions.makeHttpRequest({
  url: "https://proxy.api.spaceandtime.dev/auth/apikey",
  method: "POST",
  headers: {
    "apikey": secrets.apiKey,
    "Content-Type": "application/json"
  }
});

// Extract the access token, truncate it to 256 characters, and return it as an encoded string
const accessToken = apiResponse.data.accessToken;
return Functions.encodeString("TODO");
