<script>
    export let signed = true;

    const SIGNS = [
        { signed: true, label: "Signed" },
        { signed: false, label: "Unsigned" },
    ];
</script>

<div>
    <span>Sign</span>
    <div class="flex gap-2" role="radiogroup">
        {#each SIGNS as sign}
            <label>
                <button
                    role="radio"
                    aria-checked={signed === sign.signed}
                    class="hover:bg-gray-600 active:bg-gray-500 py-2 px-4 {signed ===
                    sign.signed
                        ? 'bg-gray-500'
                        : 'bg-gray-700'}"
                    on:click={() => (signed = sign.signed)}>
                    <span>{sign.label}</span>
                </button>
            </label>
        {/each}
    </div>
</div>
