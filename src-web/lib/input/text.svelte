<script>
    export let label;
    export let value;

    let copied;

    function copyToClipboard() {
        if (value) {
            navigator.clipboard.writeText(value);

            copied.classList.add("copied");
            copied.addEventListener(
                "animationend",
                function () {
                    copied.classList.remove("copied");
                },
                { once: true }
            );
        }
    }
</script>

<label>
    <span class="block">{label}</span>
    <div class="flex rounded input-outline">
        <div class="w-full relative">
            <div
                bind:this={copied}
                class="h-full w-full absolute flex items-center justify-end invisible opacity-0">
                <span class="mr-2 text-base">Copied</span>
            </div>
            <input
                type="text"
                class="py-2 pl-4 pr-2 w-full bg-gray-800 font-mono rounded-l h-16 outline-none"
                maxlength="25"
                bind:value
                on:keyup />
        </div>

        <button
            on:click={copyToClipboard}
            aria-label="Copy {label} Value"
            class="bg-gray-800 hover:bg-gray-600 active:bg-gray-500 py-2 px-3 rounded-r border-l-2 border-gray-700 text-gray-100 outline-none">
            <!-- This svg is from Feather Icons (https://feathericons.com). Copyright (c) 2013-2017 Cole Bemis -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.75"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="feather feather-copy">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                <path
                    d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
            </svg>
        </button>
    </div>
</label>

<div class="copied" />

<style lang="postcss">
    .input-outline:focus-within {
        @apply outline outline-2 outline-offset-2 outline-gray-400;
    }

    .copied {
        animation: copied;
        animation-duration: 500ms;
        animation-timing-function: ease-in;
    }

    @keyframes copied {
        0% {
            opacity: 1;
            visibility: visible;
        }
        25% {
            opacity: 1;
            visibility: visible;
        }
        100% {
            opacity: 0;
            visibility: hidden;
        }
    }
</style>
