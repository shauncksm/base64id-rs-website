<script>
    export let size = 64;

    const SIZES = [64, 32, 16];
</script>

<div>
    <span>Size</span>
    <div class="flex gap-2" role="radiogroup">
        {#each SIZES as thisSize}
            <label>
                <button
                    role="radio"
                    aria-checked={size === thisSize}
                    class="hover:bg-gray-600 active:bg-gray-500 py-2 px-4 {size ===
                    thisSize
                        ? 'bg-gray-500'
                        : 'bg-gray-700'}"
                    on:click={() => (size = thisSize)}>
                    <span>{thisSize} Bit</span>
                </button>
            </label>
        {/each}
    </div>
</div>
