<script context="module">
    import * as wasm from "../../dist-wasm/base64id_rs_website";
    await wasm;
</script>

<script>
    import { onMount } from "svelte";
    import TextInput from "$lib/input/text.svelte";
    import BitSize from "$lib/input/bit-size.svelte";
    import BitSign from "$lib/input/bit-sign.svelte";
    import Random from "$lib/input/random.svelte";
    import Status from "$lib/status.svelte";

    let int = "";
    let str = "";
    let size = 64;
    let signed = true;
    let error = "";
    let clearError;

    function int_changed() {
        try {
            str = wasm.encode_integer(int, size, signed);
            clearError();
        } catch (e) {
            error = e.toString();
        }
    }

    function str_changed() {
        try {
            int = wasm.decode_string(str, size, signed);
            clearError();
        } catch (e) {
            error = e.toString();
        }
    }

    $: if (int) size_changed(size);
    function size_changed(size) {
        try {
            str = wasm.encode_integer(int, size, signed);
            clearError();
        } catch (e) {
            error = e.toString();
        }
    }

    $: if (str) sign_changed(signed);
    function sign_changed(signed) {
        try {
            int = wasm.decode_string(str, size, signed);
            clearError();
        } catch (e) {
            error = e.toString();
        }
    }

    function generate_random() {
        int = wasm.random_integer(size, signed);
        str = wasm.encode_integer(int, size, signed);
    }

    onMount(function () {
        generate_random();
    });
</script>

<div>
    <Status bind:clear={clearError} bind:msg={error} />
    <div
        class="bg-gray-700 p-4 mb-4 mt-1 grid grid-cols-1 xs:grid-cols-2 gap-4">
        <TextInput bind:value={int} on:keyup={int_changed} label="Integer" />
        <TextInput bind:value={str} on:keyup={str_changed} label="Base64url" />
    </div>

    <div class="flex flex-col gap-2">
        <Random on:click={generate_random} />
        <BitSize bind:size />
        <BitSign bind:signed />
    </div>
</div>
